use super::{
  Serialize, Deserialize, Daemon, Duration, IsOperation, Uuid,
  InternalOperationOutcome
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

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser);
    };

    let Some(policy) = user
      .screen_access_regulation
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchPolicy);
    };

    let Some(new_remaining_duration) = policy
      .enabler
      .timer
      .remaining_duration()
      .checked_add(&self.increment) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::WouldBeEffectiveForTooLong);
    };

    if new_remaining_duration.total_weeks() > 3 {
      return InternalOperationOutcome::public_outcome(Outcome::WouldBeEffectiveForTooLong);
    }

    if let Err(error) = daemon
      .database_specification
      .user_screen_access_regulator()
      .change_policy_enabled_duration(
        &daemon.database_connection, 
        &self.user_id,
        &self.policy_id, 
        &self.increment,
      )
    {
      return InternalOperationOutcome::internal_error(error);
    }

    policy.enabler.timer.change_remaining_duration(self.increment);
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}
