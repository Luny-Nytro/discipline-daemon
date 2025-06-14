use serde::{Serialize, Deserialize};

use crate::{
  Uuid, OperatingSystemPassword, OperatingSystemUserId,
  OperatingSystemUsername, IsOperation, Daemon,
  user_screen_access_regulation,
  IntoPublic, InternalOperationOutcome
};

use super::{
  User, UserName, UserPublicRepr
};

pub mod change_name;
pub mod create;
pub mod delete;