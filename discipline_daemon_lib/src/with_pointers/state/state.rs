use std::collections::HashMap;

use super::*;

pub struct State {
  pub user_screen_access_regulation_previous_rule_id: u64,
  pub user_screen_access_regulation_previous_policy_id: u64,
  pub user_screen_access_regulation_rules: HashMap<u64, *mut user_screen_access_regulation::Rule>,
  pub user_screen_access_regulation_policies: HashMap<u64, *mut user_screen_access_regulation::Policy>,
  pub user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo,
  pub users: HashMap<u64, *mut user::User>,
  pub users_previous_id: u64,
}