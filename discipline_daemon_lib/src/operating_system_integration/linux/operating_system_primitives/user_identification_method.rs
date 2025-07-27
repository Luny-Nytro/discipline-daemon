use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserIdentificationMethod {
  Id(UserId),
  Name(UserName),
}
