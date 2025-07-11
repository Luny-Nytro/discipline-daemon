use super::*;
use crate::chronic::*;

impl SerializableScalarValue for Weekday {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    match self {
      Sunday    => context.write_u8(0),
      Monday    => context.write_u8(1),
      Tuesday   => context.write_u8(2),
      Wednesday => context.write_u8(3),
      Thursday  => context.write_u8(4),
      Friday    => context.write_u8(5),
      Saturday  => context.write_u8(6),
    }
  }
}

impl DeserializableScalarValue for Weekday {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    let number = value
      .as_u8()
      .map_err(|error| 
        error.change_context("deserializing a Weekday")
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
          GenericError::new("deserializing a Weekday")
            .add_error("scalar value is an integer but it's outside the valid range 0 ..= 6")
            .add_attachment("scalar value", number.to_string())
        )
      }
    }
  }
}  

impl SerializableScalarValue for Time {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    self.milliseconds_since_midnight().serialize(context);
  }
}

impl DeserializableScalarValue for Time {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_u32()
      .and_then(Time::try_from_timestamp_or_generic_error)
  }
}

impl SerializableScalarValue for DateTime {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    self.timestamp().serialize(context);
  }
}

impl DeserializableScalarValue for DateTime {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_i64()
      .and_then(DateTime::from_timestamp_or_generic_error)
  }
}

impl SerializableScalarValue for Duration {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    self.total_milliseconds().serialize(context);
  }
}

impl DeserializableScalarValue for Duration {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_u64()
      .map(Duration::from_milliseconds)
  }
}

impl SerializableScalarValue for Minute {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    self.value0().serialize(context);
  }
}

impl DeserializableScalarValue for Minute {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_u32()
      .and_then(Minute::from_0_or_generic_error)
      .map_err(|error|
        error.change_context("deserializing a minute")
      )
  }
}

impl SerializableScalarValue for Hour {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    self.value().serialize(context);
  }
}

impl DeserializableScalarValue for Hour {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_u32()
      .and_then(Hour::from_0_or_generic_error)
  }
}

// pub struct WeekdayRangeSerializer<'a> {
//   from_field: &'a String,
//   till_field: &'a String,
// }

// impl<'a> WeekdayRangeSerializer<'a> {
//   pub fn new(
//     from_field: &'a String,
//     till_field: &'a String,
//   ) -> Self {
//     Self {
//       from_field,
//       till_field,
//     }
//   }
// }

// impl<'a> CompoundValueSerializer for WeekdayRangeSerializer<'a> {
//   type CompoundValue = WeekdayRange;

//   fn serialize(
//     &self, 
//     value: &Self::CompoundValue,
//     context: &mut SerializeCompoundValueContext, 
//   ) {
//     context.write_scalar(self.from_field, &value.from());
//     context.write_scalar(self.till_field, &value.till());
//   }
// }

// pub struct WeekdayRangeIntegration {}

// impl WeekdayRangeIntegration {
//   pub fn write_full_update(
//     updates: &mut CollectionItemUpdateDraft,
//     new_value: &WeekdayRange,
//     from_field: &String,
//     till_field: &String,
//   ) {
//     let (from, till) = new_value.as_numbers();

//     updates.write_scalar(from_field, &from);
//     updates.write_scalar(till_field, &till);
//   }
// }

// pub fn serialize_weekday_range(
//   context: &mut SerializeCompoundValueContext,
//   value: &WeekdayRange,
//   from_field: &String,
//   till_field: &String,
// ) {
//   let (from, till) = value.as_numbers();
//   context.write_scalar(from_field, &from);
//   context.write_scalar(till_field, &till);
// }

// pub fn deserialize_weekday_range(
//   context: &mut DeserializeCompoundValueContext,
//   from: &String,
//   till: &String,
// ) 
//   -> Result<WeekdayRange, GenericError>
// {
//   WeekdayRange::from_timestamps(
//     context.deserializable_scalar(from)?,
//     context.deserializable_scalar(till)?,
//   )
// }

// pub struct TimeRangeSerializer<'a> {
//   from_field: &'a String,
//   till_field: &'a String,
// }

