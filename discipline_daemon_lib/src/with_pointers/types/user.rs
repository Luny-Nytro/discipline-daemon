use crate::GenericError;
use crate::common::*;
use super::user_screen_access_regulation;

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

pub trait IsUser {
  fn id(&self) -> u64;
  fn name(&self) -> &UserName;
  fn operating_system_user_id(&self) -> &OperatingSystemUserId;
  fn operating_system_user_name(&self) -> &OperatingSystemUsername;
  fn operating_system_user_password(&self) -> &OperatingSystemPassword;
  fn screen_access_regulation(&self) -> &impl user_screen_access_regulation::IsRegulator;
}

pub trait IsMutableUser: IsUser {
  fn change_name(&mut self, new_name: UserName);
  fn screen_access_regulation_mut(&mut self) -> &mut impl user_screen_access_regulation::IsMutableRegulator;
}