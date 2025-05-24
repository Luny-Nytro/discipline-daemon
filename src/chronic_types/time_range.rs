use serde::{Deserialize, Serialize};
use crate::{time, Duration, GenericError, Time};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum CreateFromNumbersError {
//   FromTooLarge,
//   TillTooLarge,
//   FromLaterThanTill,
//   DurationLongerThanSingleDay,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MakeWiderError {
  NewFromNarrowsTheRange,
  NewTillNarrowsTheRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MakeNarrowerError {
  NewFromWidensTheRange,
  NewTillWidensTheRange,
}

const MILLISECONDS_PER_DAY: u32 = 1000 * 60 * 60 * 24;
const FROM_MAX_VALUE: u32 = time::MAX_VALUE;
const TILL_MAX_VALUE: u32 = time::MAX_VALUE * 2;

// `TimeRange.till - TimeRange.from` may never be greater than
// `time::MAX_VALUE`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
  /// a number within this range: `time::MIN_VALUE ..= time::MAX_VALUE`.
  /// 
  /// Less than or equal to `TimeRange.till`.
  from: u32,
  /// a number within this range: `time::MIN_VALUE ..= (time::MAX_VALUE * 2)`.
  /// 
  /// Greater than or equal to `TimeRange.from`.
  till: u32,
}

impl TimeRange {
  pub fn new(from: Time, till: Time) -> TimeRange {
    let from = from.milliseconds_since_midnight();
    let till = till.milliseconds_since_midnight();

    if from <= till {
      Self { from, till }
    } else {
      Self { from, till: till + time::MAX_VALUE }
    }
  }

  pub fn from_numbers(from: u32, till: u32) -> Result<TimeRange, GenericError> {    
    if from > FROM_MAX_VALUE {
      return Err(
        GenericError::new("Faild to create TimeRange from raw numbers: 'from' is not in valid range")
          .add_attachment("valid range", format!("0 ..= {FROM_MAX_VALUE}"))
          .add_attachment("provided value", from.to_string())
      );
    }
    if till > TILL_MAX_VALUE {
      return Err(
        GenericError::new("Faild to create TimeRange from raw numbers: 'till' is not in valid range")
          .add_attachment("valid range", format!("0 ..= {TILL_MAX_VALUE}"))
          .add_attachment("provided value", from.to_string())
      );
    }
    if from > till {
      return Err(
        GenericError::new("Faild to create TimeRange from raw numbers: 'from' is later than 'till'")
          .add_attachment("provided 'from'", from.to_string())
          .add_attachment("provided 'till'", till.to_string())
      );
    }
    if till - from > MILLISECONDS_PER_DAY {
      return Err(
        GenericError::new("Faild to create TimeRange from raw numbers: duration between 'from' and 'till' is longer than one day")
          .add_attachment("provided 'from'", from.to_string())
          .add_attachment("provided 'till'", till.to_string())
      );
    }

    Ok(TimeRange { from, till })
  }

  pub fn as_numbers(&self) -> (u32, u32) {
    (self.from, self.till)
  }

  pub fn from(&self) -> Time {
    unsafe {
      Time::unchecked_timestamp(self.from)
    }
  }

  pub fn till(&self) -> Time {
    if self.till <= MILLISECONDS_PER_DAY {
      unsafe {
        Time::unchecked_timestamp(self.till)
      }
    } else {
      Time::wrapping_from_timestamp(self.till)
    }
  }

  pub fn is_intraday(&self) -> bool {
    self.till <= time::MAX_VALUE
  }
  
  pub fn is_crossday(&self) -> bool {
    self.till > time::MAX_VALUE
  }

  pub fn duration(&self) -> Duration {
    Duration::from_milliseconds((self.till - self.from) as u64)
  }

  pub fn contains(&self, time: Time) -> bool {
    let time = time.milliseconds_since_midnight();
    self.from <= time && time <= self.till
  }

  pub fn is_wider_than_or_equal_to(&self, other: &TimeRange) -> bool {
    self.from <= other.from
    &&
    self.till >= other.till 
  }
  
  pub fn is_wider_than(&self, other: &TimeRange) -> bool {
    self.from < other.from
    &&
    self.till > other.till 
  }
  
  pub fn is_narrower_than_or_equal_to(&self, other: &TimeRange) -> bool {
    self.from >= other.from
    &&
    self.till <= other.till 
  }

  pub fn is_narrower_than(&self, other: &TimeRange) -> bool {
    self.from > other.from
    &&
    self.till < other.till 
  }

