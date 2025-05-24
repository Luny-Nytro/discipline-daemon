use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weekday {
  /// Monday.
  Monday = 0,
  /// Tuesday.
  Tuesday = 1,
  /// Wednesday.
  Wednesday = 2,
  /// Thursday.
  Thursday = 3,
  /// Friday.
  Friday = 4,
  /// Saturday.
  Saturday = 5,
  /// Sunday.
  Sunday = 6,
}

pub use Weekday::*;

impl Weekday {
  pub fn from_number_wrapping(number: u32) -> Self {
    match number % 7 {
      0 => Weekday::Sunday,
      1 => Weekday::Monday,
      2 => Weekday::Tuesday,
      3 => Weekday::Wednesday,
      4 => Weekday::Thursday,
      5 => Weekday::Friday,
      6 => Weekday::Saturday,
      // Because the cosmos demands order 🌌
      _ => unreachable!(), 
    }
  }

  pub unsafe fn unchekced_from_number(number: u32) -> Weekday {
    match number {
      0 => Weekday::Sunday,
      1 => Weekday::Monday,
      2 => Weekday::Tuesday,
      3 => Weekday::Wednesday,
      4 => Weekday::Thursday,
      5 => Weekday::Friday,
      6 => Weekday::Saturday,
      _ => unreachable!()
    }
  }

  /// The next day in the week.
  pub const fn successor(&self) -> Weekday {
    match *self {
      Monday => Tuesday,
      Tuesday => Wednesday,
      Wednesday => Thursday,
      Thursday => Friday,
      Friday => Saturday,
      Saturday => Sunday,
      Sunday => Monday,
    }
  }

  /// The previous day in the week.
  pub const fn predecessor(&self) -> Weekday {
    match *self {
      Monday => Sunday,
      Tuesday => Monday,
      Wednesday => Tuesday,
      Thursday => Wednesday,
      Friday => Thursday,
      Saturday => Friday,
      Sunday => Saturday,
    }
  }

  /// Returns a day-of-week number starting from Monday = 1. (ISO 8601 weekday number)
  pub const fn number_from_monday(&self) -> u32 {
    self.days_since(Monday) + 1
  }

  /// Returns a day-of-week number starting from Sunday = 1.
  pub const fn number_from_sunday(&self) -> u32 {
    self.days_since(Sunday) + 1
  }

  /// Returns a day-of-week number starting from Monday = 0.
  pub const fn days_since_monday(&self) -> u32 {
    self.days_since(Monday)
  }

  /// Returns a day-of-week number starting from Sunday = 0.
  pub const fn days_since_sunday(&self) -> u32 {
    self.days_since(Sunday)
  }

  pub const fn days_till(self, later: Weekday) -> u32 {
    let earlier = self as u32;
    let later = later as u32;

    if earlier < later {
      earlier - later
    } else {
      0
    }
  }

  /// The number of days since the given day.
  pub const fn days_since(&self, other: Weekday) -> u32 {
    let lhs = *self as u32;
    let rhs = other as u32;
    if lhs < rhs {
      7 + lhs - rhs
    } else {
      lhs - rhs
    }
  }
}

impl fmt::Display for Weekday {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(match *self {
      Monday => "Monday",
      Tuesday => "Tuesday",
      Wednesday => "Wednesday",
      Thursday => "Thursday",
      Friday => "Friday",
      Saturday => "Saturday",
      Sunday => "Sunday",
    })
  }
}

/// Any weekday can be represented as an integer from 0 to 6.
/// Do not heavily depend on this though; use explicit methods whenever possible.
impl TryFrom<u8> for Weekday {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Monday),
      1 => Ok(Tuesday),
      2 => Ok(Wednesday),
      3 => Ok(Thursday),
      4 => Ok(Friday),
      5 => Ok(Saturday),
      6 => Ok(Sunday),
      _ => Err(()),
    }
  }
}

impl From<chrono::Weekday> for Weekday {
  fn from(value: chrono::Weekday) -> Self {
    match value {
      chrono::Weekday::Fri => Friday,
      chrono::Weekday::Mon => Monday,
      chrono::Weekday::Sat => Saturday,
      chrono::Weekday::Sun => Sunday,
      chrono::Weekday::Thu => Thursday,
      chrono::Weekday::Tue => Tuesday,
      chrono::Weekday::Wed => Wednesday,
    }
  }
}

use serde::{Serialize, Serializer, Deserialize, Deserializer};

impl Serialize for Weekday {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u8(*self as u8)
  }
}

impl<'de> Deserialize<'de> for Weekday {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = u8::deserialize(deserializer)?;
    match value {
      0 => Ok(Weekday::Sunday),
      1 => Ok(Weekday::Monday),
      2 => Ok(Weekday::Tuesday),
      3 => Ok(Weekday::Wednesday),
      4 => Ok(Weekday::Thursday),
      5 => Ok(Weekday::Friday),
      6 => Ok(Weekday::Saturday),
      _ => Err(serde::de::Error::custom("Invalid weekday value")),
    }
  }
}

pub mod database_serde {
  use crate::{database::{ColumnValue, DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext}, GenericError};
  use super::*;

  // pub struct Adapter {}

  // impl Adapter {
  //   pub fn new() -> Self {
  //     Self {}
  //   }
  // }

  // impl ScalarTypeAdapter for Adapter {
  //   type Type = Weekday;

  //   fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
  //     match self {
  //       Sunday => context.as_u8(0),
  //       Monday => context.as_u8(1),
  //       Tuesday => context.as_u8(2),
  //       Wednesday => context.as_u8(3),
  //       Thursday => context.as_u8(4),
  //       Friday => context.as_u8(5),
  //       Saturday => context.as_u8(6),
  //     }        
  //   }

  //   fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
  //     let number = value.as_u8().map_err(|error| 
  //       error.change_context("Failed to create a weekday from a column value: Expected value to be an integer")
  //     )?;
      
  //     match number {
  //       0 => Ok(Sunday),
  //       1 => Ok(Monday),
  //       2 => Ok(Tuesday),
  //       3 => Ok(Wednesday),
  //       4 => Ok(Thursday),
  //       5 => Ok(Friday),
  //       6 => Ok(Saturday),
  //       _ => {
  //         Err(
  //           GenericError::new("Faild to create a weekday from integer: Casted column value as u8 but the resulting u8 is outside valid range 0 ..= 6")
  //             .attach_info("u8 number", number.to_string())
  //         )
  //       }
  //     }
  //   }
  // }

  impl SerializableScalarValue for Weekday {
    fn serialize_into(&self, context: SerializeScalarValueContext) {
      match self {
        Sunday => context.as_u8(0),
        Monday => context.as_u8(1),
        Tuesday => context.as_u8(2),
        Wednesday => context.as_u8(3),
        Thursday => context.as_u8(4),
        Friday => context.as_u8(5),
        Saturday => context.as_u8(6),
      }
    }
  }

  impl DeserializableScalarValue for Weekday {
    fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
      let number = value.as_u8().map_err(|error| 
        error.change_context("Failed to create a weekday from a column value: Expected value to be an integer")
      )?;
      
      match number {
        0 => Ok(Sunday),
        1 => Ok(Monday),
        2 => Ok(Tuesday),
        3 => Ok(Wednesday),
        4 => Ok(Thursday),
        5 => Ok(Friday),
        6 => Ok(Saturday),
        _ => {
          Err(
            GenericError::new("Faild to create a weekday from integer: Casted column value as u8 but the resulting u8 is outside valid range 0 ..= 6")
              .add_attachment("u8 number", number.to_string())
          )
        }
      }
    }
  }  
}