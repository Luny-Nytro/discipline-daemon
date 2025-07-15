use serde::{Deserialize, Serialize};
use crate::{DateTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTracker {
  // A random immutable time point that we treat as the very beginning of trackable time.
  // since this TimeTracker is initialized when Discipline Daemon runs for the 1st, this is
  // likely to refer to that time point.
  epoch: DateTime,
  // how many time has passed since we began tracking time, that is, since self.beginning.
  duration: Duration,
}

impl TimeTracker {
  pub fn new(epoch: DateTime) -> Self {
    Self {
      epoch,
      duration: Duration::ZERO,
    }
  }

  pub fn from_fields(epoch: DateTime, duration: Duration) -> Self {
    Self {
      epoch,
      duration,
    }
  }
  
  pub fn epoch(&self) -> DateTime {
    self.epoch
  }

  pub fn duration_since_epoch(&self) {

  }

  pub fn duration_since(&self, time: DateTime) -> Option<Duration> {
    let epoch_timestamp = self.epoch.timestamp();
    let time_timestamp = time.timestamp();

    let Some(difference) = time_timestamp.checked_sub(epoch_timestamp) else {
      // 'time' is earlier than 'epoch'. This could happen when the operating system time
      // is misconfigured after 'epoch' was initialized: Some devices' inner clock reset
      // to a very early time, likely the time when the device was manufactured, when the 
      // device completely loses power.
      return None;
    };

    let Ok(difference) = difference.try_into() else {
      // This could fail for the same reason: operating system time misconfigured.
      return None;
    };

    Some(Duration::from_milliseconds(difference))
  }

  pub fn synchronize(&mut self, now: DateTime) {
    self.duration = now.since_or_zero(&self.epoch);
  }
}