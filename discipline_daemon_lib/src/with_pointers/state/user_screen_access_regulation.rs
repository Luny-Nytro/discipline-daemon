use std::collections::BTreeSet;
use crate::with_pointers::types::user::IsUser;
pub use crate::with_pointers::types::user_screen_access_regulation::*;
use crate::chronic::*;
use crate::common::*;
use super::State;

pub struct Rule {
  pub id: u64,
  pub condition: RuleCondition,
  // must stay valid for the lifetime of this rule
  pub policy: *mut Policy,
}

impl IsRule for Rule {
  fn id(&self) -> u64 {
    self.id
  }

  fn condition(&self) -> &RuleCondition {
    &self.condition
  }

  fn policy(&self) -> &impl IsPolicy {
    unsafe {
      &*(self.policy)
    }
  }
}

pub struct Policy {
  id: u64,
  name: PolicyName,
  rules: BTreeSet<*mut Rule>,
  is_enabled: bool,
  protetcor: CountdownTimer
}

pub struct PolicyRuleIter<'a> {
  rules: &'a BTreeSet<*mut Rule>,
  index: usize,
  iter: std::collections::btree_set::Iter<'a, *mut Rule>
}

impl<'a> PolicyRuleIter<'a> {
  pub fn new(rules: &'a BTreeSet<*mut Rule>) -> Self {
    Self {
      index: 0,
      rules,
      iter: rules.iter(),
    }
  }
}

impl<'a> Iterator for PolicyRuleIter<'a> {
  type Item = &'a Rule;

  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|rule| unsafe {
      &**rule
    })
  }
}

impl IsPolicy for Policy {
  fn id(&self) -> u64 {
    self.id
  }

  fn name(&self) -> &PolicyName {
    &self.name
  }

  fn is_enabled(&self) -> bool {
    self.is_enabled
  }

  fn protector(&self) -> &CountdownTimer {
    &self.protetcor
  }

  fn rules(&self) -> impl Iterator<Item = &impl IsRule> {
    PolicyRuleIter::new(&self.rules)
  }
}

pub struct Regulator {
  is_applying_enabled: bool,
  is_user_screen_access_blocked: bool,
  policies: Vec<*mut Policy>,
}

impl IsRegulator for Regulator {
  fn is_applying_enabled(&self) -> bool {
    self.is_applying_enabled
  }

  fn is_user_screen_access_blocked(&self) -> bool {
    self.is_user_screen_access_blocked
  }

  fn policies(&self) -> impl Iterator<Item = &impl IsPolicy> {
    PolicyIter::new(&self.policies)
  }
}

impl IsMutableRegulator for Regulator {
  fn change_is_applying_enabled(&mut self, new_value: bool) {
    self.is_applying_enabled = new_value;
  }

  fn change_is_user_screen_access_blocked(&mut self, new_value: bool) {
    self.is_user_screen_access_blocked = new_value;
  }
}

pub struct PolicyIter<'a> {
  index: usize,
  policies: &'a Vec<*mut Policy>,
}

impl<'a> PolicyIter<'a> {
  pub fn new(policies: &'a Vec<*mut Policy>) -> Self {
    Self {
      index: 0,
      policies,
    }
  }
}

impl<'a> Iterator for PolicyIter<'a> {
  type Item = &'a Policy;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.policies.len() {
      let ptr = self.policies[self.index];
      self.index += 1;
      Some(unsafe { &*ptr })
    } else {
      None
    }
  }
}

pub struct CommonInfo {
  private_password: OperatingSystemPassword,
  applying_interval: Duration
}

impl IsCommonInfo for CommonInfo {
  fn private_password(&self) -> &OperatingSystemPassword {
    &self.private_password
  }

  fn applying_interval(&self) -> Duration {
    self.applying_interval
  }
}

impl IsMutableCommonInfo for CommonInfo {
  fn change_applying_interval(&mut self, new_value: Duration) {
    self.applying_interval = new_value;
  }

  fn change_private_password(&mut self, new_value: OperatingSystemPassword) {
    self.private_password = new_value;
  }
}

pub enum CreateRuleError {
  UnknownPolicy,
}

pub fn create_rule(
  state: &mut State,
  rule_condition: RuleCondition,
  policy_id: u64,
) -> Result<(), CreateRuleError> {
  let Some(policy) = state.user_screen_access_regulation_policies.get(&policy_id) else {
    return Err(CreateRuleError::UnknownPolicy);
  };

  state.user_screen_access_regulation_previous_rule_id += 1;

  let rule = Box::new(Rule {
    id: state.user_screen_access_regulation_previous_rule_id,
    condition: rule_condition,
    policy: *policy,
  });

  let rule = Box::into_raw(rule);

  state.user_screen_access_regulation_rules.insert(
    state.user_screen_access_regulation_previous_rule_id, 
    rule,
  );

  Ok(())
}

pub enum DeleteRuleError {
  UnknownRule,
}

pub fn delete_rule(
  state: &mut State,
  rule_id: u64,
) -> Result<(), DeleteRuleError> {
  let Some(rule_pointer) = state.user_screen_access_regulation_rules.get(&rule_id) else {
    return Err(DeleteRuleError::UnknownRule);
  };

  let rule = unsafe {
    &** rule_pointer
  };

  let policy = unsafe {
    &mut *rule.policy
  };

  policy.rules.remove(rule_pointer);

  state.user_screen_access_regulation_rules.remove(&rule_id);

  Ok(())
}

fn generate_policy_id(state: &mut State) -> u64 {
  state.user_screen_access_regulation_previous_policy_id += 1;
  state.user_screen_access_regulation_previous_policy_id
}

pub fn create_policy(
  state: &mut State,
  now: DateTime,
  user_id: u64,
  name: PolicyName,
  protection_duration: Duration,
) {
  let Some(user_pointer) = state.users.get(&user_id) else {
    return;
  };

  // let id = generate_policy_id(state);

  state.user_screen_access_regulation_previous_policy_id += 1;
  let id = state.user_screen_access_regulation_previous_policy_id;

  let policy = Box::new(Policy {
    id,
    name,
    is_enabled: false,
    protetcor: CountdownTimer::new(protection_duration, now),
    rules: BTreeSet::new(),
  });

  let policy_pointer = Box::into_raw(policy);
  state.user_screen_access_regulation_policies.insert(id, policy_pointer);
  
  unsafe {
    (**user_pointer).user_screen_access_regulation.policies.;
  }
}