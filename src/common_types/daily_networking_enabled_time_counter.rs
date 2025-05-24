use crate::{DateTime, Duration};
use super::{DailyReinitializingTimeCounter, SynchronizeArg};

pub struct DailyNetworkingEnabledTimeCounter {
  time_counter: DailyReinitializingTimeCounter
}

impl DailyNetworkingEnabledTimeCounter {
  pub fn synchronize(&mut self, arg: &SynchronizeArg) {
    if arg.networking_enabled {
      self.time_counter.synchronize(&arg);
    }
  }

  pub fn networking_enabled_duration(&self) -> Duration {
    self.time_counter.time_elapsed()
  }

  pub fn reinitialize(&mut self, now: DateTime) {
    self.time_counter.reinitialize(now);
  }

  pub fn previous_synchronization_time(&self) -> DateTime {
    self.time_counter.previous_synchronization_time()
  }
}