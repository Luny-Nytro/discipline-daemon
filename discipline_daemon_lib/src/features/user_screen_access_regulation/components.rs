use serde::{Deserialize, Serialize};
use super::OperatingSystemCalls;
use crate::{
  CountdownTimer, DateTime, Duration, GenericError, 
  OperatingSystemPassword, OperatingSystemUsername, 
  TimeRange, Uuid, Weekday, WeekdayRange
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RuleActivator {
  AllTheTime,
  OnWeekday(Weekday),
  InTimeRange(TimeRange),
  InWeekdayRange(WeekdayRange),
}

impl RuleActivator {
  pub fn is_effective(&mut self, now: DateTime) -> bool {
    match self {
      RuleActivator::OnWeekday(weekday) => {
        now.weekday() == *weekday
      }
      RuleActivator::InTimeRange(time_range) => {
        time_range.contains(now.time())
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
  
  pub fn is_effective(&mut self, now: DateTime) -> bool {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEnabler {
  pub(super) timer: CountdownTimer
}

impl PolicyEnabler {
  pub fn new(duration: Duration) -> Self {
    todo!()
  }

  pub fn is_effective(&mut self, now: DateTime) -> bool {
    self.timer.synchronize(now);
    self.timer.is_finished()
  }

  pub fn synchronize(&mut self, now: DateTime) {
    self.timer.synchronize(now);
  }

  pub fn pack(timer: CountdownTimer) -> Self {
    Self {
      timer
    }
  }

  pub fn unpack_ref(&self) -> &CountdownTimer {
    &self.timer
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
  pub(super) id: Uuid,
  pub(super) name: PolicyName,
  pub(super) rules: Vec<Rule>,
  pub(super) enabler: PolicyEnabler,
}

impl Policy {
  pub const MAX_RULES: usize = 10;

  pub fn pack(
    id: Uuid,
    name: PolicyName,
    rules: Vec<Rule>,
    enabler: PolicyEnabler
  ) 
    -> Self 
  {
    Self { id, name, rules, enabler }
  }
  
  pub fn id(&self) -> &Uuid {
    &self.id
  }
  pub fn name(&self) -> &PolicyName {
    &self.name
  }
  pub fn enabler(&self) -> &PolicyEnabler {
    &self.enabler
  }
  pub fn is_enabled(&mut self, now: DateTime) -> bool {
    self.enabler.is_effective(now)
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
}

#[derive(Debug, Clone)]
pub struct Regulator {
  pub(super) policies: Vec<Policy>,
  pub(super) is_applying_enabled: bool,
  pub(super) is_user_screen_access_blocked: bool,
  pub(super) operating_system_calls: OperatingSystemCalls,
}

impl Regulator {
  pub const MAX_POLICIES: usize = 5;

  pub fn new(
    policies: Vec<Policy>,
  ) -> Self {
    Self {
      policies,
      is_applying_enabled: false,
      is_user_screen_access_blocked: false,
      operating_system_calls: OperatingSystemCalls::new(),
    }
  }
  
  pub fn is_applying_enabled(&self) -> bool {
    self.is_applying_enabled
  }

  pub fn is_user_screen_access_blocked(&self) -> bool {
    self.is_user_screen_access_blocked
  }
  
  fn allow_user_access(
    &mut self,
    username: &OperatingSystemUsername,
    password: &OperatingSystemPassword,
  ) -> 
    Result<(), GenericError> 
  {
    if !self.is_user_screen_access_blocked {
      return Ok(());
    }

    match self
      .operating_system_calls
      .change_user_password(username, password) 
    {
      Ok(_) => {
        self.is_user_screen_access_blocked = false;
        Ok(())
      }
      Err(error) => {
        Err(
          error.change_context("Allow user screen access")
        )
      }
    }
  }

  fn block_user_access(
    &mut self, 
    username: &OperatingSystemUsername,
    private_password: &OperatingSystemPassword,
  ) -> 
    Result<(), GenericError> 
  {
    if self.is_user_screen_access_blocked {
      return Ok(());
    }

    match self
      .operating_system_calls
      .change_user_password(username, private_password) 
    {
      Ok(_) => {
        self.is_user_screen_access_blocked = false;
      }
      Err(error) => {
        return Err(
          error.change_context("Block user screen access")
        )
      }
    }

    self
      .operating_system_calls
      .gracefully_logout_user(username)
      .map_err(|error| error.change_context("Block user screen access"))
  }

  pub fn apply(
    &mut self, 
    now: DateTime,
    username: &OperatingSystemUsername,
    password: &OperatingSystemPassword,
    private_password: &OperatingSystemPassword,
  ) -> 
    Result<(), GenericError> 
  {
    if self.is_applying_enabled {
      for policy in &mut self.policies {
        if policy.is_enabled(now) {
          for rule in &mut policy.rules {
            if rule.is_effective(now) {
              return self.block_user_access(
                username,
                private_password
              );
            }
          }
        }
      }
    }

    self.allow_user_access(
      username,
      password
    )
  }

  pub fn are_some_policies_enabled(&mut self, now: DateTime) -> bool {
    self.policies.iter_mut().any(|policy| policy.is_enabled(now))
  }

  pub fn get_policy_by_id(&self, policy_id: &Uuid) -> Option<&Policy> {
    self.policies.iter().find(|policy| policy.id == *policy_id)
  }
  
  pub fn find_policy_by_id_mut(&mut self, policy_id: &Uuid) -> Option<&mut Policy> {
    self.policies.iter_mut().find(|policy| policy.id == *policy_id)
  }

  pub fn policies_number(&self) -> u32 {
    self.policies.len() as u32
  }

  pub fn reached_maximum_polices_allowed(&self) -> bool {
    self.policies.len() >= Self::MAX_POLICIES
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

#[derive(Debug)]
pub struct CommonInfo {
  pub(super) private_password: OperatingSystemPassword,
  pub(super) applying_interval: Duration,
}

impl Default for CommonInfo {
  fn default() -> Self {
    Self {
      private_password: CommonInfo::generate_private_password(),
      applying_interval: CommonInfo::default_applying_interval(),
    }
  }
}

impl CommonInfo {
  pub(super) fn generate_private_password() -> OperatingSystemPassword {
    OperatingSystemPassword::generate_random_password()
  }

  pub(super) fn default_applying_interval() -> Duration {
    Duration::from_minutes(5).unwrap()
  }

  pub fn private_password(&self) -> &OperatingSystemPassword {
    &self.private_password
  }
  
  pub fn applying_interval(&self) -> Duration {
    self.applying_interval
  }
}
