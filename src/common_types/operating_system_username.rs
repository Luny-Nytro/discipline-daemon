#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatingSystemUsername(String);

impl OperatingSystemUsername {
  /// Linux rules for a username are:
  /// It must be 1 to 32 characters long.
  /// It can only contain lowercase letters (a-z), digits (0-9), dashes (-), and underscores (_).
  /// It must start with a letter
  pub fn is_valid_linux_username(username: &String) -> bool {
    if username.len() < 1 || 32 < username.len() {
      return false;
    }
  
    let mut characters = username.chars();
    
    let Some(beginning) = characters.next() else {
      println!("Username is empty");
      return false; // Empty string case (shouldn't happen due to len check)
    };

    if !beginning.is_ascii_lowercase() {
      println!("Username first lett is not ascii lowercase");
      return false;
    }

    for character in characters {
      if !character.is_ascii_lowercase() 
      && !character.is_ascii_digit() 
      && character != '-' 
      && character != '_' {
        return false
      }
    }

    true
  }

  pub fn new(username: String) -> Option<OperatingSystemUsername> {
    if Self::is_valid_linux_username(&username) {
      Some(Self(username))
    } else {
      None
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}

pub mod database_serde {
  use super::OperatingSystemUsername;
  use crate::database::*;
  use crate::GenericError;

  impl SerializableScalarValue for OperatingSystemUsername {
    fn serialize_into(&self, ctx: SerializeScalarValueContext) {
      ctx.as_string(&self.0);
    }
  }

  impl DeserializableScalarValue for OperatingSystemUsername {
    fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
      value.as_string()
        .map(OperatingSystemUsername)
        .map_err(|error|
          error.change_context("Failed to deserialize OperationSystemUsername: Failed to cast value to string")
        )
    }
  }
}

mod serde_impl {
  use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
  use super::OperatingSystemUsername;
  
  impl Serialize for OperatingSystemUsername {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_str(&self.0)
    }
  }
  
  impl<'de> Deserialize<'de> for OperatingSystemUsername {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let username = String::deserialize(deserializer)?;
      match OperatingSystemUsername::new(username) {
        Some(value) => {
          Ok(value)
        }
        None => {
          Err(Error::custom("String is an invalid linux username."))
        }
      }
    }
  }
}

mod display_impl {
  use std::fmt;
  use super::OperatingSystemUsername;


  impl fmt::Display for OperatingSystemUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
    }
  }
}