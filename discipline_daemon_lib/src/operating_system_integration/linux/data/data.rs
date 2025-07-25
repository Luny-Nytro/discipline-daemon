use std::collections::HashMap;
use super::*;

pub struct UserInfo {
  pub user_id: OperatingSystemUserId,
  pub user_name: OperatingSystemUserName,
  pub user_password: OperatingSystemUserPassword,
  pub user_screen_access_regulation: crate::screen_access_regulation::Regulation,
  pub user_screen_access_regulation_application: screen_access_regulation_application::UserScreenAccessRegulationApplicationData,
}

pub struct Data {
  pub users: HashMap<OperatingSystemUserId, UserInfo>,
  pub screen_access_regulation_application_common_info: screen_access_regulation_application::CommonScreenAccessRegulationApplicationData,
}

impl Data {
  pub fn initial() -> Self {
    Self {
      users: HashMap::new(),
      screen_access_regulation_application_common_info: screen_access_regulation_application::CommonScreenAccessRegulationApplicationData::new(),
    }
  }
}

impl Data {
  pub fn is_user_managed_given_id(&self, user_id: OperatingSystemUserId) -> bool {
    todo!()
  }
  pub fn is_user_managed_given_name(&self, user_name: &OperatingSystemUserName) -> bool {
    todo!()
  }
  pub fn is_user_managed(&self, user_identification_method: &UserIdentificationMethod) -> bool {
    todo!()
  }
  // pub fn find_user_index(&self, user_id: &Uuid) -> Option<usize> {
  //   self.users.iter().position(|user| user.id == *user_id)
  // }
  // pub fn find_user_by_index_or_panic(&self, user_index: usize) -> &User {
  //   &self.users[user_index]
  // }
  // pub fn find_user_by_id(&self, user_id: &Uuid) -> Option<&User> {
  //   self.users.iter().find(|user| user.id == *user_id)
  // }

  // pub fn find_user_by_id_mut(&mut self, user_id: &Uuid) -> Option<&mut User> {
  //   self.users.iter_mut().find(|user| user.id == *user_id)
  // }

  // pub fn delete_user_by_id(&mut self, user_id: &Uuid) {
  //   if let Some(position) = self
  //     .users
  //     .iter()
  //     .position(|user| user.id == *user_id)
  //   {
  //     self.users.remove(position);
  //   }
  // }
}