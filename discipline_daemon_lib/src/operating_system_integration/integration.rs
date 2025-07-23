use std::{collections::HashMap, sync::{Arc, Mutex}};
use super::*;

pub struct UserInfo {
  pub user_id: OperatingSystemUserId,
  pub user_name: OperatingSystemUserName,
  pub user_password: OperatingSystemUserPassword,
  pub user_screen_access_regulation: crate::screen_access_regulation::Regulation,
  pub user_screen_access_regulation_application: UserScreenAccessRegulationApplicationData,
}

pub struct IntegrationData {
  pub users: HashMap<OperatingSystemUserId, UserInfo>,
  pub screen_access_regulation_application_common_info: CommonScreenAccessRegulationApplicationData,
}

impl IntegrationData {
  pub fn initial() -> Self {
    Self {
      users: HashMap::new(),
      screen_access_regulation_application_common_info: CommonScreenAccessRegulationApplicationData::new(),
    }
  }
}

impl IntegrationData {
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

// pub enum ApiOperation {
//   AddUserGivenId { id: OperatingSystemUserId },
//   AddUserGivenName { name: OperatingSystemUserName },
//   DeleteUserGivenId { id: OperatingSystemUserId },
//   DeleteUserGivenName { id: OperatingSystemUserName },
// }

// pub(super) fn api_task(
//   integration: &mut IntegrationData,
//   operations: &mut Vec<ApiOperation>
// ) {

//     // let operations = take(&mut *operations.lock().unwrap());
//     // for operation in operations {
//     //   match operation {
//     //     ApiOperation::AddUser(id, name, password, interval) => {
//     //       integration.operating_system_users.push(OperatingSystemUser {
//     //         screen_access_status: UserScreenAccessStatus::LoginAllowed,
//     //         operating_system_user_id: id,
//     //         operating_system_user_name: name,
//     //         operating_system_user_password: password,
//     //         screen_access_regulation_enforcing_interval: interval,
//     //       });
//     //     }
//     //     ApiOperation::DeleteUser(id) => {

//     //     }
//     //   }
// }



