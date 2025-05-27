pub mod database;
pub use database::{NormaizedState, StateSchema};
use uuid::Uuid;

use crate::{user_screen_access_regulation, User};

#[derive(Debug)]
pub struct State {
  pub users: Vec<User>,
  pub user_access: user_screen_access_regulation::CommonInfo,
  // pub shadow_vaults: shadow_vaults::Feature,
  // pub networking_access: networking_access::Feature,
}

impl State {
  pub fn get_user_by_id(&self, user_id: &Uuid) -> Option<&User> {
    self.users.iter().any(|user| user.id == *user_id)
  }

  pub fn get_user_by_id_mut(&mut self, user_id: &Uuid) -> Option<&mut User> {
    self.users.iter_mut().any(|user| user.id == *user_id)
  }
}