#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Minute(u32);

const MIN_VALUE: u32 = 0;
const MAX_VALUE: u32 = 59;

impl Minute {
  pub const MIN: Self = Self(MIN_VALUE);
  pub const MAX: Self = Self(MAX_VALUE);
  
  pub fn try_from0(value: u32) -> Option<Self> {
    if value < 60 {
      Some(Self(value))
    } else {
      None
    }
  }

  pub fn from_0_or_generic_error(value: u32) -> Result<Self, GenericError> {
    if value < 60 {
      Ok(Self(value))
    } else {
      Err(
        GenericError::new("creating a minute from an integer")
          .add_error("integer must be in this range 0 =.. 59")
          .add_attachment("integer", value.to_string())
      )
    }
  }

  pub unsafe fn unchekced_from(value: u32) -> Self {
    Self(value)
  }

  pub fn value0(&self) -> u32 {
    self.0
  }

  pub fn value1(&self) -> u32 {
    self.0 + 1
  }
}

use serde::{Serialize, Deserialize, Serializer, Deserializer};

use crate::GenericError;

impl Serialize for Minute {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u32(self.0)
  }
}

impl<'de> Deserialize<'de> for Minute {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = u32::deserialize(deserializer)?;
    if value > 59 {
      return Err(serde::de::Error::custom("Minute must be between 0 and 59"));
    }
    Ok(Minute(value))
  }
}


// pub mod database {
//   use crate::database::*;
//   use crate::GenericError;
//   use super::Minute;

//   impl IntoScalarValue for Minute {
//     fn into_scalar_value(&self) -> impl IsScalarValue {
//       self.value0()
//     }
//   }

//   impl FromScalarValue for Minute {
//     fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
//       value
//         .as_u32()
//         .and_then(Minute::from_0_or_generic_error)
//         .map_err(|error|
//           error.change_context("deserializing a minute")
//         )
//     }
//   }
// }