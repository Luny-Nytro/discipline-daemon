#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u32);

impl UserId {
  pub fn new(user_id: u32) -> Self {
    Self(user_id)
  }

  pub fn as_raw(self) -> u32 {
    self.0
  }
}

mod serde_impl {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};
  use super::UserId;
  
  impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_u32(self.0)
    }
  }
  
  impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      Ok(UserId::new(u32::deserialize(deserializer)?))
    }
  }
}

mod display_impl {
  use std::fmt;
  use super::UserId;

  impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
    }
  }
}