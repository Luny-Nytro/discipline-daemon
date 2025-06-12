use super::OperatingSystemCalls;
use serde::{Deserialize, Serialize};

use crate::{
  debug, CountdownTimer, DateTime, Duration, Hour, OperatingSystemUserId, 
  OperatingSystemUsername, PasswordEnabler, TimeRange, Uuid, Weekday, 
  WeekdayRange
};

// SECTION: Enabler.
#[derive(Debug, Clone)]
pub enum Enabler {
  ForDuration(CountdownTimer),
  ByPassword(PasswordEnabler),
}

impl Enabler {
  pub fn is_effective(&mut self, now: DateTime) -> bool {
    match self {
      Enabler::ForDuration(countdown_timer) => {
        countdown_timer.on_time_synchronize(now);
        countdown_timer.is_running()
      }
      Enabler::ByPassword(enabler) => {
        enabler.is_effective()
      }
    }
  }
}

// SECTION: Activator.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Activator {
  AtWeekday(Weekday),
  NotAtWeekday(Weekday),
  InTimeRange(TimeRange),
  NotInTimeRange(TimeRange),
  AtHour(Hour),
  NotAtHour(Hour),
  ForDuration(CountdownTimer),
  InWeekdayRange(WeekdayRange),
  NotInWeekdayRange(WeekdayRange),
}

impl Activator {
  pub fn is_effective(&mut self, now: DateTime) -> bool {
    match self {
      Activator::AtWeekday(weekday) => {
        now.weekday() == *weekday
      }
      Activator::NotAtWeekday(weekday) => {
        now.weekday() != *weekday
      }
      Activator::AtHour(hour) => {
        now.hour() == *hour
      }
      Activator::NotAtHour(hour) => {
        now.hour() != *hour
      }
      Activator::InTimeRange(time_range) => {
        time_range.contains(now.time())
      }
      Activator::NotInTimeRange(time_range) => {
        !time_range.contains(now.time())
      }
      Activator::ForDuration(countdown) => {
        countdown.is_running()
      }
      Activator::InWeekdayRange(weekday_range) => {
        weekday_range.contains_weekday(now.weekday())
      }
      Activator::NotInWeekdayRange(weekday_range) => {
        !weekday_range.contains_weekday(now.weekday())
      }
    }
  }
}

// SECTION: Rule.
#[derive(Debug, Clone)]
pub struct Rule {
  pub(super) id: Uuid,
  pub(super) activator: Activator,
  pub(super) enabler: Enabler,
}

impl Rule {
  pub fn is_enabled(&mut self, now: DateTime) -> bool {
    self.enabler.is_effective(now)
  }  

  pub fn is_effective(&mut self, now: DateTime) -> bool {
    self.activator.is_effective(now) && self.is_enabled(now)
  }
}

// SECTION: Enforcer.
#[derive(Debug)]
pub struct Enforcer {
  pub(super) user_id: OperatingSystemUserId,
  pub(super) username: OperatingSystemUsername,
  pub(super) rules: Vec<Rule>,
  pub(super) is_blocked: bool,
  pub(super) is_enabled: bool,
  pub(super) operating_system_calls: OperatingSystemCalls,
}

impl Enforcer {
  pub fn new(user_id: OperatingSystemUserId, username: OperatingSystemUsername) -> Self {
    Self {
      user_id,
      username,
      rules: Vec::new(),
      is_blocked: false,
      is_enabled: false,
      operating_system_calls: OperatingSystemCalls::new(),
    }
  }

  fn allow_networking(&mut self) {
    if !self.is_blocked {
      return;
    }

    if let Ok(_) = self
      .operating_system_calls
      .allow_networking_access_for_user(&self.user_id, &self.username) 
    {
      // TODO: Save new state.
      self.is_blocked = false;
    }
  }

  fn block_networking(&mut self) {
    if !self.is_enabled {
      let username = self.username.as_ref();
      println!("Discipline.NetworkAccessEnforcer({username}).BlockNetworkAccess: Skipped because disabled.");
      return;
    }

    if self.is_blocked {
      return;
    }

    if let Ok(_) = self
    .operating_system_calls
    .block_networking_access_for_user(&self.user_id, &self.username) 
    {
      // TODO: Save new state.
      self.is_blocked = true;
    }
  }

  pub fn enforce(&mut self, now: DateTime) {
    debug!("NetworkingAccess.Enforcer.ApplyActions: \nUsername: {:?}.\nIsEnabled: {:?}.", self.is_enabled, self.username);
    
    if !self.is_enabled {
      self.allow_networking();
      return;
    }

    for rule in &mut self.rules {
      debug!("\nRule.IsEffective: {}. \nRule.IsAlive: {}. \nRule: {:?}", rule.is_effective(now), rule.is_enabled(now), rule);

      if rule.is_effective(now) {
        return self.block_networking();
      }
    }

    self.allow_networking();
  }

  pub fn are_some_rules_enabled(&mut self, now: DateTime) -> bool {
    self
      .rules
      .iter_mut()
      .any(|rule| rule.is_enabled(now))
  }
}

// SECTION: Feature.
#[derive(Debug)]
pub struct Feature {
  pub(super) enforcers: Vec<Enforcer>,
  pub(super) enforcing_interval: Duration,
}

impl Feature {
  pub fn len(&self) -> usize {
    self.enforcers.len()
  }

  pub fn is_enforcer_created_for_user(&self, username: &OperatingSystemUsername) -> bool {
    self.enforcers.iter().any(|enforcer| enforcer.username == *username)
  }

  pub fn apply_enforcers(&mut self, now: DateTime) {
    for enforcer in &mut self.enforcers {
      enforcer.enforce(now);
    }
  }

  pub fn enforcing_interval(&self) -> Duration {
    self.enforcing_interval
  }
}
