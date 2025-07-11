use serde::{Deserialize, Serialize};
use crate::{DateTime, Duration};
use super::foreground_countdown_timer::{ForegroundCountdownTimer, SynchronizeArg};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyForegroundCountdownTimer {
  countdown_timer: ForegroundCountdownTimer
}

impl DailyForegroundCountdownTimer {
  pub fn new(duration: Duration, beginning: DateTime) -> Self {
    Self {
      countdown_timer: ForegroundCountdownTimer::new(duration, beginning)
    }
  }

  pub fn synchronize(&mut self, arg: &SynchronizeArg) {
    if self.countdown_timer.previous_synchronization_time().midnight() < arg.now.midnight() {
      self.countdown_timer.reinitialize();
    } else {
      self.countdown_timer.synchronize(arg);
    }
  }

  pub fn reinitialize(&mut self) {
    self.countdown_timer.reinitialize();
  }

  pub fn duration(&self) -> Duration {
    self.countdown_timer.duration()
  }

  pub fn remaining_duration(&self) -> Duration {
    self.countdown_timer.remaining_duration()
  }

  pub fn previous_synchronization_time(&self) -> DateTime {
    self.countdown_timer.previous_synchronization_time()
  }
}