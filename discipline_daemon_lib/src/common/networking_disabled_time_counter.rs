use crate::{DateTime, Duration};
use super::{ForegroundTimeCounter, SynchronizeArg};

pub struct NetworkingDisabledTimeCounter {
  time_counter: ForegroundTimeCounter
}

impl NetworkingDisabledTimeCounter {
  pub fn synchronize(&mut self, arg: &SynchronizeArg) {
    if !arg.networking_enabled {
      self.time_counter.synchronize(arg);
    }
  }

  pub fn reinitialize(&mut self, now: DateTime) {
    self.time_counter.reinitialize(now);
  }

  pub fn networking_disabled_duration(&self) -> Duration {
    self.time_counter.time_elapsed()
  }

  pub fn previous_synchronization_time(&self) -> DateTime {
    self.time_counter.previous_synchronization_time()
  }
}