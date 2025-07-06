use serde::{Deserialize, Serialize};
use crate::{Duration, GenericError, Weekday};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateFromNumbersError {
  FromTooLarge,
  TillTooLarge,
  FromLaterThanTill,
  DurationLongerThanSingleWeek,
}

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

const FROM_MAX_VALUE: u32 = 6;
const TILL_MAX_VALUE: u32 = 13;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekdayRange {
  // from 0 .. 6, inclusive both.
  from: u32,
  // from 0 .. 13, inclusive both.
  till: u32,
}

impl WeekdayRange {
  pub fn new(from: Weekday, till: Weekday) -> WeekdayRange {
    let from = from.days_since_sunday();
    let till = till.days_since_sunday();
    if from <= till {
      Self { from, till }
    } else {
      Self { from, till: till + 7 }
    }
  }

  pub fn from_numbers(from: u32, till: u32) -> Result<WeekdayRange, GenericError> {    
    if from > FROM_MAX_VALUE {
      return Err(GenericError::new("Failed to create WeekdayRange: 'from' is not in valid range")
        .add_attachment("from", from.to_string())
        .add_attachment("valid range", format!("0 ..= {FROM_MAX_VALUE}"))
      );
    }
    if till > TILL_MAX_VALUE {
      return Err(GenericError::new("Failed to create WeekdayRange: 'till' is not in valid range")
        .add_attachment("till", till.to_string())
        .add_attachment("valid range", format!("0 ..= {TILL_MAX_VALUE}"))
      );
    }
    if from > till {
      return Err(GenericError::new("Failed to create WeekdayRange: 'from' is later than 'till'")
        .add_attachment("from", from.to_string())
        .add_attachment("till", till.to_string())
      );
    }
    if till - from > 7 {
      return Err(GenericError::new("Failed to create WeekdayRange: Duration between 'from' and 'till' is longer than one week")
        .add_attachment("from", from.to_string())
        .add_attachment("till", till.to_string())
      );
    }
    Ok(WeekdayRange { from, till })
  }

  // TODO: Rename to "unpack"
  pub fn as_numbers(&self) -> (u32, u32) {
    (self.from, self.till)
  }

  pub fn from(&self) -> Weekday {
    unsafe {
      // SAFETY: "self.from" is in valid range 0 .. 6.
      Weekday::unchekced_from_number(self.from)
    }
  }

  pub fn till(&self) -> Weekday {
    Weekday::from_number_wrapping(self.till)
  }

  pub fn duration(&self) -> Duration {
    Duration::unchecked_from_days_u32(self.till - self.from)
  }
  
  pub fn is_intra_week(&self) -> bool {
    self.till <= FROM_MAX_VALUE
  }
  
  pub fn is_cross_week(&self) -> bool {
    self.till > FROM_MAX_VALUE
  }

  pub fn contains_weekday(&self, weekday: Weekday) -> bool {
    let weekday = weekday.days_since_sunday();
    weekday >= self.from && weekday <= self.till
  }

  /// Returns true if this range contains the `other` range.
  pub fn is_wider_than_or_equal_to(&self, other: &WeekdayRange) -> bool {
    self.from <= other.from 
    && 
    self.till >= other.till
  }
  
  pub fn is_wider_than(&self, other: &WeekdayRange) -> bool {
    self.from < other.from 
    && 
    self.till > other.till
  }

  pub fn is_narrower_than_or_equal_to(&self, other: &WeekdayRange) -> bool {
    self.from >= other.from 
    && 
    self.till <= other.till
  }

  pub fn is_narrower_than(&self, other: &WeekdayRange) -> bool {
    self.from > other.from 
    && 
    self.till < other.till
  }

  pub fn make_wider_or_err(&mut self, new_from: Weekday, new_till: Weekday) -> Result<(), MakeWiderError> {
    let new_from_as_timestamp = new_from.days_since_sunday();
    let new_till_as_timestamp = if new_from <= new_till {
      new_till.days_since_sunday()
    } else {
      new_till.days_since_sunday() + 7
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

  pub fn wider_or_err(&self, new_from: Weekday, new_till: Weekday) -> Result<WeekdayRange, MakeWiderError> {
    let mut clone = self.clone();
    clone.make_wider_or_err(new_from, new_till)?;
    Ok(clone)
  }

  pub fn make_narrower_or_err(&mut self, new_from: Weekday, new_till: Weekday) -> Result<(), MakeNarrowerError> {
    let new_from_as_timestamp = new_from.days_since_sunday();
    let new_till_as_timestamp = if new_from <= new_till {
      new_till.days_since_sunday()
    } else {
      new_till.days_since_sunday() + 7
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

  pub fn narrower_or_err(&self, new_from: Weekday, new_till: Weekday) -> Result<WeekdayRange, MakeNarrowerError> {
    let mut clone = self.clone();
    clone.make_narrower_or_err(new_from, new_till)?;
    Ok(clone)
  }
}

pub mod database {
  use crate::{database::*, Weekday};
  use super::WeekdayRange;
  use crate::GenericError;

  pub struct Specification {
    from: Field,
    till: Field,
  }

  impl IsCompoundType for Specification {
    fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
      Ok(Self {
        from: definer.writable_required_field("From")?,
        till: definer.writable_required_field("Till")?,
      })
    }

    fn display_name(&self) -> &str {
      "WeekdayRange"
    }
  }

  impl Specification {
    pub fn set_from(
      &self,
      changes: &mut CollectionItemModificationsDraft,
      new_value: &Weekday
    ) ->
      Result<(), GenericError>
    {
      changes.write_scalar_field(&self.from, new_value)
    }

    pub fn set_till(
      &self,
      changes: &mut CollectionItemModificationsDraft,
      new_value: &Weekday
    ) ->
      Result<(), GenericError>
    {
      changes.write_scalar_field(&self.till, new_value)
    }

    pub fn change_range(
      &self,
      changes: &mut CollectionItemModificationsDraft,
      new_value: &WeekdayRange
    ) ->
      Result<(), GenericError>
    {
      changes.write_scalar_field(&self.from, &new_value.from)?;
      changes.write_scalar_field(&self.till, &new_value.till)
    }
  }

  impl CompoundValueSerializer for Specification {
    type CompoundValue = WeekdayRange;

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

  impl CompoundValueDeserializer for Specification {
    type CompoundValue = WeekdayRange;

    fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
      let from = context.deserializable_scalar(&self.from).map_err(|error| 
        error
          .change_context("deserializing the the 'from' field")
          .change_context("deserislizing a WeekdayRange")
      )?;

      let till = context.deserializable_scalar(&self.till).map_err(|error|
        error
          .change_context("deserializing the 'till' field")
          .change_context("deserislizing a WeekdayRange")
      )?;

      WeekdayRange::from_numbers(from, till).map_err(|error|
        error.change_context("deserislizing a WeekdayRange")
      )
    }
  }
}