use std::fmt::Write;
// a positive duration
// a datetime type representing datetime points since unix epoch
// advance datetime by given duration
// go back datetime by given duration

pub const MS_PER_SECOND: u64 = 1000;
pub const MS_PER_MINUTE: u64 = MS_PER_SECOND * 60;
pub const MS_PER_HOUR: u64 = MS_PER_MINUTE * 60;
pub const MS_PER_DAY: u64 = MS_PER_HOUR * 24;
pub const MS_PER_WEEK: u64 = MS_PER_DAY * 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(u64);

impl Duration {
  pub const ZERO: Duration = Duration(0);
  
  pub const fn from_milliseconds(milliseconds: u64) -> Self {
    Self(milliseconds)
  }

  pub const fn from_seconds(seconds: u64) -> Option<Self> {
    if let Some(milliseconds) = seconds.checked_mul(MS_PER_SECOND) {
      Some(Self(milliseconds))
    } else {
      None
    }
  }

  pub const fn from_minutes(minutes: u64) -> Option<Self> {
    if let Some(milliseconds) = minutes.checked_mul(MS_PER_MINUTE) {
      Some(Self(milliseconds))
    } else {
      None
    }
  }
  
  pub const fn unchecked_from_minutes(minutes: u64) -> Self {
    Self(minutes * MS_PER_MINUTE)
  }

  pub const fn unchecked_from_hours(hours: u64) -> Self {
    Self(hours * MS_PER_HOUR)
  }

  pub const fn from_hours(hours: u64) -> Option<Self> {
    if let Some(milliseconds) = hours.checked_mul(MS_PER_HOUR) {
      Some(Self(milliseconds))
    } else {
      None
    }
  }

  pub const fn from_days(days: u64) -> Option<Self> {
    if let Some(milliseconds) = days.checked_mul(MS_PER_DAY) {
      Some(Self(milliseconds))
    } else {
      None
    }
  }

  pub const fn from_weeks(weeks: u64) -> Option<Self> {
    if let Some(milliseconds) = weeks.checked_mul(MS_PER_WEEK) {
      Some(Self(milliseconds))
    } else {
      None
    }
  }

  pub const fn new_week() -> Self {
    Self(7 * MS_PER_DAY)
  }

  pub const fn unchecked_from_days(days: u64) -> Self {
    Self::from_milliseconds(days * MS_PER_DAY)
  }

  pub const fn unchecked_from_days_u32(days: u32) -> Self {
    Self::from_milliseconds(days as u64 * MS_PER_DAY)
  }

  pub const fn is_zero(&self) -> bool {
    self.0 == 0
  }

  pub const fn total_seconds(&self) -> u64 {
    self.0 / MS_PER_SECOND
  }

  pub const fn total_minutes(&self) -> u64 {
    self.0 / MS_PER_MINUTE
  }

  pub const fn total_hours(&self) -> u64 {
    self.0 / MS_PER_HOUR
  }

  pub const fn total_days(&self) -> u64 {
    self.0 / MS_PER_DAY
  }

  pub const fn total_weeks(&self) -> u64 {
    self.0 / MS_PER_WEEK
  }

  pub const fn total_milliseconds(&self) -> u64 {
    self.0
  }

  pub fn checked_add(&self, other: &Duration) -> Option<Self> {
    self.0.checked_add(other.0).map(Self::from_milliseconds)
  }

  pub fn unchecked_add(&self, other: &Duration) -> Self {
    Self(self.0 - other.0)
  }

  pub fn checked_sub(&self, other: &Duration) -> Option<Self> {
    self.0.checked_sub(other.0).map(Self::from_milliseconds)
  }

  pub fn unchecked_sub(&self, other: &Duration) -> Self {
    Self(self.0 - other.0)
  }

  pub fn checked_mul(&self, by: u64) -> Option<Self> {
    self.0.checked_mul(by).map(Self::from_milliseconds)
  }

  pub fn checked_div(&self, by: u64) -> Option<Self> {
    self.0.checked_div(by).map(Self::from_milliseconds)
  }

  pub fn to_std(&self) -> std::time::Duration {
    std::time::Duration::from_millis(self.0)
  }

  pub fn to_chrono(&self) -> Option<chrono::Duration> {
    Some(chrono::Duration::milliseconds(match self.0.try_into() {
      Ok(value) => value,
      Err(_) => return None,
    }))
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();
    let mut milliseconds = self.total_milliseconds();

    let days = milliseconds / MS_PER_DAY;
    milliseconds %= MS_PER_DAY;
    
    let hours = milliseconds / MS_PER_HOUR;
    milliseconds %= MS_PER_HOUR;

    let minutes = milliseconds / MS_PER_MINUTE;
    milliseconds %= MS_PER_MINUTE;

    let seconds = milliseconds / MS_PER_SECOND;
    // milliseconds %= MS_PER_SECOND;

    if days > 0 {
      write!(string, "{}D", days).unwrap();
    }
    if hours > 0 {
      if string.len() > 0 {
        write!(string, " {}H", hours).unwrap();
      } else {
        write!(string, "{}H", hours).unwrap();
      }
    }
    if minutes > 0 {
      if string.len() > 0 {
        write!(string, " {}M", minutes).unwrap();
      } else {
        write!(string, "{}M", minutes).unwrap();
      }
    }
    if seconds > 0 {
      if string.len() > 0 {
        write!(string, " {}S", seconds).unwrap();
      } else {
        write!(string, "{}S", seconds).unwrap();
      }
    }

    string
  }
}

use serde::{Serialize, Serializer, Deserialize, Deserializer};

impl Serialize for Duration {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u64(self.0)
  }
}

impl<'de> Deserialize<'de> for Duration {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(Duration::from_milliseconds(u64::deserialize(deserializer)?))
  }
}

pub mod database_serde {
  use crate::database::*;
  use crate::GenericError;
  use super::Duration;

  // pub struct Adapter;

  // impl Adapter {
  //   pub fn new() -> Self {
  //     Self {}
  //   }
  // }

  // impl ScalarTypeAdapter for Adapter {
  //   type Type = Duration;

  //   fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
  //     context.write_u64(self.total_milliseconds());      
  //   }

  //   fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
  //     let milliseconds = value.as_u64().map_err(|error|
  //       error.change_context("Failed to create a Duration from a ColumnValue: ColumnValue is not a u64 number")
  //     )?;

  //     Ok(Duration::from_milliseconds(value))        
  //   }
  // }

  impl SerializableScalarValue for Duration {
    fn write_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
      context.write_u64(self.total_milliseconds())
    }
  }

  impl DeserializableScalarValue for Duration {
    fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
      value
        .as_u64()
        .map(Duration::from_milliseconds)
        .map_err(|error|
          error.change_context("deserializing a duration")
        )
    }
  }
}