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
        GenericError::new("creating a TimeRange from raw numbers")
          .add_error(format!("'from' must be in this range 0 ..= {FROM_MAX_VALUE}"))
          .add_attachment("'from'", from.to_string())
      );
    }
    if till > TILL_MAX_VALUE {
      return Err(
        GenericError::new("creating a TimeRange from raw numbers")
          .add_error(format!("'till' must be in this range 0 ..= {TILL_MAX_VALUE}"))
          .add_attachment("'till'", till.to_string())
      );
    }
    if from > till {
      return Err(
        GenericError::new("creating a TimeRange from raw numbers")
          .add_error("'from' is later than 'till'")
          .add_attachment("from", from.to_string())
          .add_attachment("till", till.to_string())
      );
    }
    if till - from > MILLISECONDS_PER_DAY {
      return Err(
        GenericError::new("creating a TimeRange from raw numbers")
          .add_error("duration between 'from' and 'till' is longer than one day")
          .add_attachment("from", from.to_string())
          .add_attachment("till", till.to_string())
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
  use crate::database::*;
  use crate::{GenericError, Time};
  use super::TimeRange;

  pub struct Specification {
    from: ScalarFieldSpecification,
    till: ScalarFieldSpecification,
  }

  impl Specification {
    pub fn new(namespace: &mut CompoundTypeFieldsScope) -> Result<Self, GenericError> {
      Ok(Self {
        from: namespace
          .scalar_field_specification("from")
          .build()?,

        till: namespace
          .scalar_field_specification("till")
          .build()?,
      })
    }
  }

  impl Specification {
    pub fn set_from(
      &self, 
      modifications: &mut CollectionItemModifications,
      new_value: &Time,
    ) -> 
      Result<(), GenericError> 
    {
      modifications.modify_scalar_field(&self.from, new_value)
    }

    pub fn set_till(
      &self, 
      modifications: &mut CollectionItemModifications,
      new_value: &Time,
    ) -> 
      Result<(), GenericError>
    {
      modifications.modify_scalar_field(&self.from, new_value)
    }

    pub fn set_range(
      &self, 
      modifications: &mut CollectionItemModifications,
      new_value: &TimeRange,
    ) -> 
      Result<(), GenericError>
    {
      modifications.modify_scalar_field(&self.from, &new_value.from)?;
      modifications.modify_scalar_field(&self.till, &new_value.till)
    }
  }

  impl<'a> CompoundValueSerializer for Specification {
    type CompoundValue = TimeRange;

    fn serialize_into(
      &self, 
      value: &Self::CompoundValue,
      context: &mut CompoundValueSerializerContext, 
    ) -> 
      Result<(), GenericError>
    {
      context.serializable_scalar(&self.from, &value.from)?; 
      context.serializable_scalar(&self.till, &value.till) 
    }
  }

  impl<'a> CompoundValueDeserializer for Specification {
    type Output = TimeRange;

    fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
      let from = context.deserializable_scalar(&self.from).map_err(|error| 
        error
          .change_context("deserialize the 'from' field")
          .change_context("deserialize a TimeRange")
      )?;

      let till = context.deserializable_scalar(&self.till).map_err(|error| 
        error
          .change_context("deserialize the 'till' field")
          .change_context("deserialize a TimeRange")
      )?;

      TimeRange::from_numbers(from, till).map_err(|error|
        error.change_context("deserialize a TimeRange")
      )
    }
  }

  impl CompoundTypeSpecificationProvider for Specification {
    fn add_fields(&self, context: &mut CompoundTypeFieldsSpecification) -> Result<(), GenericError> {
      context.add_scalar_field(&self.from)?;
      context.add_scalar_field(&self.till)
    }
  }
}