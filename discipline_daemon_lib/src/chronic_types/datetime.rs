use std::fmt::Write;
use chrono::{self, Datelike, Timelike};
use crate::GenericError;

use super::{Duration, Time, Hour, Minute, Weekday};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
  pub fn now() -> Self {
    Self(chrono::Utc::now())
  }

  pub fn from_timestamp(timestamp: i64) -> Option<Self> {
    chrono::DateTime::from_timestamp_millis(timestamp).map(Self)
  }

  pub fn from_timestamp_or_generic_error(timestamp: i64) -> Result<Self, GenericError> {
    chrono
      ::DateTime
      ::from_timestamp_millis(timestamp)
      .ok_or_else(|| 
        GenericError::new("creating a datetime from the number of non-leap milliseconds since January 1, 1970 0:00:00.000 UTC (aka \"UNIX timestamp\")")
          .add_error("provided millisecond timestamp is out of valid range")
          .add_attachment("millisecond timestamp", timestamp.to_string())
      )
      .map(DateTime)
  }

  pub fn from_timestamp_micros(timestamp: i64) -> Option<Self> {
    chrono::DateTime::from_timestamp_micros(timestamp).map(Self)
  }

  pub fn year(&self) -> i32 {
    self.0.year()
  }

  pub fn month(&self) -> u32 {
    self.0.month()
  }

  pub fn month_day(&self) -> u32 {
    self.0.day()
  }

  // 0 to 59
  pub fn second(&self) -> u32 {
    self.0.second()
  }
  
  pub fn minute(&self) -> Minute {
    unsafe {
      Minute::unchekced_from(self.0.minute())
    }
  }
  
  // 0 to 23
  pub fn hour(&self) -> Hour {
    unsafe {
      Hour::unchecked_from(self.0.hour())
    }
  }

  pub fn timestamp(&self) -> i64 {
    self.0.timestamp_millis()
  }

  pub fn weekday(&self) -> Weekday {
    Weekday::from(self.0.weekday())
  }

  pub fn time(&self) -> Time {
    Time::from_hm(self.hour(), self.minute())
  }

  pub fn midnight(&self) -> DateTime {
    DateTime(
      self.0
        .with_second(0)
        .and_then(|datetime| datetime.with_minute(0))
        .and_then(|datetime| datetime.with_hour(0))
        .unwrap()
    )
  }

  pub fn duration_since_midnight(&self) -> Duration {
    self.since(&self.midnight()).unwrap()
  }

  pub fn since(&self, other: &DateTime) -> Option<Duration> {
    self
      .timestamp()
      .checked_sub(other.timestamp())
      // TODO: Make sure the cast to u64 is safe
      .map(|timestamp| Duration::from_milliseconds(timestamp as u64))
  }

  pub fn since_or_zero(&self, other: &DateTime) -> Duration {
    match self.timestamp().checked_sub(other.timestamp()) {
      None => Duration::from_milliseconds(0),
      // TODO: Make sure the cast to u64 is safe
      Some(value) => Duration::from_milliseconds(value as u64),
    }
  }

  /** 
   * Returns the date in a format like this:
   * 2025-03-19 08:00 AM
  */
  pub fn to_iso_8601_like_into(&self, into: &mut impl Write) {
    let hour24 = self.hour().value();
    let period;
    let hour12;

    if hour24 >= 12 {
      period = "PM";
      hour12 = hour24 - 12;
    } else {
      period = "AM";
      hour12 = hour24;
    }
    
    let year = self.year();
    let minute = self.minute().value0();
    let second = self.second();
    let month = self.month();
    let month_day = self.month_day();
  
    write!(into, "{}-{}-{} {}:{}:{} {}",
      year,
      month,
      month_day,
      hour12,
      minute,
      second,
      period,
    ).unwrap();
  }

  /** 
   * Returns the date in a format like this:
   * 2025-03-19 08:00 AM
  */
  pub fn to_iso_8601_like(&self) -> String {
    let mut string = String::new();
    self.to_iso_8601_like_into(&mut string);
    string
  }
}

use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};

impl Serialize for DateTime {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_i64(self.0.timestamp_millis())
  }
}

impl<'de> Deserialize<'de> for DateTime {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    // Deserialize the string and parse it into DateTime<Utc>
    let timestamp_milliseconds = i64::deserialize(deserializer)?;
    match chrono::DateTime::from_timestamp_millis(timestamp_milliseconds) {
      Some(value) => {
        Ok(DateTime(value))
      }
      None => {
        Err(Error::custom("Invalid DateTime timestamp."))
      }
    }
  }
}

pub mod database {
  use crate::database::*;
  use crate::GenericError;
  use super::DateTime;

  // pub struct Adapter;
  
  // impl Adapter {
  //   pub fn new() -> Self {
  //     Self {}
  //   }
  // }

  // impl ScalarTypeAdapter for Adapter {
  //   type Type = DateTime;

  //   fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
  //     context.write_i64(self.timestamp());    
  //   }

  //   fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
  //     let timestamp = value.as_i64().map_err(|error|
  //       error.change_context("Failed to create a DateTime from ColumnValue: ColumnValue is not a i64")
  //     )?;

  //     DateTime::from_timestamp(timestamp).ok_or_else(|| 
  //       GenericError::new("Failed to create a DateTime from an i64 ColumnValue: Failed to create a DateTime from timestamp")
  //         .attach_info("timestamp", timestamp.to_string())
  //     )  
  //   }
  // }

  impl IntoScalarValue for DateTime {
    fn write_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
      context.write_i64(self.timestamp())
    }
  }

  impl FromScalarValue for DateTime {
    fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
      value
        .as_i64()
        .and_then(DateTime::from_timestamp_or_generic_error)
        .map_err(|error|
          error.change_context("deserializing a datetime")
        )
    }
  }
}