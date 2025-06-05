#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatingSystemPassword(String);

impl OperatingSystemPassword {
  pub fn new(password: String) -> Option<OperatingSystemPassword> {
    if password.is_empty() {
      None
    } else {
      Some(Self(password))
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }

  pub fn generate_random_password() -> OperatingSystemPassword {
    use rand::{distr::Uniform, Rng};

    let mut rng = rand::rng();
    let letters = Uniform::new_inclusive(b'a', b'z').unwrap(); // ASCII range for lowercase letters
  
    let password = (0..10)
      .map(|_| rng.sample(&letters) as char)
      .collect();

    OperatingSystemPassword(password)
  }
}

mod database_serde {
  use crate::database::*;
  use super::OperatingSystemPassword;
  use crate::GenericError;

  impl SerializableScalarValue for OperatingSystemPassword {
    fn write_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
      context.write_string(&self.0);
    }
  }

  impl DeserializableScalarValue for OperatingSystemPassword {
    fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
      let string = value.as_string().map_err(|error|
        error.change_context("Failed to deserialize OperatingSystemPassword: Failed to cast value to string")
      )?;

      OperatingSystemPassword::new(string).ok_or_else(||
        GenericError::new("Failed to deserialize OperatingSystemPassword: Provided string is empty")
      )
    }
  }
}

mod serde_impl {
  use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
  use super::OperatingSystemPassword;
  
  impl Serialize for OperatingSystemPassword {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_str(&self.0)
    }
  }
  
  impl<'de> Deserialize<'de> for OperatingSystemPassword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let password = String::deserialize(deserializer)?;
      match OperatingSystemPassword::new(password) {
        Some(value) => {
          Ok(value)
        }
        None => {
          Err(Error::custom("String is an invalid linux user password."))
        }
      }
    }
  }
}