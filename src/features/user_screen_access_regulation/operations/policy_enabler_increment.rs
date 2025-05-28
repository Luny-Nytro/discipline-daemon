use super::{
  Serialize, Deserialize, Daemon, Duration, IsOperation, Uuid,
  GenericError
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
      .get_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::NoSuchUser;
    };

    let Some(policy) = user
      .screen_access_regulator
      .get_policy_by_id_mut(&self.policy_id) else 
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

    let mut updater = daemon
      .schema
      .user_screen_access_regulation_policy
      .create_policy_updater(&self.policy_id, &self.user_id);

    daemon
      .schema
      .user_screen_access_regulation_policy
      .enabler
      .timer()
      .set_remaining_duration(&mut updater, &new_remaining_duration);

    if let Err(error) = updater.execute(&daemon.database_connection) {
      return Outcome::InternalError(error);
    }

    policy
      .enabler
      .timer
      .change_remaining_duration(self.increment);

    Outcome::Success
  }
}
