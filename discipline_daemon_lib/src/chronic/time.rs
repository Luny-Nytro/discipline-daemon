use crate::GenericError;

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
  
  pub fn try_from_timestamp_or_generic_error(value: u32) -> Result<Self, GenericError> {
    if Self::MIN.0 <= value && value <= Self::MAX.0 {
      Ok(Self(value))
    } else {
      Err(
        GenericError::new("creating a Time from a millisecind-based timestamp from midnight")
          .add_error(format!("timestamp must be in this range {} ..= {}", Self::MIN.0, Self::MAX.0))
          .add_attachment("timestamp", value.to_string())
      )
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

// mod database_serde {
//   use crate::database::*;
//   use crate::GenericError;
//   use super::Time;

//   impl IntoScalarValue for Time {
//     fn into_scalar_value(&self) -> impl IsScalarValue {
//       self.0
//     }
//   }

//   impl FromScalarValue for Time {
//     fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
//       value
//         .as_u32()
//         .and_then(Time::try_from_timestamp_or_generic_error)
//         .map_err(|error|
//           error.change_context("deserializing a Time")
//         )
//     }
//   }
// }