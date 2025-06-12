use super::{
  Uuid, Serialize, Deserialize, GenericError, IsOperation, 
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

  fn execute(self, daemon: &mut Daemon) -> Result<Outcome, GenericError> {
    if daemon.state.users.iter().all(|user| user.id != self.user_id) {
      return Ok(Outcome::NoSuchUser { user_id: self.user_id });
    }

    if let Err(error) = daemon
      .state_database_specification
      .user_specification
      .delete_user(&daemon.database_connection, &self.user_id)
    {
      return Err(error);
    }

    daemon.state.delete_user_by_id(&self.user_id);
    Ok(Outcome::Success)
  }
}