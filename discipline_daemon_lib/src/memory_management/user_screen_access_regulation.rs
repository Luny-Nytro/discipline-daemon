use std::collections::BTreeSet;
use crate::chronic::*;
use super::*;

pub enum RuleCondition {
  Always,
  Weekday(Weekday),
  TimeRange(TimeRange),
  WeekdayRange(WeekdayRange),
}

pub struct RuleCore {
  pub id: u64,
  pub condition: RuleCondition,
}

pub struct Rule {
  core: RuleCore,
  policy: Auto<Policy>,
}

impl Auto<Rule> {
  fn is_effective(&self, now: DateTime) -> bool {
    match &self.core.condition {
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
}

pub struct PolicyName(String);

pub struct PolicyCore {
  id: u64,
  name: PolicyName,
  enabled: bool,
  protector: CountdownTimer
}

pub struct Policy {
  core: PolicyCore,
  rules: Vec<Auto<Rule>>,
}

impl Auto<Policy> {
  pub fn protected(&self) -> bool {
    self.core.protector.is_running()
  }

  pub fn synchronize(&mut self, now: DateTime) {
    self.core.protector.synchronize(now);
  }
}

pub struct Regulation {
  policies: Vec<Auto<Policy>>,
}

pub fn find_policy(state: &State, policy_id: u64) -> Option<Auto<Policy>> {
  state
    .user_screen_access_regulation_policies
    .get(&policy_id)
    .map(|policy| *policy)
}

fn create_policy_id(state: &mut State) -> u64 {
  state.user_screen_access_regulation_previous_policy_id += 1;
  state.user_screen_access_regulation_previous_policy_id
}

pub fn add_policy(
  state: &mut State,
  core: PolicyCore,
  regulation: &mut Regulation,
) -> Auto<Policy> {
  let policy = allocate(Policy {
    core,
    rules: BTreeSet::new(),
  });

  regulation.policies.push(policy);
  state.user_screen_access_regulation_policies.insert(policy.core.id, policy);

  policy
}

// pub enum CreatePolicyError {
//   NoSuchUser
// }

// pub fn create_policy_given_user_id(
//   state: &mut State,
//   user_id: u64,
//   policy_name: PolicyName,
//   policy_protection_duration: Duration,
// ) -> Result<PolicyPointer, CreatePolicyError> {
//   let Some(user) = user::find_user(state, user_id) else {
//     return Err(CreatePolicyError::NoSuchUser);
//   };

//   Ok(create_policy_given_regulation(
//     state, 
//     user.user_screen_access_regulation_source(), 
//     policy_name,
//     policy_protection_duration,
//   ))
// }

// fn create_rule_id(state: &mut State) -> u64 {
//   state.user_screen_access_regulation_previous_rule_id += 1;
//   state.user_screen_access_regulation_previous_rule_id
// }

pub fn add_rule(
  state: &mut State,
  core: RuleCore,
  mut policy: Auto<Policy>,
) {
  let rule = allocate(Rule {
    core,
    policy,
  });

  policy.rules.push(rule);
  state.user_screen_access_regulation_rules.insert(rule.core.id, rule);
}

// pub fn create_rule_given_policy_id(
//   state: &mut State,
//   policy_id: u64,
//   rule_condition: RuleCondition,
// ) 
//   -> Option<CreateRuleError>
// {
//   let Some(policy_pointer) = find_policy(state, policy_id) else {
//     return Err(CreateRuleError::UnknownPolicy);
//   };

//   state.user_screen_access_regulation_previous_rule_id += 1;

//   let rule = RulePointer(allocate(Rule {
//     id: state.user_screen_access_regulation_previous_rule_id,
//     condition: rule_condition,
//     policy: *policy_pointer,
//   }));

//   state.user_screen_access_regulation_rules.insert(
//     state.user_screen_access_regulation_previous_rule_id, 
//     rule,
//   );

//   Ok(())
// }

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
    &mut *rule.policy_pointer
  };

  policy.rules.remove(rule_pointer);

  state.user_screen_access_regulation_rules.remove(&rule_id);

  Ok(())
}
