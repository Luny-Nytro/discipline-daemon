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

  fn execute(self, daemon: &mut Daemon) -> Result<Outcome, GenericError> {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else
    {
      return Ok(Outcome::NoSuchUser);
    };

    let regulator = &mut user
      .screen_access_regulator;

    let Some(policy) = regulator
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Ok(Outcome::NoSuchPolicy);
    };

    if policy.there_is_rule_with_id(&self.rule_id) {
      return Ok(Outcome::NoSuchRule);
    }

    let now = DateTime::now();
    if policy.is_enabled(now) {
      return Ok(Outcome::MayNotDeleteRuleWhilePolicyEnabled);
    }

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .rule
      .delete_rule(
        &daemon.database_connection, 
        &self.policy_id, 
        &self.user_id,
        &self.rule_id,
      )
    {
      return Err(error.change_context("delete rule"));
    }

    policy.remove_rule_by_id(&self.rule_id);
    Ok(Outcome::Success)
  }
}