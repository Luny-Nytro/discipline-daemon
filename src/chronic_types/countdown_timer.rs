use serde::{Deserialize, Serialize};
use crate::{Duration, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownTimer {
  duration: Duration,
  remaining_duration: Duration,
  previous_synchronization_time: DateTime,
}

impl CountdownTimer {
  pub fn new(duration: Duration, now: DateTime) -> Self {
    Self {
      duration,
      remaining_duration: duration,
      previous_synchronization_time: now,
    }
  }

  pub fn is_running(&self) -> bool {
    self.remaining_duration > Duration::ZERO
  }

  pub fn is_finished(&self) -> bool {
    self.remaining_duration == Duration::ZERO
  }

  pub fn reinitialize(&mut self) {
    self.remaining_duration = self.duration;
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn remaining_duration(&self) -> Duration {
    self.remaining_duration
  }

  pub fn set_remaining_duration(&mut self, new_value: Duration) {
    self.remaining_duration = new_value;
  }

  pub fn synchronize(&mut self, now: DateTime) {
    let interval = now
      .since_or_zero(&self.previous_synchronization_time);

    if let Some(remaining_duration) = self.remaining_duration.checked_sub(&interval) {
      self.remaining_duration = remaining_duration;
      self.previous_synchronization_time = now;
    }
  }

}

// impl TimeSynchronizeListener for CountdownTimer {
//   fn on_time_synchronize(&mut self, event: &SynchronizeContext) {
//     if let Some(duration) = self.remaining_duration.checked_sub(event.interval()) {
//       self.remaining_duration = duration;
//     }
//   }
// }

pub mod database {
  use crate::database::*;
  use crate::{Duration, GenericError};
  use super::CountdownTimer;

  pub struct Specification {
    duration: ScalarFieldSpecification,
    remaining_duration: ScalarFieldSpecification,
    previous_synchronization_time: ScalarFieldSpecification,
  }

  impl Specification {
    pub fn new(scope: &mut CompoundTypeFieldsScope) -> Result<Self, GenericError> {
      Ok(Self {
        duration: scope
          .scalar_field_specification("Duraion")
          .build()?,

        remaining_duration: scope
          .scalar_field_specification("RemainingDuration")
          .build()?,

        previous_synchronization_time: scope
          .scalar_field_specification("PreviousSynchronizationTime")
          .build()?,
      })
    }
    
    pub fn update_remaining_duration(
      &self,
      modifications: &mut CollectionItemModificationsDraft,
      new_remaining_duration: &Duration,
    ) -> 
      Result<(), GenericError>
    {
      modifications.modify_scalar_field(&self.remaining_duration, new_remaining_duration)
    }

    pub fn update_after_synchronization(
      &self,
      modifications: &mut CollectionItemModificationsDraft,
      countdown_timer: &CountdownTimer,
    ) -> 
      Result<(), GenericError>
    {
      modifications.modify_scalar_field(
        &self.remaining_duration, 
        &countdown_timer.remaining_duration,
      )?;

      modifications.modify_scalar_field(
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
    type Output = CountdownTimer;

    fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
      Ok(CountdownTimer {
        duration: context.deserializable_scalar(&self.duration).map_err(|error|
          error
            .change_context("deserializing the 'Duration' column")
            .change_context("deserializing a CountdownTimer")
        )?,
        remaining_duration: context.deserializable_scalar(&self.remaining_duration).map_err(|error|
          error
            .change_context("deserializing the 'RemainingDuration' column")
            .change_context("deserializing a CountdownTimer")
        )?,
        previous_synchronization_time: context.deserializable_scalar(&self.previous_synchronization_time).map_err(|error|
          error
            .change_context("deserializing the 'PreviousSynchronizationTime' column")
            .change_context("deserializing a CountdownTimer")
        )?,
      })
    }
  }
}