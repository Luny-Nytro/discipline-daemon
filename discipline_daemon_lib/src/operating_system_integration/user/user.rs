use serde::{Serialize, Deserialize};
use crate::{
  user_screen_access_regulation, DaemonMutex, GenericError, OperatingSystemUserPassword, 
  OperatingSystemUserId, OperatingSystemUserName, Uuid
};

#[derive(Debug, Clone, PartialEq, Eq)]
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
  pub operating_system_user_name: OperatingSystemUserName,
  pub operating_system_user_password: OperatingSystemUserPassword,
  pub screen_access_regulation: user_screen_access_regulation::Regulation,
}

impl User {
  pub fn from_fields(
    id: Uuid,
    name: UserName,
    operating_system_user_id: OperatingSystemUserId,
    operating_system_user_name: OperatingSystemUserName,
    operating_system_user_password: OperatingSystemUserPassword,
    screen_access_regulation: user_screen_access_regulation::Regulation,
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
  pub fn operating_system_user_name(&self) -> &OperatingSystemUserName {
    &self.operating_system_user_name
  }
  pub fn operating_system_user_password(&self) -> &OperatingSystemUserPassword {
    &self.operating_system_user_password
  }
  pub fn screen_access_regulator(&self) -> &user_screen_access_regulation::Regulation {
    &self.screen_access_regulation
  }
}
