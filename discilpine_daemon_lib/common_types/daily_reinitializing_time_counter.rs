use serde::{Deserialize, Serialize};
use crate::{DateTime, Duration};
use super::foreground_time_counter::ForegroundTimeCounter;

pub use super::foreground_time_counter::SynchronizeArg;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReinitializingTimeCounter {
  time_counter: ForegroundTimeCounter
}

impl DailyReinitializingTimeCounter {
  pub fn new(now: DateTime) -> Self {
    Self {
      time_counter: ForegroundTimeCounter::new(now)
    }
  }

  pub fn synchronize(&mut self, arg: &SynchronizeArg) {
    // if one day or more elapsed since previous synchronization, reset the time counter.
    if self.time_counter.previous_synchronization_time().midnight() < arg.now.midnight() {
      self.time_counter.reinitialize(arg.now);
    } else {
      self.time_counter.synchronize(arg.now);
    }
  }

  /// Returns how much time the counter counted since the start of the day.
  pub fn time_elapsed(&self) -> Duration {
    self.time_counter.time_elapsed()
  }

  pub fn previous_synchronization_time(&self) -> DateTime {
    self.time_counter.previous_synchronization_time()
  }
}