// impl<'a> TimeRangeSerializer<'a> {
//   pub fn new(
//     from_field: &'a String,
//     till_field: &'a String,
//   ) -> Self {
//     Self {
//       from_field,
//       till_field,
//     }
//   }
// }

// impl<'a> CompoundValueSerializer for TimeRangeSerializer<'a> {
//   type CompoundValue = TimeRange;

//   fn serialize(
//     &self, 
//     value: &Self::CompoundValue,
//     context: &mut SerializeCompoundValueContext, 
//   ) {
//     context.write_scalar(self.from_field, &value.from());
//     context.write_scalar(self.till_field, &value.till());
//   }
// }

// pub struct TimeRangeDeserializer<'a> {
//   from_field_identifier: &'a String,
//   till_field_identifier: &'a String,
// }

// impl<'a> TimeRangeDeserializer<'a> {
//   pub fn new(
//     from_field_identifier: &'a String,
//     till_field_identifier: &'a String,
//   ) -> Self {
//     Self {
//       from_field_identifier,
//       till_field_identifier,
//     }
//   }
// }

// impl<'a> CompoundValueDeserializer for TimeRangeDeserializer<'a> {
//   type CompoundValue = TimeRange;

//   fn deserialize(
//     &self, 
//     context: &DeserializeCompoundValueContext,
//   ) -> Result<Self::CompoundValue, GenericError> {
//     TimeRange::from_timestamps(
//       context.deserializable_scalar(self.from_field_identifier)?, 
//       context.deserializable_scalar(self.till_field_identifier)?,
//     )
//   }
// }

// pub struct TimeRangeIntegration {}


// pub struct TimeRangeFields<'a> {
//   pub from: &'a String,
//   pub till: &'a String,
// }

// impl TimeRangeIntegration {
//   pub fn write_full_update_given_fields(
//     updates: &mut CollectionItemUpdateDraft,
//     fields: TimeRangeFields,
//     new_value: &TimeRange,
//   ) {
//     let (from, till) = new_value.as_numbers();

//     updates.write_scalar(fields.from, &from);
//     updates.write_scalar(fields.till, &till);
//   }

//   pub fn write_full_update(
//     updates: &mut CollectionItemUpdateDraft,
//     new_value: &TimeRange,
//     from_field: &String,
//     till_field: &String,
//   ) {
//     let (from, till) = new_value.as_numbers();

//     updates.write_scalar(from_field, &from);
//     updates.write_scalar(till_field, &till);
//   }
// }

// pub fn serialize_time_range(
//   context: &mut SerializeCompoundValueContext,
//   value: &TimeRange,
//   from_field: &String,
//   till_field: &String,
// ) {
//   let (from, till) = value.as_numbers();
//   context.write_scalar(from_field, &from);
//   context.write_scalar(till_field, &till);
// }

// pub fn deserialize_time_range(
//   context: &mut DeserializeCompoundValueContext,
//   from: &String,
//   till: &String,
// ) 
//   -> Result<TimeRange, GenericError>
// {
//   TimeRange::from_timestamps(
//     context.deserializable_scalar(from)?,
//     context.deserializable_scalar(till)?,
//   )
// }

// pub struct CountdownTimerSerializer<'a> {
//   duration_field_identifier: &'a String,
//   remaining_duration_field_identifier: &'a String,
//   previous_synchronization_time_field_identifier: &'a String,
// }

// impl<'a> CountdownTimerSerializer<'a> {
//   pub fn new(
//     duration_field_identifier: &'a String,
//     remaining_duration_field_identifier: &'a String,
//     previous_synchronization_time_field_identifier: &'a String,
//   ) -> Self {
//     Self {
//       duration_field_identifier,
//       remaining_duration_field_identifier,
//       previous_synchronization_time_field_identifier,
//     }
//   }
// }

// impl<'a> CompoundValueSerializer for CountdownTimerSerializer<'a> {
//   type CompoundValue = CountdownTimer;

