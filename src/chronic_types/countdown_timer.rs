use serde::{Deserialize, Serialize};
use crate::{Duration, DateTime, SynchronizeContext};

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
  use crate::database::{Column, ColumnNamesapce, CompoundValueSerializer, CompoundValueDeserializer, DeserializeContext, SerializeContext, UpdateStatementSetClause};
  use crate::{Duration, GenericError};
  use super::CountdownTimer;

  pub struct Adapter {
    duration: Column,
    remaining_duration: Column,
    previous_synchronization_time: Column,
  }

  impl Adapter {
    pub fn new(column_namespace: ColumnNamesapce) -> Result<Self, GenericError> {
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

    pub fn columns(&self) -> Vec<&Column> {
      vec![
        &self.duration, 
        &self.remaining_duration, 
        &self.previous_synchronization_time
      ]
    }

    pub fn columns_iterator(&self) -> impl Iterator<Item = &Column> {
      [
        &self.duration, 
        &self.remaining_duration,
        &self.previous_synchronization_time,
      ].into_iter()
    }

    pub fn update_remaining_duration(
      &self,
      update_statement_set_clause: &mut UpdateStatementSetClause,
      new_remaining_duration: &Duration,
    ) -> 
      Result<(), GenericError>
    {
      update_statement_set_clause.update_column(
        &self.remaining_duration, 
        new_remaining_duration
      )
    }

    pub fn update_after_synchronize(
      &self,
      update_statement_set_clause: &mut UpdateStatementSetClause,
      countdown_timer: &CountdownTimer,
    ) -> 
      Result<(), GenericError>
    {
      update_statement_set_clause.update_column(
        &self.remaining_duration, 
        &countdown_timer.remaining_duration,
      )?;

      update_statement_set_clause.update_column(
        &self.previous_synchronization_time, 
        &countdown_timer.previous_synchronization_time,
      )?;

      Ok(())
    }
  }

  impl CompoundValueSerializer for Adapter {
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

  impl CompoundValueDeserializer for Adapter {
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
}