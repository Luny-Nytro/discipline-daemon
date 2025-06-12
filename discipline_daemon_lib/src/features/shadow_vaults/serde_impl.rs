use serde::{Serializer, Deserializer, Serialize, Deserialize, de::Error};
use super::{CreateShadowVaultDatumError, CreateShadowVaultNameError, ShadowVaultDatum, ShadowVaultName};

// SECTION: ShadowVaultName.
impl Serialize for ShadowVaultName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer 
  {
    serializer.serialize_str(self.as_ref())
  }
}

impl<'de> Deserialize<'de> for ShadowVaultName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let name = String::deserialize(deserializer)?;
    ShadowVaultName::new(name).map_err(|error| {
      match error {
        CreateShadowVaultNameError::NameTooLong => {
          D::Error::custom("NameTooLong")
        }
        CreateShadowVaultNameError::NameTooShort => {
          D::Error::custom("NameTooShort")
        }
      }
    })
  }
}

// SECTION: ShadowVaultDatum.
impl Serialize for ShadowVaultDatum {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer 
  {
    serializer.serialize_str(self.as_ref())
  }
}

impl<'de> Deserialize<'de> for ShadowVaultDatum {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let datum = String::deserialize(deserializer)?;
    ShadowVaultDatum::new(datum).map_err(|error| {
      match error {
        CreateShadowVaultDatumError::DatumTooLong => {
          D::Error::custom("DatumIsLongerThan40Characters")
        }
      }
    })
  }
}