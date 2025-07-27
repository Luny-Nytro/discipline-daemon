mod async_operation_scheduling;
use async_operation_scheduling::*;

mod operating_system_primitives;
pub use operating_system_primitives::*;

mod operating_system_interactions;
use operating_system_interactions::*;

mod users;
pub use users::{OperatingSystemIntegration, OperatingSystemIntegrationData, UserInfo};

pub mod screen_access_regulation_application;
pub mod internet_access_regulation_application;


use crate::*;