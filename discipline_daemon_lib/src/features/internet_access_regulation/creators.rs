use serde::{Deserialize, Serialize};

use crate::{
  CountdownTimer, DateTime, Duration, Hour, Password, PasswordEnabler, TimeRange, Uuid, Weekday, WeekdayRange
};

use super::{
  Activator, Enabler, Rule
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivatorCreator {
  AtWeekday(Weekday),
  NotAtWeekday(Weekday),
  InTimeRange(TimeRange),
  NotInTimeRange(TimeRange),
  AtHour(Hour),
  NotAtHour(Hour),
  ForDuration { duration: Duration },
  InWeekdayRange(WeekdayRange),
  NotInWeekdayRange(WeekdayRange),
}

impl ActivatorCreator {
  pub fn create(self, now: DateTime) -> Activator {
    match self {
      Self::AtWeekday(weekday) => {
        Activator::AtWeekday(weekday)
      }
      Self::NotAtWeekday(weekday) => {
        Activator::NotAtWeekday(weekday)
      }
      Self::InTimeRange(time_range) => {
        Activator::InTimeRange(time_range)
      }
      Self::NotInTimeRange(time_range) => {
        Activator::NotInTimeRange(time_range)
      }
      Self::AtHour(hour) => {
        Activator::AtHour(hour)
      }
      Self::NotAtHour(hour) => {
        Activator::NotAtHour(hour)
      }
      Self::ForDuration { duration } => {
        Activator::ForDuration(CountdownTimer::new(duration, now))
      }
      Self::InWeekdayRange(weekday_range) => {
        Activator::InWeekdayRange(weekday_range)
      }
      Self::NotInWeekdayRange(weekday_range) => {
        Activator::NotInWeekdayRange(weekday_range)
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnablerCreator {
  ForDuration(Duration),
  ByPassword(Password),
}

impl EnablerCreator {
  pub fn create(self, now: DateTime) -> Enabler {
    match self {
      Self::ForDuration(duration) => {
        Enabler::ForDuration(CountdownTimer::new(duration, now))
      }
      Self::ByPassword(password) => {
        Enabler::ByPassword(PasswordEnabler::new(password, false))
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreator {
  pub id: Option<Uuid>,
  enabler: EnablerCreator,
  activator: ActivatorCreator,
}

impl RuleCreator {
  pub fn create(self, now: DateTime) -> Rule {
    Rule {
      id: self.id.unwrap_or_else(Uuid::new_v4),
      enabler: self.enabler.create(now),
      activator: self.activator.create(now),
    }
  }
}
