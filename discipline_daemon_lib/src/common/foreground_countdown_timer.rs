use serde::{Deserialize, Serialize};
use crate::{DateTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForegroundCountdownTimer {
  duration: Duration,
  remaining_duration: Duration,
  previous_synchronization_time: DateTime,
}

impl ForegroundCountdownTimer {
  pub fn new(duration: Duration, beginning: DateTime) -> Self {
    Self {
      duration,
      remaining_duration: duration,
      previous_synchronization_time: beginning,
    }
  }

  pub fn synchronize(&mut self, arg: &SynchronizeArg) {
    let Some(duration_since_previous_synchronization) = arg.now.since(&self.previous_synchronization_time) else {
      return;
    };

    let Some(remaining_duration) = self.remaining_duration.checked_sub(&duration_since_previous_synchronization) else {
      return;
    };

    self.remaining_duration = remaining_duration.min(arg.synchronization_interval);
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

  pub fn remaining_duration(&self) -> &Duration {
    &self.remaining_duration
  }

  pub fn previous_synchronization_time(&self) -> &DateTime {
    &self.previous_synchronization_time
  }
}

pub struct SynchronizeArg {
  pub now: DateTime,
  pub synchronization_interval: Duration,
}