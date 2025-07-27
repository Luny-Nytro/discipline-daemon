use std::collections::HashMap;

use super::*;

pub struct State {
  pub user_screen_access_regulation_previous_rule_id: u64,
  pub user_screen_access_regulation_previous_policy_id: u64,
  pub user_screen_access_regulation_rules: HashMap<u64, user_screen_access_regulation::RulePointer>,
  pub user_screen_access_regulation_policies: HashMap<u64, user_screen_access_regulation::PolicyPointer>,
  // pub user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo,
  pub users: HashMap<u64, user::UserPointer>,
  pub users_previous_id: u64,
}