use crate::chronic::*;
use crate::common::*;

pub enum RuleCondition {
  Always,
  Weekday(Weekday),
  TimeRange(TimeRange),
  WeekdayRange(WeekdayRange),
}

pub trait IsRule {
  fn id(&self) -> u64;

  fn condition(&self) -> &RuleCondition;

  fn is_effective(&self, now: DateTime) -> bool {
    match self.condition() {
      RuleCondition::Always => {
        true
      }
      RuleCondition::Weekday(weekday) => {
        *weekday == now.weekday()
      }
      RuleCondition::TimeRange(range) => {
        range.contains_time(now.time())
      }
      RuleCondition::WeekdayRange(range) => {
        range.contains_weekday(now.weekday())
      }
    }
  }

  fn policy(&self) -> &impl IsPolicy;
}

pub trait IsMutableRule: IsRule {
  fn policy_mut(&self) -> &impl IsMutablePolicy;
}

pub struct PolicyName(String);

pub trait IsPolicy {
  fn id(&self) -> u64;

  fn name(&self) -> &PolicyName;

  fn is_enabled(&self) -> bool;

  fn protector(&self) -> &CountdownTimer;
  
  fn is_protected(&self) -> bool {
    self.protector().is_running()
  }

  fn rules(&self) -> impl Iterator<Item = &impl IsRule>;
}

pub trait IsMutablePolicy: IsPolicy {
  fn change_name(&self, new_name: PolicyName);

  fn protector_mut(&self) -> &mut CountdownTimer;

  fn synchronize(&self, now: DateTime) {
    self.protector_mut().synchronize(now);
  }
}

pub trait IsRegulator {
  fn policies(&self) -> impl Iterator<Item = &impl IsPolicy>;
  fn is_applying_enabled(&self) -> bool;
  fn is_user_screen_access_blocked(&self) -> bool;
}

pub trait IsMutableRegulator: IsRegulator {
  fn change_is_applying_enabled(&mut self, new_value: bool);
  fn change_is_user_screen_access_blocked(&mut self, new_value: bool);
}

pub trait IsCommonInfo {
  fn private_password(&self) -> &OperatingSystemPassword;
  fn applying_interval(&self) -> Duration;
}

pub trait IsMutableCommonInfo: IsCommonInfo {
  fn change_private_password(&mut self, new_value: OperatingSystemPassword);
  fn change_applying_interval(&mut self, new_value: Duration);
}