mod api;
pub use api::{IsRemoteProcedureCall, IntoPublic};

use uuid::Uuid;

mod operating_system_integration;
pub use operating_system_integration::*;

// mod with_pointers;

pub mod utility;
pub use utility::GenericError;

pub mod database;
pub use database::Database;


pub mod chronic;
pub use chronic::*;

pub mod common;
pub use common::{
  Password,
  password,
};

pub mod features;
pub use features::*;

pub mod user;
pub use user::User;

pub mod state;
pub use state::AppState;

pub mod daemon;
pub use daemon::*;

// pub mod synchronize_source;
// pub use synchronize_source::*;


// pub mod web_client;

// pub fn main() {
//   // _ = AppMutex::open_from_command_line_arguments_then_run();
// }

// fn main() {}



pub type Tried<A, B> = Result<A, B>;
