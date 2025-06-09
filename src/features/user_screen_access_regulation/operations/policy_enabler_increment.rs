use super::{
  Serialize, Deserialize, Daemon, Duration, IsOperation, Uuid,
  GenericError
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  WouldBeEffectiveForTooLong,
  InternalError(GenericError),
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

  fn execute(self, daemon: &mut Daemon) -> Self::Outcome {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::NoSuchUser;
    };

    let Some(policy) = user
      .screen_access_regulator
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::NoSuchPolicy;
    };

    let Some(new_remaining_duration) = policy
      .enabler
      .timer
      .remaining_duration()
      .checked_add(&self.increment) else 
    {
      return Outcome::WouldBeEffectiveForTooLong;
    };

    if new_remaining_duration.total_weeks() > 3 {
      return Outcome::WouldBeEffectiveForTooLong;
    }

    let mut modifications_draft = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .create_modifications_draft();

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .enabler_field_specification
      .timer()
      .update_remaining_duration(&mut modifications_draft, &new_remaining_duration)
    {
      return Outcome::InternalError(error);
    }

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .apply_modifications_draft(
        &daemon.database_connection, 
        &modifications_draft, 
        &self.policy_id, 
        &self.user_id,
      )
    {
      return Outcome::InternalError(error);
    }

    policy
      .enabler
      .timer
      .set_remaining_duration(self.increment);

    Outcome::Success
  }
}