//   fn serialize(
//     &self, 
//     value: &Self::CompoundValue,
//     context: &mut SerializeCompoundValueContext, 
//   ) {
//     context.write_scalar(self.duration_field_identifier, &value.duration());
//     context.write_scalar(self.remaining_duration_field_identifier, &value.remaining_duration());
//     context.write_scalar(self.previous_synchronization_time_field_identifier, &value.previous_synchronization_time());
//   }
// }

// pub struct CountdownTimerDeserializer<'a> {
//   duration_field_identifier: &'a String,
//   remaining_duration_field_identifier: &'a String,
//   previous_synchronization_time_field_identifier: &'a String,
// }

// impl<'a> CountdownTimerDeserializer<'a> {
//   pub fn new(
//     duration_field_identifier: &'a String,
//     remaining_duration_field_identifier: &'a String,
//     previous_synchronization_time_field_identifier: &'a String,
//   ) -> Self {
//     Self {
//       duration_field_identifier,
//       remaining_duration_field_identifier,
//       previous_synchronization_time_field_identifier,
//     }
//   }
// }

// impl<'a> CompoundValueDeserializer for CountdownTimerDeserializer<'a> {
//   type CompoundValue = CountdownTimer;

//   fn deserialize(
//     &self, 
//     context: &DeserializeCompoundValueContext,
//   ) -> Result<Self::CompoundValue, GenericError> {
//     Ok(CountdownTimer::new_with_state(
//       context.deserializable_scalar(self.duration_field_identifier)?, 
//       context.deserializable_scalar(self.remaining_duration_field_identifier)?, 
//       context.deserializable_scalar(self.previous_synchronization_time_field_identifier)?,
//     ))
//   }
// }

// pub struct CountdownTimerIntegration;

// impl CountdownTimerIntegration {
//   pub fn write_full_update(
//     updates: &mut CollectionItemUpdateDraft,
//     new_value: &CountdownTimer,
//     duration_field: &String,
//     remaining_duration_field: &String,
//     previous_synchronization_time_field: &String,
//   ) {
//     updates.write_scalar(duration_field, &new_value.duration());
//     updates.write_scalar(remaining_duration_field, &new_value.remaining_duration());
//     updates.write_scalar(previous_synchronization_time_field, &new_value.previous_synchronization_time());
//   }
  
//   pub fn write_duration_update(
//     updates: &mut CollectionItemUpdateDraft,
//     new_value: &Duration,
//     duration_field: &String,
//     remaining_duration_field: &String,
//     previous_synchronization_time_field: &String,
//   ) {
//     updates.write_scalar(duration_field, new_value);
//   }
  
//   pub fn write_remaining_duration_update(
//     updates: &mut CollectionItemUpdateDraft,
//     new_value: &Duration,
//     duration_field: &String,
//     remaining_duration_field: &String,
//     previous_synchronization_time_field: &String,
//   ) {
//     updates.write_scalar(remaining_duration_field, new_value);
//   }
  
//   pub fn write_previous_synchrinization_time_update(
//     updates: &mut CollectionItemUpdateDraft,
//     new_value: &DateTime,
//     duration_field: &String,
//     remaining_duration_field: &String,
//     previous_synchronization_time_field: &String,
//   ) {
//     updates.write_scalar(previous_synchronization_time_field, new_value);
//   }
// }

// pub fn serialize_countdown_timer(
//   context: &mut SerializeCompoundValueContext,
//   value: &CountdownTimer,
//   duration_field: &String,
//   remaining_duration_field: &String,
//   previous_synchronization_time_field: &String,
// ) {
//   context.write_scalar(duration_field, &value.duration());
//   context.write_scalar(remaining_duration_field, &value.remaining_duration());
//   context.write_scalar(previous_synchronization_time_field, &value.previous_synchronization_time());
// }

// pub fn deserialize_countdown_timer(
//   context: &mut DeserializeCompoundValueContext,
//   duration_field: &String,
//   remaining_duration_field: &String,
//   previous_synchronization_time_field: &String,
// )
//   -> Result<CountdownTimer, GenericError>
// {
//   Ok(CountdownTimer::new_with_state(
//     context.deserializable_scalar(duration_field)?, 
//     context.deserializable_scalar(remaining_duration_field)?, 
//     context.deserializable_scalar(previous_synchronization_time_field)?,
//   ))
// }