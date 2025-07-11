use serde::{Serialize, Deserialize};
use crate::{
  user_screen_access_regulation, DaemonMutex, GenericError, OperatingSystemPassword, 
  OperatingSystemUserId, OperatingSystemUsername, Uuid
};

#[derive(Debug, Clone)]
pub struct UserName(String);

impl UserName {
  pub const MIN_LENGTH: usize = 3;
  pub const MAX_LENGTH: usize = 15;

  pub fn new(name: String) -> Result<Self, GenericError> {
    if name.len() < Self::MIN_LENGTH 
    || name.len() > Self::MAX_LENGTH
    {
      return Err(
        GenericError::new("creating UserName")
        .add_error("user name length is outside valid length")
        .add_attachment("name", name)
        .add_attachment("minimum length", Self::MIN_LENGTH.to_string())
        .add_attachment("maximum length", Self::MAX_LENGTH.to_string())
      );
    }

    Ok(Self(name))
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}

#[derive(Debug, Clone)]
pub struct User {
  pub id: Uuid,
  pub name: UserName,
  pub operating_system_user_id: OperatingSystemUserId,
  pub operating_system_user_name: OperatingSystemUsername,
  pub operating_system_user_password: OperatingSystemPassword,
  pub screen_access_regulation: user_screen_access_regulation::Regulator,
}

impl User {
  pub fn pack(
    id: Uuid,
    name: UserName,
    operating_system_user_id: OperatingSystemUserId,
    operating_system_user_name: OperatingSystemUsername,
    operating_system_user_password: OperatingSystemPassword,
    screen_access_regulation: user_screen_access_regulation::Regulator,
  ) -> Self 
  {
    Self {
      id, 
      name,
      operating_system_user_id,
      operating_system_user_name,
      operating_system_user_password,
      screen_access_regulation,
    }
  }

  pub fn id(&self) -> &Uuid {
    &self.id
  }
  pub fn name(&self) -> &UserName {
    &self.name
  }
  pub fn operating_system_user_id(&self) -> &OperatingSystemUserId {
    &self.operating_system_user_id
  }
  pub fn operating_system_user_name(&self) -> &OperatingSystemUsername {
    &self.operating_system_user_name
  }
  pub fn operating_system_user_password(&self) -> &OperatingSystemPassword {
    &self.operating_system_user_password
  }
  pub fn screen_access_regulator(&self) -> &user_screen_access_regulation::Regulator {
    &self.screen_access_regulation
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeUserNameArgument {
  user_id: Uuid,
  new_user_name: UserName
}

#[derive(Debug, Clone)]
pub enum ChangeUserNameOutcome {
  NoSuchUser,
  Success,
  InternalError,
}

pub fn execute(
  daemon: DaemonMutex, 
  argument: ChangeUserNameArgument
) 
  -> ChangeUserNameOutcome
{
  let mut daemon = daemon.lock().unwrap();
  let Some(user) = daemon.state.users.iter().find(predicate) else {
    return ChangeUserNameOutcome::NoSuchUser;
  };

  let mut draft = daemon.database.create_user_update_draft();
  draft.update_name(&argument.new_user_name);
  
  if let Err(error) = draft.commit(&argument.user_id) {
    // daemon.internal_logger().log_error()
    return ChangeUserNameOutcome::InternalError;
  }

  user.name = argument.new_user_name;
  ChangeUserNameOutcome::Success
}