use crate::DateTime;

pub struct TimeTracker {
  beginning: DateTime,
}

impl TimeTracker {
  pub fn new(now: DateTime) -> Self {
    Self {
      beginning: now,
    }
  }

  pub fn from_fields(beginning: DateTime) -> Self {
    Self {
      beginning,
    }
  }
  
  pub fn synchronize(&mut self, now: DateTime) {

  }
}