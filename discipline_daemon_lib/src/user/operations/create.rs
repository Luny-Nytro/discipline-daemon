use super::{
  Uuid, OperatingSystemPassword, OperatingSystemUserId,
  OperatingSystemUsername, User, UserName, Serialize, Deserialize,
  IsOperation, Daemon, GenericError, user_screen_access_regulation,
  UserPublicRepr, IntoPublic
}; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Option<Uuid>,
  user_name: UserName,
  operating_system_user_name: OperatingSystemUsername,
  operating_system_user_password: OperatingSystemPassword,
}

pub enum Outcome {
  UserIdIsUsedByAnotherUser,
  OperatingSystemUserWithGivenIdIsAlreadyManaged,
  OperatingSystemUserWithGivenNameIsAlreadyManaged,
  Success(UserPublicRepr),
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Result<Outcome, GenericError> {
    if let Some(user_id) = &self.user_id {
      if daemon.state.users.iter().any(|user| user.id == *user_id) {
        return Ok(Outcome::UserIdIsUsedByAnotherUser);
      }
    }

    if daemon
      .state
      .users
      .iter()
      .any(|user| user.operating_system_username == self.operating_system_user_name)
    {
      return Ok(Outcome::OperatingSystemUserWithGivenIdIsAlreadyManaged);
    }

    let operating_system_user_id = match OperatingSystemUserId::from_username(&self.operating_system_user_name) {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(error.change_context("creating a user"));
      }
    };


    if daemon
      .state
      .users
      .iter()
      .any(|user| user.operating_system_user_id == operating_system_user_id)
    {
      return Ok(Outcome::OperatingSystemUserWithGivenIdIsAlreadyManaged);
    }

    let user = User {
      id: self.user_id.unwrap_or_else(Uuid::new_v4),
      name: self.user_name,
      operating_system_user_id: operating_system_user_id,
      operating_system_username: self.operating_system_user_name,
      operating_system_password: self.operating_system_user_password,
      screen_access_regulator: user_screen_access_regulation::Regulator::new(Vec::new()),
    };

    if let Err(error) = daemon
      .state_database_specification
      .user_specification
      .add_user(&daemon.database_connection, &user)
    {
      return Err(error.change_context("creating a user"));
    }

    daemon.state.users.push(user.clone());
    Ok(Outcome::Success(user.into_public()))
  }
}