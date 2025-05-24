use super::{Duration, Hour, Minute};

const MS_PER_SECOND: u32 = 1000;
const MS_PER_MINUTE: u32 = MS_PER_SECOND * 60;
const MS_PER_HOUR: u32 = MS_PER_MINUTE * 60;
// const MS_PER_DAY: u32 = MS_PER_HOUR * 24;
// const MS_PER_WEEK: u32 = MS_PER_DAY * 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(u32);

/// The minimum value represents the very start of the day.
pub const MIN_VALUE: u32 = 0;
/// The maximum value represents the very end of the day.
pub const MAX_VALUE: u32 = 86400000 - 1;

const MILLISECONDS_PER_DAY: u32 = 1000 * 60 * 60 * 24;

impl Time {
  /// The minimum value represents the very start of the day.
  const MIN: Time = Time(MIN_VALUE);
  /// The maximum value represents the very end of the day.
  const MAX: Time = Time(MAX_VALUE);

  pub fn from_hm(hour: Hour, minute: Minute) -> Self {
    Self(hour.value() * MS_PER_HOUR + minute.value0() * MS_PER_MINUTE)
  }

  pub fn try_from_timestamp(value: u32) -> Option<Self> {
    if Self::MIN.0 <= value && value <= Self::MAX.0 {
      Some(Self(value))
    } else {
      None
    }
  }

  pub unsafe fn unchecked_timestamp(value: u32) -> Self {
    Self(value)
  }

  pub fn wrapping_from_timestamp(value: u32) -> Self {
    Self(value % (MILLISECONDS_PER_DAY + 1))
  }


  pub fn hour(&self) -> u32 {
    self.0 / MS_PER_HOUR
  }

  pub fn minute(&self) -> u32 {
    self.0 % MS_PER_HOUR / MS_PER_MINUTE
  }

  pub fn value(&self) -> u32 {
    self.0
  }

  pub fn from_midnight(&self) -> Duration {
    Duration::from_milliseconds(
      self.0 as u64
    )
  }

  pub fn till_midnight(&self) -> Duration {
    Duration::from_milliseconds(
      (MAX_VALUE - self.0) as u64
    )
  }

  pub fn to_12_hour_based_string_with_period(&self) -> String {
    let hour = self.hour();
    let minute = self.minute();

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

    format!("{:02}:{:02} {}", hour_12, minute, period)
  }

  /// Adds a millisecond duration, wrapping around at 24 hours.
  pub fn wrapping_add(self, duration: Duration) -> Self {
    // Compute the wrapped result directly using modular arithmetic.
    // We use `(MS_PER_DAY + 1)` because the range is **inclusive**.
    let wrapped_ms = (self.0 as u64 + duration.total_milliseconds()) 
    % (MILLISECONDS_PER_DAY as u64 + 1);
    Self(wrapped_ms as u32)
  }

  pub fn milliseconds_since_midnight(&self) -> u32 {
    self.0
  }
}

mod serde_impl {
  use super::{Time, MAX_VALUE};
  use serde::{Serialize, Serializer, Deserialize, Deserializer};

  impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_u32(self.0)
    }
  }

  impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let value = u32::deserialize(deserializer)?;
      if value > MAX_VALUE {
        return Err(serde::de::Error::custom("Time must be between 0 and 86400000 (inclusive)"));
      }
      Ok(Time(value))
    }
  }

}

mod database_serde {
  use crate::database::*;
  use crate::GenericError;
  use super::{Time, MIN_VALUE, MAX_VALUE};

  // pub struct Adapter {}

  // impl Adapter {
  //   pub fn new() -> Self {
  //     Self {}
  //   }
  // }

  // impl ScalarTypeAdapter for Adapter {
  //   type Type = Time;

  //   fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
  //     context.as_u32(self.0);      
  //   }

  //   fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
  //     let timestamp = value.as_u32().map_err(|error|
  //       error.change_context("Failed to create a Time from a ColumnValue: Expected ColumnValue to be a u32 number")
  //     )?;

  //     Time::try_from_timestamp(timestamp).ok_or_else(||
  //       GenericError::new(format!("Failed to create a Time from a u32 ColumnValue: Expected ColumnValue to be in this range {MIN_VALUE} ..= {MAX_VALUE}"))
  //         .attach_info("ColumnValue", timestamp.to_string())
  //     )        
  //   }
  // }

  impl SerializableScalarValue for Time {
    fn serialize_into(&self, ctx: SerializeScalarValueContext) {
      ctx.as_u32(self.0);
    }
  }

  impl DeserializableScalarValue for Time {
    fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
      let timestamp = value.as_u32().map_err(|error|
        error.change_context("Failed to create a Time from a ColumnValue: Expected ColumnValue to be a u32 number")
      )?;

      Time::try_from_timestamp(timestamp).ok_or_else(||
        GenericError::new(format!("Failed to create a Time from a u32 ColumnValue: Expected ColumnValue to be in this range {MIN_VALUE} ..= {MAX_VALUE}"))
          .add_attachment("ColumnValue", timestamp.to_string())
      )
    }
  }
}