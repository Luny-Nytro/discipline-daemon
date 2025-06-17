#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hour(u32);

const MIN_VALUE: u32 = 0;
const MAX_VALUE: u32 = 23;

impl Hour {
  pub const MIN: Self = Self(MIN_VALUE);
  pub const MAX: Self = Self(MAX_VALUE);

  pub fn try_from0(value: u32) -> Option<Self> {
    if value <= MAX_VALUE {
      Some(Self(value))
    } else {
      None
    }
  }

  pub fn from_0_or_generic_error(value: u32) -> Result<Self, GenericError> {
    if value <= MAX_VALUE {
      Ok(Self(value))
    } else {
      Err(
        GenericError::new("creating an hour from an integer")
          .add_error("integer must be this range 0 ..= 23")
          .add_attachment("integer", value.to_string())
      )
    }
  }

  pub unsafe fn unchecked_from(value: u32) -> Self {
    Self(value)
  }

  pub fn value(&self) -> u32 {
    self.0
  }

  pub fn value1(&self) -> u32 {
    self.0 + 1
  }

  pub fn difference(&self, rhs: Hour) -> u32 {
    let lhs = self.0;
    let rhs = rhs.0;

    if lhs >= rhs {
      lhs - rhs
    } else {
      (24 - rhs) + lhs
    }
  }
  
  pub fn to_12_based_string_with_period(&self) -> String {
    let hour = self.value();

    let hour_12 = if hour == 0 {
      12
    } else if hour > 12 {
      hour - 12
    } else {
      hour
    };

    let period = if hour < 12 { 
      "AM" 
    } else { 
      "PM" 
    };

    format!("{:02} {}", hour_12, period)
  }
}

use serde::{Serialize, Deserialize, Serializer, Deserializer};

use crate::GenericError;

impl Serialize for Hour {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u32(self.0)
  }
}

impl<'de> Deserialize<'de> for Hour {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = u32::deserialize(deserializer)?;
    if value > 23 {
      return Err(serde::de::Error::custom("Hour must be between 0 and 23"));
    }
    Ok(Hour(value))
  }
}

pub mod database {
  use crate::database::*;
  use crate::GenericError;
  use super::Hour;

  // pub struct Adapter {}

  // impl Adapter {
  //   pub fn new() -> Self {
  //     Self {}
  //   }
  // }

  // impl ScalarTypeAdapter for Adapter {
  //   type Type = Hour;

  //   fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
  //     context.write_u32(self.value());
  //   }

  //   fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
  //     let number = value.as_u32().map_err(|error|
  //       error.change_context("Failed to create an Hour from a ColumnValue: Expected ColumnValue to be a u32")
  //     )?;

  //     Hour::try_from0(number).ok_or_else(|| 
  //       GenericError::new("Failed to create an Hour from a u32 ColumnValue: Expected ColumnValue to be in this range 0 ..= 23")
  //         .attach_info("ColumnValue", number.to_string())
  //     )        
  //   }
  // }

  impl IntoScalarValue for Hour {
    fn into_scalar_value(&self) -> impl IsScalarValue {
      self.value()
    }
  }

  impl FromScalarValue for Hour {
    fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
      value
        .as_u32()
        .and_then(Hour::from_0_or_generic_error)
        .map_err(|error|
          error.change_context("deserializing an hour")
        )
    }
  }
}