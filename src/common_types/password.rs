#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateError {
  PasswordTooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
  const MAX_LENGTH: usize = 40;

  pub fn new(value: String) -> Result<Self, CreateError> {
    if value.len() > Self::MAX_LENGTH {
      Err(CreateError::PasswordTooLong)
    } else {
      Ok(Self(value))
    }
  }
}

mod database_serde {
  use crate::{database::{ColumnValue, DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext}, GenericError};
  use super::Password;

  impl SerializableScalarValue for Password {
    fn serialize_into(&self, ctx: SerializeScalarValueContext) {
      ctx.as_string(&self.0);
    }
  }

  impl DeserializableScalarValue for Password {
    fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
      value.as_string()
        .map(Password)
        .map_err(|error|
          error.change_context("Failed to deserialize password: Failed to cast value to string")
        )
    }
  }
}

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