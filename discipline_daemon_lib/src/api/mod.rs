mod rpc;
pub use rpc::IsRemoteProcedureCall;

use serde::{Deserialize, Serialize};
use crate::{user, user_screen_access_regulation, DaemonMutex};

mod into_public;
pub use into_public::IntoPublic;

mod server;
pub use server::launch_thread;

type UserCreate = user::operations::create::Operation;
static USER_CREATE_ID: &'static str = "UserCreate"; 

type UserDelete = user::operations::delete::Operation;
static USER_DELETE_ID: &'static str = "UserDelete"; 

type UserChangeName = user::operations::change_name::Operation;
static USER_CHANGE_NAME_ID: &'static str = "UserChangeName"; 

type UserScreenAccessRegulationPolicyCreate = user_screen_access_regulation::operations::create_policy::CreatePolicy;
static USER_SCREEN_ACCESS_REGULATION_POLICY_CREATE_ID: &'static str = "UserScreenAccessRegulationPolicyCreate"; 

type UserScreenAccessRegulationPolicyDelete = user_screen_access_regulation::operations::delete_policy::Operation;
static USER_SCREEN_ACCESS_REGULATION_POLICY_DELETE_ID: &'static str = "UserScreenAccessRegulationPolicyDelete"; 

type UserScreenAccessRegulationPolicyIncrementEnabledDuration = user_screen_access_regulation::operations::increase_policy_enabled_duration::Operation;
static USER_SCREEN_ACCESS_REGULATION_POLICY_INCREMENT_ENABLED_DURATION_ID: &'static str = "UserScreenAccessRegulationPolicyIncrementEnabledDuration"; 

type UserScreenAccessRegulationPolicyChangeName = user_screen_access_regulation::operations::change_policy_name::ChangePolicyName;
static USER_SCREEN_ACCESS_REGULATION_POLICY_CHANGE_NAME_ID: &'static str = "UserScreenAccessRegulationPolicyChangeName"; 

type UserScreenAccessRegulationRuleCreate = user_screen_access_regulation::operations::create_rule::Operation;
static USER_SCREEN_ACCESS_REGULATION_RULE_CREATE_ID: &'static str = "UserScreenAccessRegulationRuleCreate"; 

type UserScreenAccessRegulationRuleDelete = user_screen_access_regulation::operations::delete_rule::Operation;
static USER_SCREEN_ACCESS_REGULATION_RULE_DELETE_ID: &'static str = "UserScreenAccessRegulationRuleDelete"; 

type UserScreenAccessRegulationRuleActivatorExpandTimeRange = user_screen_access_regulation::operations::change_rule_activator_time_range::Operation;
static USER_SCREEN_ACCESS_REGULATION_RULE_ACTIVATOR_EXPAND_TIME_RANGE_ID: &'static str = "UserScreenAccessRegulationRuleActivatorExpandTimeRange"; 

type UserScreenAccessRegulationRuleActivatorExpandWeekdayRange = user_screen_access_regulation::operations::change_rule_activator_weekday_range::Operation;
static USER_SCREEN_ACCESS_REGULATION_RULE_ACTIVATOR_EXPAND_WEEKDAY_RANGE_ID: &'static str = "UserScreenAccessRegulationRuleActivatorExpandWeekdayRange"; 

#[macro_use]
#[macro_export]
macro_rules! find_operation_type {
  ($operation_id:expr, |$op_type:ident| $code:block else $else:block) => {
    match $operation_id {
      id if id == USER_CREATE_ID => {
        type $op_type = UserCreate;
        $code
      }
      id if id == USER_DELETE_ID => {
        type $op_type = UserDelete;
        $code
      }
      id if id == USER_CHANGE_NAME_ID => {
        type $op_type = UserChangeName;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_POLICY_CREATE_ID => {
        type $op_type = UserScreenAccessRegulationPolicyCreate;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_POLICY_DELETE_ID => {
        type $op_type = UserScreenAccessRegulationPolicyDelete;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_POLICY_INCREMENT_ENABLED_DURATION_ID => {
        type $op_type = UserScreenAccessRegulationPolicyIncrementEnabledDuration;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_POLICY_CHANGE_NAME_ID => {
        type $op_type = UserScreenAccessRegulationPolicyChangeName;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_RULE_CREATE_ID => {
        type $op_type = UserScreenAccessRegulationRuleCreate;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_RULE_DELETE_ID => {
        type $op_type = UserScreenAccessRegulationRuleDelete;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_RULE_ACTIVATOR_EXPAND_TIME_RANGE_ID => {
        type $op_type = UserScreenAccessRegulationRuleActivatorExpandTimeRange;
        $code
      }
      id if id == USER_SCREEN_ACCESS_REGULATION_RULE_ACTIVATOR_EXPAND_WEEKDAY_RANGE_ID => {
        type $op_type = UserScreenAccessRegulationRuleActivatorExpandWeekdayRange;
        $code
      }
      _ => {
        $else
      },
    }
  };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
  UserCreate(user::operations::create::Operation),
  UserDelete(user::operations::delete::Operation),
  UserChangeName(user::operations::change_name::Operation),
  UserScreenAccessRegulationPolicyCreate(user_screen_access_regulation::operations::create_policy::CreatePolicy),
  UserScreenAccessRegulationPolicyDelete(user_screen_access_regulation::operations::delete_policy::Operation),
  UserScreenAccessRegulationPolicyIncrementEnabledDuration(user_screen_access_regulation::operations::increase_policy_enabled_duration::Operation),
  UserScreenAccessRegulationPolicyChangeName(user_screen_access_regulation::operations::change_policy_name::ChangePolicyName),
  UserScreenAccessRegulationRuleCreate(user_screen_access_regulation::operations::create_rule::Operation),
  UserScreenAccessRegulationRuleDelete(user_screen_access_regulation::operations::delete_rule::Operation),
  UserScreenAccessRegulationRuleActivatorExpandTimeRange(user_screen_access_regulation::operations::change_rule_activator_time_range::Operation),
  UserScreenAccessRegulationRuleActivatorExpandWeekdayRange(user_screen_access_regulation::operations::change_rule_activator_weekday_range::Operation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  UserCreate(user::operations::create::Outcome),
  UserDelete(user::operations::delete::Outcome),
  UserChangeName(user::operations::change_name::Outcome),
  UserScreenAccessRegulationPolicyCreate(user_screen_access_regulation::operations::create_policy::Outcome),
  UserScreenAccessRegulationPolicyDelete(user_screen_access_regulation::operations::delete_policy::Outcome),
  UserScreenAccessRegulationPolicyIncrementEnabledDuration(user_screen_access_regulation::operations::increase_policy_enabled_duration::Outcome),
  UserScreenAccessRegulationPolicyChangeName(user_screen_access_regulation::operations::change_policy_name::Outcome),
  UserScreenAccessRegulationRuleCreate(user_screen_access_regulation::operations::create_rule::Outcome),
  UserScreenAccessRegulationRuleDelete(user_screen_access_regulation::operations::delete_rule::Outcome),
  UserScreenAccessRegulationRuleActivatorExpandTimeRange(user_screen_access_regulation::operations::change_rule_activator_time_range::Outcome),
  UserScreenAccessRegulationRuleActivatorExpandWeekdayRange(user_screen_access_regulation::operations::change_rule_activator_weekday_range::Outcome),
}

pub struct Api {
  daemon_mutex: DaemonMutex,
  // operation_map
}

impl Api {

}