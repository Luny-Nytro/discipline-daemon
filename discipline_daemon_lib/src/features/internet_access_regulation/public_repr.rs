use serde::{Deserialize, Serialize};

use crate::{
  password_enabler, CountdownTimer, Duration, Hour, OperatingSystemUsername, TimeRange, ToPublicRepr, Uuid, Weekday, WeekdayRange 
};

use super::{
  Activator, Feature, Enabler, Rule, Enforcer
};

// SECTION: Destructor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnablerPublicRepr {
  ForDuration(CountdownTimer),
  ByPassword(password_enabler::public_repr::PublicRepr),
}

impl ToPublicRepr for Enabler {
  type PublicRepr = EnablerPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    match self {
      Enabler::ForDuration(inner) => {
        EnablerPublicRepr::ForDuration(inner.clone())
      } 
      Enabler::ByPassword(inner) => {
        EnablerPublicRepr::ByPassword(inner.to_public_repr())
      }
    }
  }
}

// SECTION: Activator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivatorPublicRepr {
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

impl ToPublicRepr for Activator {
  type PublicRepr = ActivatorPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    match self {
      Activator::AtWeekday(inner) => {
        ActivatorPublicRepr::AtWeekday(inner.clone())
      } 
      Activator::NotAtWeekday(inner) => {
        ActivatorPublicRepr::NotAtWeekday(inner.clone())
      } 
      Activator::InTimeRange(inner) => {
        ActivatorPublicRepr::InTimeRange(inner.clone())
      } 
      Activator::NotInTimeRange(inner) => {
        ActivatorPublicRepr::NotInTimeRange(inner.clone())
      } 
      Activator::AtHour(inner) => {
        ActivatorPublicRepr::AtHour(inner.clone())
      } 
      Activator::NotAtHour(inner) => {
        ActivatorPublicRepr::NotAtHour(inner.clone())
      } 
      Activator::ForDuration(inner) => {
        ActivatorPublicRepr::ForDuration(inner.clone())
      } 
      Activator::InWeekdayRange(inner) => {
        ActivatorPublicRepr::InWeekdayRange(inner.clone())
      } 
      Activator::NotInWeekdayRange(inner) => {
        ActivatorPublicRepr::NotInWeekdayRange(inner.clone())
      }
    }
  }
}

// SECTION: Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePublicRepr {
  id: Uuid,
  enabler: EnablerPublicRepr,
  activator: Activator,
}

impl ToPublicRepr for Rule {
  type PublicRepr = RulePublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    RulePublicRepr {
      id: self.id.clone(),
      enabler: self.enabler.to_public_repr(),
      activator: self.activator.clone(),
    }
  }
}

// SECTION: Enforcer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcerPublicRepr {
  rules: Vec<RulePublicRepr>,
  username: OperatingSystemUsername,
  is_blocked: bool,
  is_enabled: bool,
}

impl ToPublicRepr for Enforcer {
  type PublicRepr = EnforcerPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    EnforcerPublicRepr {
      rules: self.rules.iter_mut().map(ToPublicRepr::to_public_repr).collect(),
      username: self.username.clone(),
      is_blocked: self.is_blocked,
      is_enabled: self.is_enabled,
    }
  }
}

// SECTION: Feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturePublicRepr {
  enforcers: Vec<EnforcerPublicRepr>,
  enforcing_interval: Duration,
}

impl ToPublicRepr for Feature {
  type PublicRepr = FeaturePublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    FeaturePublicRepr {
      enforcing_interval: self.enforcing_interval,
      enforcers: self
        .enforcers
        .iter_mut()
        .map(|enforcer| enforcer.to_public_repr())
        .collect(),
    }
  }
}