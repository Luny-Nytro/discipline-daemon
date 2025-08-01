use std::fmt::Debug;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::Uuid;
use crate::chronic::{DateTime, Duration, TimeRange, WeekdayRange};
use crate::operating_system_integration::UserId;
use crate::logic::internet_access_regulation::*;
use crate::api::IntoPublic;
use crate::Daemon;
use super::public::*;
use crate::database::internet_access_regulation_rule as rule_db;
use crate::database::internet_access_regulation_policy as policy_db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicy {
  user_id: UserId,
  policy_creator: PolicyCreator
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatePolicyReturn {
  NoSuchUser { user_id: UserId },
  ReachedMaximumPolicesAllowed,
  Success(PolicyPublicRepr),
  InternalError,
}

impl CreatePolicy {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationCreatePolicy";

  pub fn execute(self, daemon: Arc<Daemon>) -> CreatePolicyReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return CreatePolicyReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return CreatePolicyReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;

    if regulation.reached_maximum_polices_allowed() {
      return CreatePolicyReturn::ReachedMaximumPolicesAllowed;
    }

    let now = DateTime::now();
    let policy = self.policy_creator.create(now);

    if let Err(error) = policy_db::add_policy(
      daemon.database(), 
      &policy, 
      self.user_id
    ) {
      daemon.internal_logger().log_error(error);
      return CreatePolicyReturn::InternalError;
    }

    regulation.add_policy(policy.clone());
    CreatePolicyReturn::Success(policy.into_public())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicy {
  user_id: UserId,
  policy_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeletePolicyReturn {
  NoSuchUser { user_id: UserId },
  NoSuchPolicy,
  PolicyIsStillEnabled,
  Success,
  InternalError,
}

impl DeletePolicy {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationDeletePolicy";

  pub fn execute(self, daemon: Arc<Daemon>) -> DeletePolicyReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return DeletePolicyReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return DeletePolicyReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;

    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return DeletePolicyReturn::NoSuchPolicy;
    };

    // let now = DateTime::now();
    if policy.is_enabled() {
      return DeletePolicyReturn::PolicyIsStillEnabled;
    }

    if let Err(error) = policy_db::delete_policy(
      daemon.database(), 
      &self.policy_id,
    ) {
      daemon.internal_logger().log_error(error);
      return DeletePolicyReturn::InternalError;
    }

    regulation.remove_policy_by_id(&self.policy_id);
    DeletePolicyReturn::Success
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePolicyName {
  user_id: UserId,
  policy_id: Uuid,
  new_name: PolicyName
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdatePolicyNameReturn {
  NoSuchUser { user_id: UserId },
  NoSuchPolicy,
  Success,
  InternalError,
}

impl UpdatePolicyName {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationUpdatePolicyName";

  pub fn execute(self, daemon: Arc<Daemon>) -> UpdatePolicyNameReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return UpdatePolicyNameReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return UpdatePolicyNameReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;
  
    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return UpdatePolicyNameReturn::NoSuchPolicy;
    };

    if let Err(error) = policy_db::update_name(
      daemon.database(), 
      &self.policy_id, 
      &self.new_name,
    ) {
      daemon.internal_logger().log_error(error);
      return UpdatePolicyNameReturn::InternalError;
    }

    policy.name = self.new_name;
    UpdatePolicyNameReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncreasePolicyProtection {
  user_id: UserId,
  policy_id: Uuid,
  increment: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncreasePolicyProtectionReturn {
  NoSuchUser { user_id: UserId },
  NoSuchPolicy(Uuid),
  WouldBeEffectiveForTooLong,
  Success,
  InternalError,
}

impl IncreasePolicyProtection {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationIncreasePolicyProtection";

  pub fn execute(self, daemon: Arc<Daemon>) -> IncreasePolicyProtectionReturn { 
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return IncreasePolicyProtectionReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return IncreasePolicyProtectionReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;
  
    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return IncreasePolicyProtectionReturn::NoSuchPolicy(self.policy_id);
    };

    let Some(new_remaining_duration) = policy
      .protector()
      .remaining_duration()
      .checked_add(&self.increment) else 
    {
      return IncreasePolicyProtectionReturn::WouldBeEffectiveForTooLong;
    };

    if new_remaining_duration.total_weeks() > 3 {
      return IncreasePolicyProtectionReturn::WouldBeEffectiveForTooLong;
    }

    if let Err(error) = policy_db::update_enabled_duration(
      daemon.database(), 
      &self.policy_id, 
      new_remaining_duration,
    ) {
      daemon.internal_logger().log_error(error);
      return IncreasePolicyProtectionReturn::InternalError;
    }

    policy.protector.change_remaining_duration(self.increment);
    IncreasePolicyProtectionReturn::Success 
  }

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRule {
  user_id: UserId,
  policy_id: Uuid,
  rule_creator: RuleCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreateRuleReturn {
  NoSuchUser { user_id: UserId },
  NoSuchPolicy { policy_id: Uuid },
  RuleCreationLimitReached,
  ProvidedRuleIdIsUsedByAnotherRule,
  Success(RulePublicRepr),
  InternalError,
}

impl CreateRule {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationCreateRule";
    
  pub fn execute(self, daemon: Arc<Daemon>) -> CreateRuleReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return CreateRuleReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return CreateRuleReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;
  
    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return CreateRuleReturn::NoSuchPolicy { policy_id: self.policy_id };
    };

    if policy.reached_maximum_rules_allowed() {
      return CreateRuleReturn::RuleCreationLimitReached;
    }

    let rule = self.rule_creator.create();
    // Note: The database will handle verifing whether "self.creator.id" is available
    // or taken.
    //
    // TODO: Let's do that ourselves so we can return "ProvidedRuleIdIsUsedByAnotherRule"
    // since if the database were to fail, it won't tell us if it is because of a duplicate id. 
    if let Err(error) = rule_db::add_rule(
      daemon.database(), 
      &rule, 
      &self.policy_id, 
      policy.rules_number(),
    ) {
      daemon.internal_logger().log_error(error);
      return CreateRuleReturn::InternalError;
    }

    policy.add_rule(rule.clone());
    CreateRuleReturn::Success(rule.into_public())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRule {
  user_id: UserId,
  rule_id: Uuid,
  policy_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleReturn {
  NoSuchUser { user_id: UserId },
  NoSuchPolicy { policy_id: Uuid },
  NoSuchRule { rule_id: Uuid },
  PolicyIsStillEnabled,
  Success,
  InternalError,
}

impl DeleteRule {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationDeleteRule";

  pub fn execute(self, daemon: Arc<Daemon>) -> DeleteRuleReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return DeleteRuleReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return DeleteRuleReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;
  
    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return DeleteRuleReturn::NoSuchPolicy { policy_id: self.policy_id };
    };

    if policy.there_is_rule_with_id(&self.rule_id) {
      return DeleteRuleReturn::NoSuchRule { rule_id: self.rule_id };
    }

    // let now = DateTime::now();
    if policy.is_enabled() {
      return DeleteRuleReturn::PolicyIsStillEnabled;
    }

    if let Err(error) = rule_db::delete_rule(daemon.database(), &self.rule_id) {
      daemon.internal_logger().log_error(error);
      return DeleteRuleReturn::InternalError;
    }

    policy.remove_rule_by_id(&self.rule_id);
    DeleteRuleReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRuleActivatorTimeRange {
  user_id: UserId,
  rule_id: Uuid,
  policy_id: Uuid,
  new_time_range: TimeRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateRuleActivatorTimeRangeReturn {
  NoSuchUser { user_id: UserId },
  NoSuchPolicy { policy_id: Uuid },
  NoSuchRule { rule_id: Uuid },
  WouldMakeRuleLessRestrictive,
  WrongActivatorType,
  Success,
  InternalError,
}

impl UpdateRuleActivatorTimeRange {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationUpdateRuleActivatorTimeRange";

  // TODO: Refuse to execute this operation if it would result in making 
  // the user blocked for too long or for most of the time.
  //
  // This is crucial for safety to prevent the app user from accidently 
  // blocking himself outside of his account forever or most of the time.
  pub fn execute(self, daemon: Arc<Daemon>) -> UpdateRuleActivatorTimeRangeReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return UpdateRuleActivatorTimeRangeReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return UpdateRuleActivatorTimeRangeReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;
  
    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return UpdateRuleActivatorTimeRangeReturn::NoSuchPolicy { policy_id: self.policy_id };
    };

    let Some(rule) = policy.find_rule_by_id_mut(&self.rule_id) else {
      return UpdateRuleActivatorTimeRangeReturn::NoSuchRule { rule_id: self.rule_id };
    };

    let RuleActivator::InTimeRange(time_range) = &mut rule.activator else {
      return UpdateRuleActivatorTimeRangeReturn::WrongActivatorType;
    };
    
    if self.new_time_range.is_narrower_than(time_range) {
      return UpdateRuleActivatorTimeRangeReturn::WouldMakeRuleLessRestrictive;
    }

    if let Err(error) = rule_db::update_activator_time_range(
      daemon.database(),
      &self.rule_id,
      &self.new_time_range,
    ) {
      daemon.internal_logger().log_error(error);
      return UpdateRuleActivatorTimeRangeReturn::InternalError;
    }

    *time_range = self.new_time_range;
    UpdateRuleActivatorTimeRangeReturn::Success
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRuleActivatorWeekdayRange {
  user_id: UserId,
  rule_id: Uuid,
  policy_id: Uuid,
  new_weekday_range: WeekdayRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateRuleActivatorWeekdayRangeReturn {
  NoSuchUser { user_id: UserId },
  NoSuchPolicy { policy_id: Uuid },
  NoSuchRule { rule_id: Uuid },
  WouldMakeRuleLessRestrictive,
  WrongActivatorType,
  Success,
  InternalError,
}

impl UpdateRuleActivatorWeekdayRange {
  pub const HUMAN_READABLE_ID: &'static str = "InternetAccessRegulationUpdateRuleActivatorWeekdayRange";

  // TODO: Refuse to execute this operation if it would result in making 
  // the user blocked for too long or for most of the time.
  //
  // This is crucial for safety to prevent the app user from accidently 
  // blocking himself outside of his account forever or most of the time.

  pub fn execute(self, daemon: Arc<Daemon>) -> UpdateRuleActivatorWeekdayRangeReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return UpdateRuleActivatorWeekdayRangeReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return UpdateRuleActivatorWeekdayRangeReturn::NoSuchUser { user_id: self.user_id };
    };

    let regulation = &mut user.user_internet_access_regulation_logic;
  
    let Some(policy) = regulation.find_policy_by_id_mut(&self.policy_id) else {
      return UpdateRuleActivatorWeekdayRangeReturn::NoSuchPolicy { policy_id: self.policy_id };
    };

    let Some(rule) = policy.find_rule_by_id_mut(&self.rule_id) else {
      return UpdateRuleActivatorWeekdayRangeReturn::NoSuchRule { rule_id: self.rule_id };
    };

    let RuleActivator::InWeekdayRange(weekday_range) = &mut rule.activator else {
      return UpdateRuleActivatorWeekdayRangeReturn::WrongActivatorType;
    };
    
    if self.new_weekday_range.is_narrower_than(weekday_range) {
      return UpdateRuleActivatorWeekdayRangeReturn::WouldMakeRuleLessRestrictive;
    }

    if let Err(error) = rule_db::update_activator_weekday_range(
      daemon.database(), 
      &self.rule_id, 
      &self.new_weekday_range,
    ) {
      daemon.internal_logger().log_error(error);
      return UpdateRuleActivatorWeekdayRangeReturn::InternalError;
    }

    *weekday_range = self.new_weekday_range;
    UpdateRuleActivatorWeekdayRangeReturn::Success
  }
}