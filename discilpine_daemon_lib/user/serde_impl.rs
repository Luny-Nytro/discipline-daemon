use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use super::UserName;

impl Serialize for UserName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.as_ref())
  }
}

impl<'de> Deserialize<'de> for UserName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    UserName::new(String::deserialize(deserializer)?)
      .map_err(|error| Error::custom(format!("{error:?}")))
  }
}