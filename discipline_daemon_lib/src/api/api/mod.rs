mod rpc;

use std::fmt::Debug;
use crate::Database;
use super::*;

mod into_public;
pub use into_public::IntoPublic;

mod basic_web_server;
use basic_web_server::WebServer;

mod operations;
pub use operations::*;

mod server;

// pub use server::launch_thread;
// pub struct Api {
//   daemon_mutex: DaemonMutex,
//   // operation_map
// }

// impl Api {

// }