  pub fn make_wider_or_err(&mut self, new_from: Time, new_till: Time) -> Result<(), MakeWiderError> {
    let new_from_as_timestamp = new_from.milliseconds_since_midnight();
    let new_till_as_timestamp = if new_from <= new_till {
      new_till.milliseconds_since_midnight()
    } else {
      new_till.milliseconds_since_midnight() + time::MAX_VALUE
    };

    // if `new_from` is later than `self.from`.
    if new_from_as_timestamp > self.from {
      return Err(MakeWiderError::NewFromNarrowsTheRange);
    }
    // if `new_till` is earlier than `self.till`.
    if new_till_as_timestamp < self.till {
      return Err(MakeWiderError::NewTillNarrowsTheRange);
    }

    self.from = new_from_as_timestamp;
    self.till = new_till_as_timestamp;

    Ok(())
  }

  pub fn wider_or_err(&self, new_from: Time, new_till: Time) -> Result<TimeRange, MakeWiderError> {
    let mut clone = self.clone();
    clone.make_wider_or_err(new_from, new_till)?;
    Ok(clone)
  }

  pub fn make_narrower_or_err(&mut self, new_from: Time, new_till: Time) -> Result<(), MakeNarrowerError> {
    let new_from_as_timestamp = new_from.milliseconds_since_midnight();
    let new_till_as_timestamp = if new_from <= new_till {
      new_till.milliseconds_since_midnight()
    } else {
      new_till.milliseconds_since_midnight() + time::MAX_VALUE
    };

    // if `new_from` is eariler than `self.from`.
    if new_from_as_timestamp < self.from {
      return Err(MakeNarrowerError::NewFromWidensTheRange);
    }
    
    // if `new_till` is later than `self.till`.
    if new_till_as_timestamp > self.till {
      return Err(MakeNarrowerError::NewTillWidensTheRange);
    }

    self.from = new_from_as_timestamp;
    self.till = new_till_as_timestamp;

    Ok(())
  }

  pub fn narrower_or_err(&self, new_from: Time, new_till: Time) -> Result<TimeRange, MakeNarrowerError> {
    let mut clone = self.clone();
    clone.make_narrower_or_err(new_from, new_till)?;
    Ok(clone)
  }
}

pub mod database {
  use crate::database::{Column, ColumnNamesapce, CompoundValueSerializer, CompoundValueDeserializer, DeserializeContext, SerializeContext, UpdateStatement};
  use crate::{GenericError, Time};
  use super::TimeRange;

  pub struct Schema {
    from: Column,
    till: Column,
  }

  impl Schema {
    pub fn new(column_namespace: ColumnNamesapce) -> Result<Self, GenericError> {
      Ok(Self {
        from: column_namespace
          .create_column_builder("from")
          .build()?,

        till: column_namespace
          .create_column_builder("till")
          .build()?,
      })
    }

    pub fn columns(&self) -> Vec<&Column> {
      vec![&self.from, &self.till]
    }
  }

  impl Schema {
    pub fn set_from(
      &self, 
      statement: &mut UpdateStatement,
      new_value: &Time,
    ) {
      statement.set(&self.from, new_value);
    }

    pub fn set_till(
      &self, 
      statement: &mut UpdateStatement,
      new_value: &Time,
    ) {
      statement.set(&self.from, new_value);
    }

    pub fn set_range(
      &self, 
      statement: &mut UpdateStatement,
      new_value: &TimeRange,
    ) {
      statement.set(&self.from, &new_value.from);
      statement.set(&self.till, &new_value.till);
    }
  }

  impl<'a> CompoundValueSerializer for Schema {
    type Input = TimeRange;

    fn serialize_into(
      &self, 
      value: &Self::Input,
      context: &mut SerializeContext, 
    ) {
      context.serializable_scalar(&self.from, &value.from);  
      context.serializable_scalar(&self.till, &value.till);  
    }
  }

  impl<'a> CompoundValueDeserializer for Schema {
    type Output = TimeRange;

    fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
      let from = context.deserializable_scalar(&self.from).map_err(|error| 
        error.change_context("Faild to deserialize TimeRange: Failed to deserialize 'from' field")
      )?;

      let till = context.deserializable_scalar(&self.till).map_err(|error| 
        error.change_context("Faild to deserialize TimeRange: Failed to deserialize 'till' field")
      )?;

      TimeRange::from_numbers(from, till).map_err(|error|
        error.change_context("Failed to deserialize TimeRange: Fields deserialized successfully, but some invariants are invlidated")
      )
    }
  }
}