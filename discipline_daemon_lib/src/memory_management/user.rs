use super::*;
use crate::GenericError;

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
  // fn operating_system_user_id(&self) -> &OperatingSystemUserId;
  // fn operating_system_user_name(&self) -> &OperatingSystemUsername;
  // fn operating_system_user_password(&self) -> &OperatingSystemPassword;
  fn screen_access_regulation(&self) -> &impl user_screen_access_regulation::RegulationHandle;
}

pub trait IsMutableUser: IsUser {
  fn change_name(&mut self, new_name: UserName);
}

pub struct User {
  pub id: u64,
  pub name: UserName,
  // pub operating_system_user_id: OperatingSystemUserId,
  // pub operating_system_user_name: OperatingSystemUsername,
  // pub operating_system_user_password: OperatingSystemPassword,
  pub user_screen_access_regulation_source: user_screen_access_regulation::Regulation,
}

#[derive(Debug, Clone, Copy)]
pub struct UserPointer(*mut User);

impl UserPointer {
  pub fn user_screen_access_regulation_source(&self) -> &user_screen_access_regulation::Regulation {
    todo!()
  }
}

impl IsUser for UserPointer {
  fn id(&self) -> u64 {
    unsafe {
      (*self.0).id
    }
  }

  fn name(&self) -> &UserName {
    unsafe {
      &(*self.0).name
    }
  }

  // fn operating_system_user_id(&self) -> &OperatingSystemUserId {
  //   &self.operating_system_user_id
  // }

  // fn operating_system_user_name(&self) -> &OperatingSystemUsername {
  //   &self.operating_system_user_name
  // }

  // fn operating_system_user_password(&self) -> &OperatingSystemPassword {
  //   &self.operating_system_user_password
  // }

  fn screen_access_regulation(&self) -> &impl user_screen_access_regulation::RegulationHandle {
    unsafe {
      &(*self.0).user_screen_access_regulation_source
    }
  }
}

// impl IsMutableUser for UserPointer {
//   fn change_name(&self, new_name: UserName) {
//     // self.name = new_name;
//   }
// }

pub fn find_user(state: &State, user_id: u64) -> Option<UserPointer> {
  state.users.get(&user_id).map(|user| *user)
}