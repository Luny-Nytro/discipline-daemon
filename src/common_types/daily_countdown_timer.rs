use serde::{Deserialize, Serialize};
use crate::{CountdownTimer, DateTime, Duration, TimeSynchronizeListener};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCountdownTimer {
  countdown_timer: CountdownTimer
}

impl DailyCountdownTimer {
  pub fn new(duration: Duration, beginning: DateTime) -> Self {
    Self {
      countdown_timer: CountdownTimer::new(duration, beginning)
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

impl TimeSynchronizeListener for DailyCountdownTimer {
  fn on_time_synchronize(&mut self, event: &crate::SynchronizeContext) {
    if self.countdown_timer
      .previous_synchronization_time()
      .midnight() < event.datetime().midnight() 
    {
      self.countdown_timer.reinitialize();
    } else {
      self.countdown_timer.on_time_synchronize(event.datetime().clone());
    }
  }
}