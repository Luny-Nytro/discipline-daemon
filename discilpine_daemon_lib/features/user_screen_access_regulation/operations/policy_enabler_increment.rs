use super::{
  Serialize, Deserialize, Daemon, Duration, IsOperation, Uuid,
  GenericError
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  WouldBeEffectiveForTooLong,
  Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  policy_id: Uuid,
  user_id: Uuid,
  increment: Duration,
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

    let Some(policy) = user
      .screen_access_regulator
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Ok(Outcome::NoSuchPolicy);
    };

    let Some(new_remaining_duration) = policy
      .enabler
      .timer
      .remaining_duration()
      .checked_add(&self.increment) else 
    {
      return Ok(Outcome::WouldBeEffectiveForTooLong);
    };

    if new_remaining_duration.total_weeks() > 3 {
      return Ok(Outcome::WouldBeEffectiveForTooLong);
    }

    let mut modifications_draft = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .create_modifications_draft();

    daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .enabler_field_specification
      .timer()
      .update_remaining_duration(&mut modifications_draft, &new_remaining_duration)?;

    daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .apply_modifications_draft(
        &daemon.database_connection, 
        &modifications_draft, 
        &self.user_id,
        &self.policy_id, 
      )?;

    policy
      .enabler
      .timer
      .change_remaining_duration(self.increment);

    Ok(Outcome::Success)
  }
}
