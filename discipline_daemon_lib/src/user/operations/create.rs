use super::{
  Uuid, OperatingSystemPassword, OperatingSystemUserId,
  OperatingSystemUsername, User, UserName, Serialize, Deserialize,
  IsRemoteProcedureCall, Daemon, db, user_screen_access_regulation,
  UserPublicRepr, IntoPublic, 
}; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Option<Uuid>,
  user_name: UserName,
  operating_system_user_name: OperatingSystemUsername,
  operating_system_user_password: OperatingSystemPassword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  UserIdIsUsedByAnotherUser,
  OperatingSystemUserWithGivenIdIsAlreadyManaged,
  OperatingSystemUserWithGivenNameIsAlreadyManaged,
  Success(UserPublicRepr),
  InternalError,
}

impl IsRemoteProcedureCall for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    if let Some(user_id) = &self.user_id {
      if daemon.state.users.iter().any(|user| user.id == *user_id) {
        return Outcome::UserIdIsUsedByAnotherUser;
      }
    }

    if daemon.state.users.iter()
      .any(|user| user.operating_system_user_name == self.operating_system_user_name)
    {
      return Outcome::OperatingSystemUserWithGivenIdIsAlreadyManaged;
    }

    let operating_system_user_id = match OperatingSystemUserId::from_username(&self.operating_system_user_name) {
      Ok(value) => {
        value
      }
      Err(error) => {
        daemon.log_internal_error(error);
        return Outcome::InternalError;
      }
    };


    if daemon.state.users.iter()
      .any(|user| user.operating_system_user_id == operating_system_user_id)
    {
      return Outcome::OperatingSystemUserWithGivenIdIsAlreadyManaged;
    }

    let user = User {
      id: self.user_id.unwrap_or_else(Uuid::new_v4),
      name: self.user_name,
      operating_system_user_id: operating_system_user_id,
      operating_system_user_name: self.operating_system_user_name,
      operating_system_user_password: self.operating_system_user_password,
      screen_access_regulation: user_screen_access_regulation::Regulation::new(Vec::new()),
    };

    if let Err(error) = db::add_user(&daemon.database, &user) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    daemon.state.users.push(user.clone());
    Outcome::Success(user.into_public())
  }
}