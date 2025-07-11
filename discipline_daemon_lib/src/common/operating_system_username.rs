use crate::GenericError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatingSystemUsername(String);

impl OperatingSystemUsername {
  pub fn validate_linux_username(username: &String) -> Result<(), GenericError> {
    if username.len() < 1 || 32 < username.len() {
      return Err(
        GenericError::new("validate linux username")
          .add_error("linux username length must begine with an ascii lowercase letter")
          .add_attachment("username", username)
          .add_attachment("username beginning", username.len().to_string())
      );
    }
  
    let mut characters = username.chars();
    
    let Some(beginning) = characters.next() else {
      return Err(
        GenericError::new("validate linux username")
          .add_error("linux username length must begine with an ascii lowercase letter")
          .add_attachment("username", username)
          .add_attachment("username beginning", username.len().to_string())
      );
    };

    if !beginning.is_ascii_lowercase() {
      return Err(
        GenericError::new("validate linux username")
          .add_error("linux username length must begine with an ascii lowercase letter")
          .add_attachment("username", username)
          .add_attachment("username beginning", beginning.to_string())
      );
    }

    for character in characters {
      if !character.is_ascii_lowercase() 
      && !character.is_ascii_digit() 
      && character != '-' 
      && character != '_' 
      {
        return Err(
          GenericError::new("validate linux username")
            .add_error("linux username may only conttaon ascii lowercase letters, ascii digits, hypens or underscores")
            .add_attachment("username", username)
        );
      }
    }

    Ok(())
  }

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

  pub fn new_or_generic_error(username: String) -> Result<OperatingSystemUsername, GenericError> {
    Self::validate_linux_username(&username)?;
    Ok(Self(username))
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}

// pub mod database_serde {
//   use super::OperatingSystemUsername;
//   use crate::database::*;
//   use crate::GenericError;

//   impl IntoScalarValue for OperatingSystemUsername {
//     fn into_scalar_value(&self) -> impl IsScalarValue {
//       &self.0
//     }
//   }

//   impl FromScalarValue for OperatingSystemUsername {
//     fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
//       value.as_string()
//         .and_then(OperatingSystemUsername::new_or_generic_error)
//         .map_err(|error|
//           error.change_context("deserializing an OperatingSystemUsername")
//         )
//     }
//   }
// }

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