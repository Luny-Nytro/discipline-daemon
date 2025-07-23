use crate::{user_screen_access_regulation, DateTime, OperationScheduler, TimeTracker, User, Uuid};

#[derive(Debug)]
pub struct AppState {
  // pub time_tracker: TimeTracker,
  pub users: Vec<User>,
  pub user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo,
  // pub shadow_vaults: shadow_vaults::Feature,
  // pub networking_access: networking_access::Feature,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      // TODO: Should we recieve the current as argument?
      // time_tracker: TimeTracker::new(DateTime::now()),
      users: Vec::new(),
      user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo::default(),
    }
  }
}

impl AppState {
  pub fn operating_system_integration(&self) -> OperationScheduler {
    todo!()
  }
  pub fn find_user_index(&self, user_id: &Uuid) -> Option<usize> {
    self.users.iter().position(|user| user.id == *user_id)
  }
  pub fn find_user_by_index_or_panic(&self, user_index: usize) -> &User {
    &self.users[user_index]
  }
  pub fn find_user_by_id(&self, user_id: &Uuid) -> Option<&User> {
    self.users.iter().find(|user| user.id == *user_id)
  }

  pub fn find_user_by_id_mut(&mut self, user_id: &Uuid) -> Option<&mut User> {
    self.users.iter_mut().find(|user| user.id == *user_id)
  }

  pub fn delete_user_by_id(&mut self, user_id: &Uuid) {
    if let Some(position) = self
      .users
      .iter()
      .position(|user| user.id == *user_id)
    {
      self.users.remove(position);
    }
  }
}