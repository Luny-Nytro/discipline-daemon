use std::process::Command;
use crate::GenericError;
use super::OperatingSystemUserName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatingSystemUserId(u32);

impl OperatingSystemUserId {
  pub fn new(user_id: u32) -> Self {
    Self(user_id)
  }

  pub fn as_raw(self) -> u32 {
    self.0
  }

  pub fn from_username(username: &OperatingSystemUserName) -> Result<OperatingSystemUserId, GenericError> {
    let output = Command::new("id")
      .arg("-u")
      .arg(username.as_ref()) 
      .output();
  
    let output = match output {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.OperatingSystemUserId.FromUsername: \nError: {error}");
        return Err(GenericError::new("getting an operating system user id given its name"));
      }
    };

    if output.status.success() {
      let user_id = match String::from_utf8(output.stdout) {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!("Discipline.OperatingSystemUserId.FromUsername.StdoutToString: \n{error}.");
          return Err(GenericError::new("getting an operating system user id given its name"));
        }
      };

      let user_id = match user_id.trim().to_string().parse::<u32>() {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!("Discipline.OperatingSystemUserId.FromUsername.ParseUserId: \nError: {error}.");
          return Err(GenericError::new("getting an operating system user id given its name"));
        }
      };

      return Ok(OperatingSystemUserId::new(user_id))
    } 
    
    match String::from_utf8(output.stderr) {
      Ok(stderr) => {
        eprintln!("Discipline.OperatingSystemUserId.FromUsername: \nUser: {username}. \nStderr: {stderr}");
        Err(GenericError::new("getting an operating system user id given its name"))
      }
      Err(_) => {
        eprintln!("Discipline.OperatingSystemUserId.FromUsername: \nUser: {username}.");
        Err(GenericError::new("getting an operating system user id given its name"))
      }
    }
  }
}

// mod database {
//   use crate::database::*;
//   use super::OperatingSystemUserId;
//   use crate::GenericError;

//   impl IntoScalarValue for OperatingSystemUserId {
//     fn into_scalar_value(&self) -> impl IsScalarValue {
//       &self.0
//     }
//   }

//   impl FromScalarValue for OperatingSystemUserId {
//     fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
//       value.as_u32()
//         .map(OperatingSystemUserId)
//         .map_err(|error|
//           error.change_context("deserializing an OperatingSystemUserId")
//         )
//     }
//   }
// }

mod serde_impl {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};
  use super::OperatingSystemUserId;
  
  impl Serialize for OperatingSystemUserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_u32(self.0)
    }
  }
  
  impl<'de> Deserialize<'de> for OperatingSystemUserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      Ok(OperatingSystemUserId::new(u32::deserialize(deserializer)?))
    }
  }
}

mod display_impl {
  use std::fmt;
  use super::OperatingSystemUserId;

  impl fmt::Display for OperatingSystemUserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
    }
  }
}