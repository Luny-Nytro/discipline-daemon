pub mod database;
pub use database::{NormaizedState, StateAdapter};

use crate::user_screen_time_regulation;

#[derive(Debug)]
pub struct State {
  pub user_access: user_screen_time_regulation::CommonInfo,
  // pub shadow_vaults: shadow_vaults::Feature,
  // pub networking_access: networking_access::Feature,
}