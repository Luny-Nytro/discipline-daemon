use crate::{user_screen_access_regulation, GenericError, OperatingSystemPassword, OperatingSystemUserId, OperatingSystemUsername, Uuid};

#[derive(Debug, Clone)]
pub struct UserName(String);

impl UserName {
  pub const MIN_LENGTH: usize = 3;
  pub const MAX_LENGTH: usize = 15;

  pub fn new(name: String) -> Result<Self, GenericError> {
    if name.len() < Self::MIN_LENGTH {
      return Err(GenericError::new("create user name")
        .add_error("name is too short")
        .add_attachment("name", name)
        .add_attachment("min length", Self::MIN_LENGTH.to_string())
      );
    }

    if name.len() > Self::MAX_LENGTH {
      return Err(GenericError::new("create user name")
        .add_error("name is too long")
        .add_attachment("name", name)
        .add_attachment("max length", Self::MAX_LENGTH.to_string())
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