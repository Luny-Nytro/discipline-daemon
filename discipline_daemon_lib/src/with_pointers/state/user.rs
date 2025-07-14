use crate::with_pointers::types::user::*;
use crate::common::*;
use super::user_screen_access_regulation;

pub struct User {
  pub id: u64,
  pub name: UserName,
  pub operating_system_user_id: OperatingSystemUserId,
  pub operating_system_user_name: OperatingSystemUsername,
  pub operating_system_user_password: OperatingSystemPassword,
  pub user_screen_access_regulation: user_screen_access_regulation::Regulator,
}

impl IsUser for User {
  fn id(&self) -> u64 {
    self.id
  }

  fn name(&self) -> &UserName {
    &self.name
  }

  fn operating_system_user_id(&self) -> &OperatingSystemUserId {
    &self.operating_system_user_id
  }

  fn operating_system_user_name(&self) -> &OperatingSystemUsername {
    &self.operating_system_user_name
  }

  fn operating_system_user_password(&self) -> &OperatingSystemPassword {
    &self.operating_system_user_password
  }

  fn screen_access_regulation(&self) -> &impl user_screen_access_regulation::IsRegulator {
    &self.user_screen_access_regulation
  }
}

impl IsMutableUser for User {
  fn change_name(&mut self, new_name: UserName) {
    self.name = new_name;
  }

  fn screen_access_regulation_mut(&mut self) -> &mut impl user_screen_access_regulation::IsMutableRegulator {
    &mut self.user_screen_access_regulation
  }
}