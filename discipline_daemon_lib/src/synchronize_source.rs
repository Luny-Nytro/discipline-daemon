use crate::{DateTime, Duration, Time};

pub struct SynchronizeSource {
  last_synchronization_time: DateTime,
  last_context: SynchronizeContext,
}

impl SynchronizeSource {
  pub fn create_context_for_now(&mut self) -> SynchronizeContext {
    let now = DateTime::now();

    SynchronizeContext {
      time: now.time(), 
      datetime: now, 
      interval: now.since_or_zero(&self.last_synchronization_time)
    }
  }
}

pub struct SynchronizeContext {
  time: Time,
  datetime: DateTime,
  interval: Duration,
}

impl SynchronizeContext {
  pub fn new_for_now() -> Self {
    todo!()
  }
  pub fn datetime(&self) -> DateTime {
    self.datetime
  }

  pub fn time(&self) -> Time {
    self.time
  }

  pub fn interval(&self) -> Duration {
    self.interval
  }
}

pub trait TimeSynchronizeListener {
  fn on_time_synchronize(&mut self, event: &SynchronizeContext);
}