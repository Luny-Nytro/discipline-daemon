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

    let mut modifications_draft = daemon
      .state_database_specification
      .user_specification
      .create_modifications_draft();

    if let Err(error) = daemon
      .state_database_specification
      .user_specification
      .update_name(&mut modifications_draft, &self.new_user_name)
    {
      return InternalOperationOutcome::internal_error(
        error
          .change_context("changing a user name")
          .add_attachment("user id", self.user_id.to_string())
          .add_attachment("user current name", user.name.as_ref())
      );
    }

    user.name = self.new_user_name;
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}