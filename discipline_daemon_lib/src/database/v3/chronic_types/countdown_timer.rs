use crate::database::*;
use crate::{Duration, GenericError};
use super::CountdownTimer;

// provide_database_integration_for_scalar_value(Weekday, to_number, from_number);

pub trait Lunar {

}

pub trait IsScalarValue {
  fn display_name(&self) -> &str;
  fn to_scalar_value(&self) -> &impl Lunar;
  fn from_scalar_value();
}

pub trait IsCompoundType {
  fn serialize(&self, context: &mut SerializeCompoundTypeContext);
  fn deserialize(&self, context: &mut DeserializeCompoundTypeContext);
}

macro_rules! provide_database_integration_for_scalar_value {
  (
    $scalar_value_type: ident, 
    $to_scalar_value: ident, 
    $from_scalar_value: ident,
  ) => {
    impl crate::database::SerializableScalarValue for $scalar_value_type {
      fn serialize(&self, code: &mut String) {
        self.$to_scalar_value().serialize_into(code);
      }
    }

    impl crate::database::DeserializableScalarValue for $scalar_value_type {
      fn deserialize(scalar_value: ScalarValue) -> Result<Self, GenericError> {
        scalar_value.try_into().and_then()
      }
    }
  };
}

// implement_serializable_scalar_type_for!(Moon, timer);

fn serialize_screen_access_policy_enabled_countdown_timer() {

}

pub struct Specification {
  duration: Field,
  remaining_duration: Field,
  previous_synchronization_time: Field,
}

impl IsCompoundType for Specification {
  fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      duration: definer.writable_required_field("Duraion")?,
      remaining_duration: definer.writable_required_field("RemainingDuration")?,
      previous_synchronization_time: definer.writable_required_field("PreviousSynchronizationTime")?,
    })
  }

  fn display_name(&self) -> &str {
    "CountdownTimer"
  }
}

impl Specification {
  pub fn write_remaining_duration(
    &self,
    changes: &mut CollectionItemModificationsDraft,
    new_remaining_duration: &Duration,
  ) -> 
    Result<(), GenericError>
  {
    changes.write_scalar_field(&self.remaining_duration, new_remaining_duration)
  }

  pub fn set_after_synchronization(
    &self,
    draft: &mut CollectionItemModificationsDraft,
    countdown_timer: &CountdownTimer,
  ) -> 
    Result<(), GenericError>
  {
    draft.write_scalar_field(
      &self.remaining_duration, 
      &countdown_timer.remaining_duration,
    )?;

    draft.write_scalar_field(
      &self.previous_synchronization_time, 
      &countdown_timer.previous_synchronization_time,
    )
  }
}

impl CompoundValueSerializer for Specification {
  type CompoundValue = CountdownTimer;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.duration, &value.duration)?;
    context.serializable_scalar(&self.remaining_duration, &value.remaining_duration)?;
    context.serializable_scalar(&self.previous_synchronization_time, &value.previous_synchronization_time)
  }
}

impl CompoundValueDeserializer for Specification {
  type CompoundValue = CountdownTimer;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
    Ok(CountdownTimer {
      duration: context.deserializable_scalar(&self.duration)?,
      remaining_duration: context.deserializable_scalar(&self.remaining_duration)?,
      previous_synchronization_time: context.deserializable_scalar(&self.previous_synchronization_time)?,
    })
  }
}