use super::{
  Uuid, Serialize, Deserialize, InternalOperationOutcome, IsOperation, 
  Daemon,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  NoSuchUser { user_id: Uuid },
  UserHasEnabledPolicies,
  Success,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    if daemon.state.users.iter().all(|user| user.id != self.user_id) {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser { user_id: self.user_id });
    }

    if let Err(error) = daemon
      .database_specification
      .user_module()
      .delete_user(&daemon.database_connection, &self.user_id)
    {
      return InternalOperationOutcome::internal_error(error);
    }

    daemon.state.delete_user_by_id(&self.user_id);
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}