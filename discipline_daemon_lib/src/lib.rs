pub mod logic {
  pub use crate::chronic;
  pub use crate::chronic::*;
  pub use crate::features::internet_access_regulation;
  pub use crate::features::screen_access_regulation;
}

pub mod error {
  pub use crate::utility::{GenericError, InternalErrorLogger};
}

mod api;
pub use api::IntoPublic;

use uuid::Uuid;

mod operating_system_integration;
pub use operating_system_integration::{
  User,
  UserId,
  UserName,
  UserPassword,
};

// mod memory_management;

pub mod utility;
pub use utility::{GenericError, InternalErrorLogger};

pub mod database;
pub use database::Database;


pub mod chronic;
pub use chronic::*;

// pub mod common;
// pub use common::{
//   Password,
//   password,
// };

pub mod features;
pub use features::*;

pub mod daemon;
pub use daemon::*;