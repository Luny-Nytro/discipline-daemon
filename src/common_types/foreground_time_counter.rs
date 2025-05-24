use serde::{Deserialize, Serialize};
use crate::{Duration, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForegroundTimeCounter {
  time_elapsed: Duration,
  previous_synchronization_time: DateTime,
}

impl ForegroundTimeCounter {
  pub fn new(now: DateTime) -> Self {
    Self {
      time_elapsed: Duration::ZERO,
      previous_synchronization_time: now,
    }
  }

  pub fn synchronize(&mut self, arg: &SynchronizeArg) {
    let Some(duration_since_previous_sync) = arg.now.since(&self.previous_synchronization_time) else {
      return;
    };

    let Some(time_elapsed) = self.time_elapsed.checked_add(&duration_since_previous_sync) else {
      return;
    };

    self.time_elapsed = time_elapsed.min(arg.minimum_synchronization_interval);
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

pub struct SynchronizeArg {
  pub now: DateTime,
  pub minimum_synchronization_interval: Duration
}
