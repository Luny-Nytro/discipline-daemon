use serde::{Deserialize, Serialize};
use crate::{
  CountdownTimer, DateTime, Duration, GenericError, 
  TimeRange, Uuid, Weekday, WeekdayRange
};

pub const MAXIMUM_RULE_NUMBER: usize = 10;
pub const MAXIMUM_POLICY_NUMBER: usize = 5;

// TODO: Add a variant that is effective according to a weekday and time range condition
// TODO: Add a variant that is effective according to a screen time allowance condition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RuleActivator {
  AllTheTime,
  OnWeekday(Weekday),
  InTimeRange(TimeRange),
  InWeekdayRange(WeekdayRange),
}

impl RuleActivator {
  pub fn is_effective(&self, now: DateTime) -> bool {
    match self {
      RuleActivator::OnWeekday(weekday) => {
        now.weekday() == *weekday
      }
      RuleActivator::InTimeRange(time_range) => {
        time_range.contains_time(now.time())
      }
      RuleActivator::InWeekdayRange(weekday_range) => {
        weekday_range.contains_weekday(now.weekday())
      }
      RuleActivator::AllTheTime => {
        true
      }
    }
  }
}

/// A Rule may not be made less restrictive after it is created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
  pub id: Uuid,
  pub activator: RuleActivator,
}

impl Rule {
  pub fn new(id: Uuid, activator: RuleActivator) -> Self {
    Self {
      id,
      activator,
    }
  }
  
  pub fn id(&self) -> &Uuid {
    &self.id
  }

  pub fn activator(&self) -> &RuleActivator {
    &self.activator
  }
  
  pub fn is_effective(&self, now: DateTime) -> bool {
    self.activator.is_effective(now)
  }
}

#[derive(Debug, Clone)]
pub struct PolicyName(String);

impl PolicyName {
  pub const MIN_LENGTH: usize = 1;
  pub const MAX_LENGTH: usize = 25;

  pub fn new(name: String) -> Result<Self, GenericError> {
    if name.len() < Self::MIN_LENGTH {
      return Err(
        GenericError::new("Failed to create a PolicyName: Provided name is too short")
          .add_attachment("name", name)
          .add_attachment("min length", PolicyName::MIN_LENGTH.to_string())
      );
    }

    if name.len() > Self::MAX_LENGTH {
      return Err(
        GenericError::new("Failed to create a PolicyName: Provided name is too long")
          .add_attachment("name", name)
          .add_attachment("max length", PolicyName::MAX_LENGTH.to_string())
      );
    }

    Ok(Self(name))
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}

// TODO: Disable a policy right when it's no longer protected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
  pub(super) id: Uuid,
  pub(super) name: PolicyName,
  pub(super) rules: Vec<Rule>,
  pub(super) is_effective: bool,
  pub(super) protector: CountdownTimer,
}

impl Policy {
  pub(super) fn new(
    id: Uuid, 
    name: PolicyName, 
    protection_duration: Duration,
    protection_beginning: DateTime,
  ) -> Self {
    Policy {
      id,
      name,
      rules: Vec::new(),
      is_effective: false,
      protector: CountdownTimer::new(protection_duration, protection_beginning),
    }
  }

  pub fn from_fields(
    id: Uuid,
    name: PolicyName,
    rules: Vec<Rule>,
    is_enabled: bool,
    protector_duration: Duration,
    protector_remaining_duration: Duration,
    protector_previous_synchronization_time: DateTime,
  ) 
    -> Self 
  {
    Self { 
      id, 
      name, 
      rules, 
      is_effective: is_enabled,
      protector: CountdownTimer::from_fields(
        protector_duration, 
        protector_remaining_duration, 
        protector_previous_synchronization_time
      )
    }
  }
  
  pub fn id(&self) -> &Uuid {
    &self.id
  }

  pub fn name(&self) -> &PolicyName {
    &self.name
  }

  pub fn protector(&self) -> &CountdownTimer {
    &self.protector
  }

  pub fn protector_mut(&mut self) -> &mut CountdownTimer {
    &mut self.protector
  }
  
