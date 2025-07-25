use crate::{screen_access_regulation::Regulation, Database};

use super::{
  Serialize, Deserialize, Daemon, Duration, IsRemoteProcedureCall, Uuid,
  policy_db
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  WouldBeEffectiveForTooLong,
  Success,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  policy_id: Uuid,
  user_id: Uuid,
  increment: Duration,
}

impl IsRemoteProcedureCall for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::NoSuchUser;
    };

    let Some(policy) = user
      .screen_access_regulation
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::NoSuchPolicy;
    };

    let Some(new_remaining_duration) = policy
      .protector()
      .remaining_duration()
      .checked_add(&self.increment) else 
    {
      return Outcome::WouldBeEffectiveForTooLong;
    };

    if new_remaining_duration.total_weeks() > 3 {
      return Outcome::WouldBeEffectiveForTooLong;
    }

    if let Err(error) = policy_db::update_enabled_duration(
      &daemon.database, 
      &self.policy_id, 
      new_remaining_duration,
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    policy.protector_mut().change_remaining_duration(self.increment);
    Outcome::Success
  }
}
