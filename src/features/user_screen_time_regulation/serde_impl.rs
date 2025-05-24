use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use super::PolicyName;

impl Serialize for PolicyName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.as_ref())
  }
}

impl<'de> Deserialize<'de> for PolicyName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let username = String::deserialize(deserializer)?;
    PolicyName::new(username)
      .map_err(|error| Error::custom(format!("{error:?}")))
  }
}