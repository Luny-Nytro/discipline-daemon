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
  InternalError(GenericError),
  Success,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon.state.get_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    let regulator = &mut user.screen_access_regulator;

    let Some(policy) = regulator.get_policy_by_id_mut(&self.policy_id) else {
      return Outcome::NoSuchPolicy;
    };

    let now = DateTime::now();
    if policy.is_enabled(now) {
      return Outcome::MayNotDeletePolicyWhileEnabled;
    }

    if let Err(error) = daemon
      .schema
      .user_screen_access_regulation
      .policy
      .delete_policy(&daemon.database_connection, &self.policy_id, &self.user_id)
    {
      return Outcome::InternalError(error);
    }

    regulator.remove_policy_by_id(&self.policy_id);
    Outcome::Success
  }
}