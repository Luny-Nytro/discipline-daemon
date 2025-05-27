use serde::{Serialize, Deserialize};
use crate::{CountdownTimer, DateTime, Duration, Uuid};
use super::{Policy, PolicyEnabler, PolicyName, Rule, RuleActivator};

pub type RuleActivatorCreator = RuleActivator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreator {
  pub id: Option<Uuid>,
  pub activator: RuleActivatorCreator,
}

impl RuleCreator {
  pub fn create(self) -> Rule {
    Rule {
      id: self.id.unwrap_or_else(Uuid::new_v4),
      activator: self.activator,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEablerCreator {
  duration: Duration
}

impl PolicyEablerCreator {
  pub fn create(self, now: DateTime) -> PolicyEnabler {
    PolicyEnabler {
      timer: CountdownTimer::new(self.duration, now)
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCreator {
  id: Option<Uuid>,
  name: PolicyName,
  enabler: PolicyEablerCreator
}

impl PolicyCreator {
  pub fn create(self, now: DateTime) -> Policy {
    Policy {
      id: self.id.unwrap_or_else(Uuid::new_v4),
      name: self.name,
      rules: Vec::new(),
      enabler: self.enabler.create(now),
      creation_time: now,
    }
  }
}