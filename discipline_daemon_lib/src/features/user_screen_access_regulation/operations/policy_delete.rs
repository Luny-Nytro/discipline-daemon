use super::{
  Serialize, Deserialize, Uuid, Daemon,
  DateTime, IsOperation, InternalOperationOutcome,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicy {
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

impl IsOperation for DeletePolicy {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser);
    };

    let regulator = &mut user.screen_access_regulator;

    let Some(policy) = regulator.find_policy_by_id_mut(&self.policy_id) else {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchPolicy);
    };

    let now = DateTime::now();
    if policy.is_enabled(now) {
      return InternalOperationOutcome::public_outcome(Outcome::MayNotDeletePolicyWhileEnabled);
    }

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .delete_policy(
        &daemon.database_connection, 
        &self.user_id,
        &self.policy_id, 
      ) 
    {
      return InternalOperationOutcome::internal_error(error);
    }

    regulator.remove_policy_by_id(&self.policy_id);
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}