use std::process::Command;
use super::OperatingSystemUsername;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatingSystemUserId(u32);

impl OperatingSystemUserId {
  pub fn new(user_id: u32) -> Self {
    Self(user_id)
  }

  pub fn as_raw(self) -> u32 {
    self.0
  }

  pub fn from_username(username: &OperatingSystemUsername) -> Result<OperatingSystemUserId, ()> {
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
        return Err(());
      }
    };

    if output.status.success() {
      let user_id = match String::from_utf8(output.stdout) {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!("Discipline.OperatingSystemUserId.FromUsername.StdoutToString: \n{error}.");
          return Err(());
        }
      };

      let user_id = match user_id.trim().to_string().parse::<u32>() {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!("Discipline.OperatingSystemUserId.FromUsername.ParseUserId: \nError: {error}.");
          return Err(());
        }
      };

      return Ok(OperatingSystemUserId::new(user_id))
    } 
    
    match String::from_utf8(output.stderr) {
      Ok(stderr) => {
        eprintln!("Discipline.OperatingSystemUserId.FromUsername: \nUser: {username}. \nStderr: {stderr}");
        Err(())
      }
      Err(_) => {
        eprintln!("Discipline.OperatingSystemUserId.FromUsername: \nUser: {username}.");
        Err(())
      }
    }
  }
}

mod database {
  use crate::database::*;
  use super::OperatingSystemUserId;
  use crate::GenericError;

  impl SerializableScalarValue for OperatingSystemUserId {
    fn write_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
      context.write_u32(self.0)
    }
  }

  impl DeserializableScalarValue for OperatingSystemUserId {
    fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
      value.as_u32()
        .map(OperatingSystemUserId)
        .map_err(|error|
          error.change_context("deserializing an OperatingSystemUserId")
        )
    }
  }
}