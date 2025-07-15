use super::{
  Serialize, Deserialize, Uuid, Daemon,
  DateTime, IsRemoteProcedureCall, policy_db,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  MayNotDeletePolicyWhileEnabled,
  Success,
  InternalError,
}

impl IsRemoteProcedureCall for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    

    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    let regulation = &mut user.screen_access_regulation;

    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return Outcome::NoSuchPolicy;
    };

    // let now = DateTime::now();
    if policy.is_enabled() {
      return Outcome::MayNotDeletePolicyWhileEnabled;
    }

    if let Err(error) = policy_db::delete_policy(
      &daemon.database, 
      &self.policy_id,
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    regulation.remove_policy_by_id(&self.policy_id);
    Outcome::Success
  }
}