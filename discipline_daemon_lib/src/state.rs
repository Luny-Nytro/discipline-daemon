use crate::{user_screen_access_regulation, User, Uuid};

#[derive(Debug)]
pub struct AppState {
  pub users: Vec<User>,
  pub user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo,
  // pub shadow_vaults: shadow_vaults::Feature,
  // pub networking_access: networking_access::Feature,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      users: Vec::new(),
      user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo::default(),
    }
  }
}

impl AppState {
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