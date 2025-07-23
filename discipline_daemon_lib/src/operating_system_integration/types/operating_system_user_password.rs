use crate::GenericError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatingSystemUserPassword(String);

impl OperatingSystemUserPassword {
  pub fn new(password: String) -> Option<OperatingSystemUserPassword> {
    if password.is_empty() {
      None
    } else {
      Some(Self(password))
    }
  }

  pub fn new_or_generic_error(password: String) -> Result<OperatingSystemUserPassword, GenericError> {
    if password.is_empty() {
      Err(
        GenericError::new("creating an OperatingSystemPassword")
          .add_error("password may not be the empty string")
      )
    } else {
      Ok(Self(password))
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }

  pub fn generate_random_password() -> OperatingSystemUserPassword {
    use rand::{distr::Uniform, Rng};

    let mut rng = rand::rng();
    let letters = Uniform::new_inclusive(b'a', b'z').unwrap(); // ASCII range for lowercase letters
  
    let password = (0..10)
      .map(|_| rng.sample(&letters) as char)
      .collect();

    OperatingSystemUserPassword(password)
  }
}

mod serde_impl {
  use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
  use super::OperatingSystemUserPassword;
  
  impl Serialize for OperatingSystemUserPassword {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_str(&self.0)
    }
  }
  
  impl<'de> Deserialize<'de> for OperatingSystemUserPassword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let password = String::deserialize(deserializer)?;
      match OperatingSystemUserPassword::new(password) {
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