  pub fn is_enabled(&self) -> bool {
    self.is_effective
  }

  pub fn is_protected(&mut self, now: DateTime) -> bool {
    self.protector.synchronize(now);
    self.protector.is_running() && self.is_effective
  }

  pub fn there_is_rule_with_id(&self, rule_id: &Uuid) -> bool {
    self.rules.iter().any(|rule| rule.id == *rule_id)
  }
  
  pub fn find_rule_by_id(&self, rule_id: &Uuid) -> Option<&Rule> {
    self.rules.iter().find(|rule| rule.id == *rule_id)
  }
  
  pub fn find_rule_by_id_mut(&mut self, rule_id: &Uuid) -> Option<&mut Rule> {
    self.rules.iter_mut().find(|rule| rule.id == *rule_id)
  }
  
  pub fn remove_rule_by_id(&mut self, rule_id: &Uuid) {
    if let Some(index) = self
      .rules
      .iter()
      .position(|rule| rule.id == *rule_id) 
    {
      self.rules.remove(index);
    }
  }

  fn are_some_rules_effective(&self, now: DateTime) -> bool {
    self.rules.iter().any(|rule| rule.is_effective(now))
  } 

  pub(super) fn reached_maximum_rules_allowed(&self) -> bool {
    self.rules.len() >= MAXIMUM_RULE_NUMBER
  }

  pub(super) fn rules_number(&self) -> usize {
    self.rules.len()
  }

  pub(super) fn add_rule(&mut self, rule: Rule) {
    self.rules.push(rule);
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
  Block,
  Allow,
}

#[derive(Debug, Clone)]
pub struct Regulation {
  pub(super) policies: Vec<Policy>,
}

impl Default for Regulation {
  fn default() -> Self {
    Self {
      policies: Vec::new(),
    }
  }
}

impl Regulation {
  pub fn new(policies: Vec<Policy>) -> Self {
    Self {
      policies,
    }
  }
  
  pub fn from_fields(policies: Vec<Policy>) -> Self {
    Self {
      policies,
    }
  }
  
  fn are_some_policies_effective(&mut self, now: DateTime) -> bool {
    self.policies.iter_mut().any(|policy| 
      policy.is_enabled() 
      && 
      policy.are_some_rules_effective(now)
    )
  }

  pub fn calculate_action(&mut self, now: DateTime) -> Action {
    if self.are_some_policies_effective(now) {
      Action::Block
    } else {
      Action::Allow
    }
  }

  pub fn are_some_policies_enabled(&self) -> bool {
    self.policies.iter().any(|policy| policy.is_enabled())
  }

  pub fn are_some_policies_protected(&mut self, now: DateTime) -> bool {
    self.policies.iter_mut().any(|policy| policy.is_protected(now))
  }

  pub fn find_policy_by_id(&self, policy_id: &Uuid) -> Option<&Policy> {
    self.policies.iter().find(|policy| policy.id == *policy_id)
  }
  
  pub fn find_policy_by_id_mut(&mut self, policy_id: &Uuid) -> Option<&mut Policy> {
    self.policies.iter_mut().find(|policy| policy.id == *policy_id)
  }

  pub fn policies_number(&self) -> usize {
    self.policies.len()
  }

  pub fn reached_maximum_polices_allowed(&self) -> bool {
    self.policies.len() >= MAXIMUM_POLICY_NUMBER
  }

  pub fn add_policy(&mut self, policy: Policy) {
    self.policies.push(policy);
  }

  pub fn remove_policy_by_id(&mut self, policy_id: &Uuid) {
    if let Some(index) = self
      .policies
      .iter()
      .position(|policy| policy.id == *policy_id)
    {
      self.policies.remove(index);
    }
  }
}

// TODO: Add maximum_policies_number and maximum_rules_number fields
// that the user may modify

// TODO: Add policies_number and rules_number fields for tracking memory usage.
#[derive(Debug)]
pub struct CommonInfo {
}

impl Default for CommonInfo {
  fn default() -> Self {
    Self {}
  }
}