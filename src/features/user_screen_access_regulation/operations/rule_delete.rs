use super::{
  Serialize, Deserialize, Uuid, GenericError, IsOperation,
  Daemon, DateTime,
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  NoSuchRule,
  MayNotDeleteRuleWhilePolicyEnabled,
  InternalError(GenericError),
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

  fn execute(self, daemon: &mut Daemon) -> Self::Outcome {
    let Some(user) = daemon
      .state
      .get_user_by_id_mut(&self.user_id) else
    {
      return Outcome::NoSuchUser;
    };

    let regulator = &mut user
      .screen_access_regulator;

    let Some(policy) = regulator
      .get_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::NoSuchPolicy;
    };

    if policy.has_rule_with_id(&self.rule_id) {
      return Outcome::NoSuchRule;
    }

    let now = DateTime::now();
    if policy.is_enabled(now) {
      return Outcome::MayNotDeleteRuleWhilePolicyEnabled;
    }

    if let Err(error) = daemon
      .schema
      .user_screen_access_regulation
      .policy
      .delete_policy(
        &daemon.database_connection, 
        &self.policy_id, 
        &self.user_id
      )
    {
      return Outcome::InternalError(error.change_context("delete rule"));
    }

    policy.remove_rule_by_id(&self.rule_id);
    Outcome::Success
  }
}