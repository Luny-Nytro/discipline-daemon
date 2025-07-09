use crate::{
  user_screen_access_regulation, GenericError, OperatingSystemPassword, 
  OperatingSystemUserId, OperatingSystemUsername, Uuid,
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
  pub operating_system_username: OperatingSystemUsername,
  pub operating_system_password: OperatingSystemPassword,
  pub screen_access_regulator: user_screen_access_regulation::Regulator,
}

impl User {
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
    &self.operating_system_username
  }
  pub fn operating_system_user_password(&self) -> &OperatingSystemPassword {
    &self.operating_system_password
  }
  pub fn screen_access_regulator(&self) -> &user_screen_access_regulation::Regulator {
    &self.screen_access_regulator
  }
}