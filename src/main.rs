use uuid::Uuid;

pub mod utility;
pub use utility::GenericError;

pub mod database;

pub mod to_public_repr;
pub use to_public_repr::ToPublicRepr;

pub mod operation;
pub use operation::IsOperation;

pub mod chronic_types;
pub use chronic_types::*;

pub mod common_types;
pub use common_types::{
  OperatingSystemPassword,
  OperatingSystemUserId,
  OperatingSystemUsername,
  Password,
  operating_system_password,
  operating_system_user_id,
  operating_system_username,
  password,
};

pub mod features;
pub use features::*;

pub mod user;
pub use user::User;

pub mod state;
pub use state::{State, NormaizedState, Specification};

pub mod daemon;
pub use daemon::*;

// pub mod operation;
// pub use operation::IsOperation;

// pub mod synchronize_source;
// pub use synchronize_source::*;


// pub mod web_client;

pub fn main() {
  // _ = AppMutex::open_from_command_line_arguments_then_run();
}

// fn main() {}
