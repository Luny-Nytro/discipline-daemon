use super::{
  Serialize, Deserialize, Uuid, Daemon,
  GenericError, DateTime, IsOperation,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_id: Uuid,
}

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  MayNotDeletePolicyWhileEnabled,
  Success,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Result<Outcome, GenericError> {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Ok(Outcome::NoSuchUser);
    };

    let regulator = &mut user.screen_access_regulator;

    let Some(policy) = regulator.find_policy_by_id_mut(&self.policy_id) else {
      return Ok(Outcome::NoSuchPolicy);
    };

    let now = DateTime::now();
    if policy.is_enabled(now) {
      return Ok(Outcome::MayNotDeletePolicyWhileEnabled);
    }

    daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .delete_policy(
        &daemon.database_connection, 
        &self.user_id,
        &self.policy_id, 
      )?;

    regulator.remove_policy_by_id(&self.policy_id);
    Ok(Outcome::Success)
  }
}