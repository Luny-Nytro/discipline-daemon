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

  pub fn change_remaining_duration(&mut self, new_value: Duration) {
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

pub mod database_serde {
  use crate::database::{
    Column, ColumnNamespace, CompoundValueSerializer, 
    CompoundValueDeserializer, DeserializeContext, 
    SerializeContext, UpdateStatement,
    WriteColumns, WriteColumnsContext,
  };

  use crate::{Duration, GenericError};
  use super::CountdownTimer;

  pub struct Schema {
    duration: Column,
    remaining_duration: Column,
    previous_synchronization_time: Column,
  }

  impl Schema {
    pub fn new(column_namespace: ColumnNamespace) -> Result<Self, GenericError> {
      Ok(Self {
        duration: column_namespace
          .create_column_builder("duration")
          .build()?,

        remaining_duration: column_namespace
          .create_column_builder("remaining_duration")
          .build()?,

        previous_synchronization_time: column_namespace
          .create_column_builder("previous_synchronization_time")
          .build()?,
      })
    }
    
    pub fn set_remaining_duration(
      &self,
      updater: &mut UpdateStatement,
      new_remaining_duration: &Duration,
    ) {
      updater.set(&self.remaining_duration, new_remaining_duration);
    }

    pub fn update_after_synchronize(
      &self,
      updater: &mut UpdateStatement,
      countdown_timer: &CountdownTimer,
    ) {
      updater.update_column(
        &self.remaining_duration, 
        &countdown_timer.remaining_duration,
      );

      updater.update_column(
        &self.previous_synchronization_time, 
        &countdown_timer.previous_synchronization_time,
      );
    }
  }

  impl CompoundValueSerializer for Schema {
    type Input = CountdownTimer;

    fn serialize_into(
      &self, 
      value: &CountdownTimer,
      context: &mut SerializeContext, 
    ) {
      context.serializable_scalar(&self.duration, &value.duration);
      context.serializable_scalar(&self.remaining_duration, &value.remaining_duration);
      context.serializable_scalar(&self.previous_synchronization_time, &value.previous_synchronization_time);
    }
  }

  impl CompoundValueDeserializer for Schema {
    type Output = CountdownTimer;

    fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
      Ok(CountdownTimer {
        duration: context.deserializable_scalar(&self.duration).map_err(|error|
          error.change_context("Failed to deserialize CountdownTimer: Failed to deserialize the 'duration' column")
        )?,
        remaining_duration: context.deserializable_scalar(&self.remaining_duration).map_err(|error|
          error.change_context("Failed to deserialize CountdownTimer: Failed to deserialize the 'remaining_duration' column")
        )?,
        previous_synchronization_time: context.deserializable_scalar(&self.previous_synchronization_time).map_err(|error|
          error.change_context("Failed to deserialize CountdownTimer: Failed to deserialize the 'previous_synchronization_time' column")
        )?,
      })
    }
  }

  impl WriteColumns for Schema {
    fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
      context.write_scalar_type(&self.duration)?;
      context.write_scalar_type(&self.remaining_duration)?;
      context.write_scalar_type(&self.previous_synchronization_time)?;
      Ok(())
    }
  }
}