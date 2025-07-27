use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard, PoisonError}};
use crate::database::operating_system_integration_linux_data as db;
use super::*;

#[derive(Debug, Clone)]
pub struct UserInfo {
  pub user_id: UserId,
  pub user_name: UserName,
  pub user_password: UserPassword,
  pub user_screen_access_regulation: crate::screen_access_regulation::Regulation,
  pub user_screen_access_regulation_application: screen_access_regulation_application::UserScreenAccessRegulationApplicationData,
  pub user_internet_access_regulation: crate::internet_access_regulation::Regulation,
  pub user_internet_access_regulation_application: internet_access_regulation_application::UserInternetAccessRegulationApplicationData,
}

pub struct OperatingSystemIntegrationData {
  pub users: HashMap<UserId, UserInfo>,
  pub screen_access_regulation_application_common_info: screen_access_regulation_application::CommonScreenAccessRegulationApplicationData,
}

impl OperatingSystemIntegrationData {
  pub fn initial() -> Self {
    Self {
      users: HashMap::new(),
      screen_access_regulation_application_common_info: screen_access_regulation_application::CommonScreenAccessRegulationApplicationData::new(),
    }
  }
}

impl OperatingSystemIntegrationData {
  pub fn is_user_managed_given_id(&self, user_id: UserId) -> bool {
    todo!()
  }
  pub fn is_user_managed_given_name(&self, user_name: &UserName) -> bool {
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

pub struct OperatingSystemIntegration {
  data: Arc<Mutex<OperatingSystemIntegrationData>>,
  async_operation_scheduler: Arc<OperationScheduler>,
}

impl OperatingSystemIntegration {
  pub fn new(operating_system_integration_data: OperatingSystemIntegrationData) -> Self {
    let data = Arc::new(Mutex::new(operating_system_integration_data));
    
    Self {
      data: Arc::clone(&data),
      async_operation_scheduler: OperationScheduler::new(data),
    }
  }

  pub fn open(database: &Database) -> Result<Self, GenericError> {
    Ok(Self::new(db::retrieve(database)?))
  }

  pub fn lock_data(&self) -> Result<MutexGuard<'_, OperatingSystemIntegrationData>, PoisonError<MutexGuard<'_, OperatingSystemIntegrationData>>> {
    self.data.lock()
  } 
}