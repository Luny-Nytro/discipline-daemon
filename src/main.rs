use uuid::Uuid;

// pub mod daemon;
// pub use daemon::*;

// pub mod user;
// pub use user::User;
// 
// pub mod chronic_types;
// pub use chronic_types::*;

// pub mod common_types;
// pub use common_types::{
//   // DailyCountdownTimer,
//   OperatingSystemPassword,
//   OperatingSystemUserId,
//   OperatingSystemUsername,
//   Password,
//   // daily_countdown_timer,
//   // daily_user_access_allowance,
//   operating_system_password,
//   operating_system_user_id,
//   operating_system_username,
//   password,
// };

// pub mod features;
// pub use features::*;

// pub mod operation;
// pub use operation::IsOperation;

// pub mod into_public;
// pub use into_public::ToPublicRepr;

pub mod database;
// pub mod synchronize_source;
// pub use synchronize_source::*;

pub mod utility;
pub use utility::GenericError;

// pub mod web_client;

// pub mod state;
// pub use state::{State, NormaizedState, StateSchema};

pub fn main() {
  // _ = AppMutex::open_from_command_line_arguments_then_run();
}

// fn main() {}
