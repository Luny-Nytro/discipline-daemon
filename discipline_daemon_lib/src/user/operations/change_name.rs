use super::{
  Daemon, IsOperation, Serialize, Deserialize, Uuid, UserName,
  InternalOperationOutcome,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  new_user_name: UserName
}

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  Success,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser);
    };

    if let Err(error) = daemon
      .database_specification
      .user_module()
      .change_user_name(
        &daemon.database_connection, 
        &self.user_id, 
        &self.new_user_name
      )
    {
      return InternalOperationOutcome::internal_error(error);
    }

    user.name = self.new_user_name;
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}