use serde::{Deserialize, Serialize};
use crate::{Duration, DateTime};
use super::SynchronizeArg;

UserActiveTimeAllowance;

UserActiveTimeCounter;
UserActiveCountdownTime;
DeviceActiveTimeCounter;
DeviceActiveCountdownTimer;
NetworkingEnabledTimeCounter;
NetworkingEnabledCountdownTimer,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundTimeCounter {
  time_elapsed: Duration,
  previous_synchronization_time: DateTime,
}

impl BackgroundTimeCounter {
  pub fn new(now: DateTime) -> Self {
    Self {
      time_elapsed: Duration::ZERO,
      previous_synchronization_time: now,
    }
  }

  pub fn synchronize(&mut self, arg: &SynchronizeArg) {
    let Some(duration_since_previous_sync) = arg.datetime.since(&self.previous_synchronization_time) else {
      return;
    };

    let Some(time_elapsed) = self.time_elapsed.checked_add(&duration_since_previous_sync) else {
      return;
    };

    self.time_elapsed = time_elapsed;
  }

  /// Returns how much time elapsed since the creation of this TimeCounter
  /// or since the last call to `TimeCounter::reinitialize`.
  pub fn time_elapsed(&self) -> Duration {
    self.time_elapsed
  }

  pub fn reinitialize(&mut self, now: DateTime) {
    self.time_elapsed = Duration::ZERO;
    self.previous_synchronization_time = now;
  }

  pub fn previous_synchronization_time(&self) -> DateTime {
    self.previous_synchronization_time
  }
}


pub mod database {
  use crate::database::{Deserialize, DeserializeContext, ScalarTypeError, SerializationContext, Serialize};
  use super::BackgroundTimeCounter;

  impl Serialize for BackgroundTimeCounter {
    fn serialize_into(&self, ctx: &mut SerializationContext) {
      ctx.scalar(&self.time_elapsed);
      ctx.scalar(&self.previous_synchronization_time);
    }
  }

  #[derive(Debug)]
  pub enum DeserializeError {
    TimeElapsed(ScalarTypeError),
    PreviousSynchronizationTime(ScalarTypeError),
  }

  impl Deserialize for BackgroundTimeCounter {
    type Error = DeserializeError;

    fn columns_number() -> usize {
      2
    }

    fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
      Ok(Self {
        time_elapsed: match ctx.scalar_type() {
          Ok(value) => value,
          Err(error) => return Err(DeserializeError::TimeElapsed(error)),
        },
        previous_synchronization_time: match ctx.scalar_type() {
          Ok(value) => value,
          Err(error) => return Err(DeserializeError::PreviousSynchronizationTime(error)),
        },
      })
    }
  }
}