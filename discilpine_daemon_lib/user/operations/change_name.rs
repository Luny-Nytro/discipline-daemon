use super::{
  Daemon, IsOperation, Serialize, Deserialize, Uuid, UserName,
  GenericError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeName {
  user_id: Uuid,
  new_user_name: UserName
}

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  Success,
}

impl IsOperation for ChangeName {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Result<Outcome, GenericError> {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Ok(Outcome::NoSuchUser);
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
      return Err(
        error
          .change_context("changing a user name")
          .add_attachment("user id", self.user_id.to_string())
          .add_attachment("user current name", user.name.as_ref())
      );
    }

    user.name = self.new_user_name;
    Ok(Outcome::Success)
  }
}