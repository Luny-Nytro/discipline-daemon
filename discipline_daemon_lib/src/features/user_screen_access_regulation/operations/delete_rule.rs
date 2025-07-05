use super::{
  Serialize, Deserialize, Uuid, IsOperation,
  Daemon, DateTime, InternalOperationOutcome,
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  NoSuchRule,
  MayNotDeleteRuleWhilePolicyEnabled,
  Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_id: Uuid,
  rule_id: Uuid,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else
    {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser);
    };

    let regulator = &mut user.screen_access_regulator;

    let Some(policy) = regulator
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchPolicy);
    };

    if policy.there_is_rule_with_id(&self.rule_id) {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchRule);
    }

    let now = DateTime::now();
    if policy.is_enabled(now) {
      return InternalOperationOutcome::public_outcome(Outcome::MayNotDeleteRuleWhilePolicyEnabled);
    }

    if let Err(error) = daemon
      .database_specification
      .user_screen_access_regulator()
      .delete_rule(
        &daemon.database_connection, 
        &self.user_id,
        &self.policy_id, 
        &self.rule_id,
      )
    {
      return InternalOperationOutcome::internal_error(error.change_context("delete rule"));
    }

    policy.remove_rule_by_id(&self.rule_id);
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}