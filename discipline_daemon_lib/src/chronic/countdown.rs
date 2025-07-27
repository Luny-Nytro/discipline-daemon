use serde::{Deserialize, Serialize};
use super::{Duration, DateTime, TimeTracker};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countdown {
  duration: Duration,
  beginning: DateTime,
}

impl Countdown {
  pub fn new(duration: Duration, beginning: DateTime) -> Self {
    Self {
      duration,
      beginning,
    }
  }

  pub fn from_fields(duration: Duration, beginning: DateTime) -> Self {
    Self { 
      duration,
      beginning,
    }
  }

  pub fn duration(&self) -> Duration {
    self.duration
  }

  pub fn change_duration(&mut self, new_value: Duration) {
    self.duration = new_value;
  }

  pub fn remaining_duration(&self, time_tracker: &TimeTracker) -> Option<Duration> {
    time_tracker.duration_since(self.beginning)
  }

  pub fn is_running(&self, time_tracker: &TimeTracker) -> Option<bool> {
    self
      .remaining_duration(time_tracker)
      .map(|duration| duration > Duration::ZERO)
  }

  pub fn is_finished(&self, time_tracker: &TimeTracker) -> Option<bool> {
    self
      .remaining_duration(time_tracker)
      .map(|duration| duration == Duration::ZERO)
  }

  pub fn reinitialize(&mut self, new_beginning: DateTime) {
    self.beginning = new_beginning;
  }
}