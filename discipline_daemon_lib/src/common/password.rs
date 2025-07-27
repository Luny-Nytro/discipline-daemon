use crate::GenericError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateError {
  PasswordTooLong,
  PasswordTooShort,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
  const MIN_LENGTH: usize = 1;
  const MAX_LENGTH: usize = 40;

  pub fn new(value: String) -> Result<Self, CreateError> {
    if value.len() < Self::MAX_LENGTH {
      Err(CreateError::PasswordTooShort)
    } else if value.len() > Self::MAX_LENGTH {
      Err(CreateError::PasswordTooLong)
    } else {
      Ok(Self(value))
    }
  }

  pub fn new_or_generic_error(value: String) -> Result<Self, GenericError> {
    if value.len() < Self::MAX_LENGTH {
      Err(
        GenericError::new("creating a Password")
          .add_error("password is too short")
          .add_attachment("minimum valid length", Self::MIN_LENGTH.to_string())
          .add_attachment("password", value)
      )
    } else if value.len() > Self::MAX_LENGTH {
      Err(
        GenericError::new("creating a Password")
          .add_error("password is too long")
          .add_attachment("maximum valid length", Self::MAX_LENGTH.to_string())
          .add_attachment("password", value)
      )
    } else {
      Ok(Self(value))
    }
  }
}

// mod database {
//   use crate::database::*;
//   use crate::GenericError;
//   use super::Password;

//   impl IntoScalarValue for Password {
//     fn into_scalar_value(&self) -> impl IsScalarValue {
//       &self.0
//     }
//   }

//   impl FromScalarValue for Password {
//     fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
//       value.as_string()
//         .map(Password)
//         .map_err(|error|
//           error.change_context("deserializing a password")
//         )
//     }
//   }
// }

mod serde_impl {
  use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
  use super::Password;
  
  impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_str(&self.0)
    }
  }
  
  impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let password = String::deserialize(deserializer)?;
      match Password::new(password) {
        Ok(value) => {
          Ok(value)
        }
        Err(_) => {
          Err(Error::custom("Password too long."))
        }
      }
    }
  }
}