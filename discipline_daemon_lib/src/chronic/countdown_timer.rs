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

  pub fn pack(
    duration: Duration,
    remaining_duration: Duration,
    previous_synchronization_time: DateTime,
  ) -> Self {
    Self { 
      duration,
      remaining_duration,
      previous_synchronization_time,
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

  pub fn previous_synchronization_time(&self) -> DateTime {
    self.previous_synchronization_time
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

// mod database {
//   use crate::database::*;
//   use crate::{Duration, GenericError};
//   use super::CountdownTimer;

//   pub struct Specification {
//     duration: Field,
//     remaining_duration: Field,
//     previous_synchronization_time: Field,
//   }

//   impl IsCompoundType for Specification {
//     fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
//       Ok(Self {
//         duration: definer.writable_required_field("Duraion")?,
//         remaining_duration: definer.writable_required_field("RemainingDuration")?,
//         previous_synchronization_time: definer.writable_required_field("PreviousSynchronizationTime")?,
//       })
//     }

//     fn display_name(&self) -> &str {
//       "CountdownTimer"
//     }
//   }

//   impl Specification {
//     pub fn write_remaining_duration(
//       &self,
//       changes: &mut CollectionItemModificationsDraft,
//       new_remaining_duration: &Duration,
//     ) -> 
//       Result<(), GenericError>
//     {
//       changes.write_scalar_field(&self.remaining_duration, new_remaining_duration)
//     }

//     pub fn set_after_synchronization(
//       &self,
//       draft: &mut CollectionItemModificationsDraft,
//       countdown_timer: &CountdownTimer,
//     ) -> 
//       Result<(), GenericError>
//     {
//       draft.write_scalar_field(
//         &self.remaining_duration, 
//         &countdown_timer.remaining_duration,
//       )?;

//       draft.write_scalar_field(
//         &self.previous_synchronization_time, 
//         &countdown_timer.previous_synchronization_time,
//       )
//     }
//   }

//   impl CompoundValueSerializer for Specification {
//     type CompoundValue = CountdownTimer;

//     fn serialize_into(
//       &self, 
//       value: &Self::CompoundValue,
//       context: &mut CompoundValueSerializerContext, 
//     ) ->
//       Result<(), GenericError>
//     {
//       context.serializable_scalar(&self.duration, &value.duration)?;
//       context.serializable_scalar(&self.remaining_duration, &value.remaining_duration)?;
//       context.serializable_scalar(&self.previous_synchronization_time, &value.previous_synchronization_time)
//     }
//   }

//   impl CompoundValueDeserializer for Specification {
//     type CompoundValue = CountdownTimer;

//     fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
//       Ok(CountdownTimer {
//         duration: context.deserializable_scalar(&self.duration)?,
//         remaining_duration: context.deserializable_scalar(&self.remaining_duration)?,
//         previous_synchronization_time: context.deserializable_scalar(&self.previous_synchronization_time)?,
//       })
//     }
//   }
// }