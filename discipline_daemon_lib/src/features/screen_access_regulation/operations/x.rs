use std::fmt::Debug;
use super::*;

pub trait ExecuteOperationContext {
  fn database(&self) -> &Database;
  fn internal_logger(&self) -> &impl InternalLogger;
}

pub trait InternalLogger {
  fn log(&self, value: impl Debug);
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicy {
  user_id: Uuid,
  policy_creator: PolicyCreator
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatePolicyReturn {
  ReachedMaximumPolicesAllowed,
  Success(PolicyPublicRepr),
  InternalError,
}

fn create_policy(
  context: impl ExecuteOperationContext,
  operation: CreatePolicy,
  regulation: &mut Regulation,
) 
  -> CreatePolicyReturn
{
  if regulation.reached_maximum_polices_allowed() {
    return CreatePolicyReturn::ReachedMaximumPolicesAllowed;
  }

  let now = DateTime::now();
  let policy = operation.policy_creator.create(now);

  if let Err(error) = policy_db::add_policy(
    context.database(), 
    &policy, 
    &operation.user_id
  ) {
    context.internal_logger().log(error);
    return CreatePolicyReturn::InternalError;
  }

  regulation.add_policy(policy.clone());
  CreatePolicyReturn::Success(policy.into_public())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicy {
  policy_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeletePolicyReturn {
  NoSuchPolicy,
  MayNotDeletePolicyWhileEnabled,
  Success,
  InternalError,
}

fn delete_policy(
  context: impl ExecuteOperationContext,
  operation: DeletePolicy,
  regulation: &mut Regulation,
) 
  -> DeletePolicyReturn
{
  let Some(policy) = regulation.find_policy_by_id_mut(&operation.policy_id) else {
    return DeletePolicyReturn::NoSuchPolicy;
  };

  // let now = DateTime::now();
  if policy.is_enabled() {
    return DeletePolicyReturn::MayNotDeletePolicyWhileEnabled;
  }

  if let Err(error) = policy_db::delete_policy(
    context.database(), 
    &operation.policy_id,
  ) {
    context.internal_logger().log(error);
    return DeletePolicyReturn::InternalError;
  }

  regulation.remove_policy_by_id(&operation.policy_id);
  DeletePolicyReturn::Success
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePolicyName {
  policy_id: Uuid,
  new_name: PolicyName
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdatePolicyNameReturn {
  NoSuchPolicy,
  Success,
  InternalError,
}

fn update_policy_name(
  context: impl ExecuteOperationContext,
  operation: UpdatePolicyName,
  regulation: &mut Regulation,
) 
  -> UpdatePolicyNameReturn 
{
  let Some(policy) = regulation.find_policy_by_id_mut(&operation.policy_id) else {
    return UpdatePolicyNameReturn::NoSuchPolicy;
  };

  if let Err(error) = policy_db::update_name(
    context.database(), 
    &operation.policy_id, 
    &operation.new_name,
  ) {
    context.internal_logger().log(error);
    return UpdatePolicyNameReturn::InternalError;
  }

  policy.name = operation.new_name;
  UpdatePolicyNameReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncreasePolicyProtection {
  policy_id: Uuid,
  increment: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncreasePolicyProtectionReturn {
  // NoSuchUser,
  NoSuchPolicy(Uuid),
  WouldBeEffectiveForTooLong,
  Success,
  InternalError,
}


fn increase_policy_protection(
  context: impl ExecuteOperationContext,
  operation: IncreasePolicyProtection,
  regulation: &mut Regulation,
) 
  -> IncreasePolicyProtectionReturn 
{ 
  let Some(policy) = regulation.find_policy_by_id_mut(&operation.policy_id) else {
    return IncreasePolicyProtectionReturn::NoSuchPolicy(operation.policy_id);
  };

  let Some(new_remaining_duration) = policy
    .protector()
    .remaining_duration()
    .checked_add(&operation.increment) else 
  {
    return IncreasePolicyProtectionReturn::WouldBeEffectiveForTooLong;
  };

  if new_remaining_duration.total_weeks() > 3 {
    return IncreasePolicyProtectionReturn::WouldBeEffectiveForTooLong;
  }

  if let Err(error) = policy_db::update_enabled_duration(
    context.database(), 
    &operation.policy_id, 
    new_remaining_duration,
  ) {
    context.internal_logger().log(error);
    return IncreasePolicyProtectionReturn::InternalError;
  }

  policy.protector.change_remaining_duration(operation.increment);
  IncreasePolicyProtectionReturn::Success 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRule {
  policy_id: Uuid,
  rule_creator: RuleCreator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreateRuleReturn {
  NoSuchUser { user_id: Uuid },
  NoSuchPolicy { policy_id: Uuid },
  RuleCreationLimitReached,
  ProvidedRuleIdIsUsedByAnotherRule,
  Success(RulePublicRepr),
  InternalError,
}

fn create_rule(
  context: impl ExecuteOperationContext,
  CreateRule { policy_id, rule_creator }: CreateRule,
  regulation: &mut Regulation
) 
  -> CreateRuleReturn 
{
  let Some(policy) = regulation.find_policy_by_id_mut(&policy_id) else {
    return CreateRuleReturn::NoSuchPolicy { policy_id };
  };

  if policy.reached_maximum_rules_allowed() {
    return CreateRuleReturn::RuleCreationLimitReached;
  }

  let rule = rule_creator.create();
  // Note: The database will handle verifing whether "self.creator.id" is available
  // or taken.
  //
  // TODO: Let's do that ourselves so we can return "ProvidedRuleIdIsUsedByAnotherRule"
  // since if the database were to fail, it won't tell us if it is because of a duplicate id. 
  if let Err(error) = rule_db::add_rule(
    context.database(), 
    &rule, 
    &policy_id, 
    policy.rules_number(),
  ) {
    context.internal_logger().log(error);
    return CreateRuleReturn::InternalError;
  }

  policy.add_rule(rule.clone());
  CreateRuleReturn::Success(rule.into_public())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRule {
  rule_id: Uuid,
  policy_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeleteRuleReturn {
  NoSuchPolicy,
  NoSuchRule,
  MayNotDeleteRuleWhilePolicyEnabled,
  Success,
  InternalError,
}

fn delete_rule(
  context: impl ExecuteOperationContext,
  DeleteRule { rule_id, policy_id }: DeleteRule,
  regulation: &mut Regulation,
) 
  -> DeleteRuleReturn 
{
  let Some(policy) = regulation
    .find_policy_by_id_mut(&policy_id) else 
  {
    return DeleteRuleReturn::NoSuchPolicy;
  };

  if policy.there_is_rule_with_id(&rule_id) {
    return DeleteRuleReturn::NoSuchRule;
  }

  // let now = DateTime::now();
  if policy.is_enabled() {
    return DeleteRuleReturn::MayNotDeleteRuleWhilePolicyEnabled;
  }

  if let Err(error) = rule_db::delete_rule(context.database(), &rule_id) {
    context.internal_logger().log(error);
    return DeleteRuleReturn::InternalError;
  }

  policy.remove_rule_by_id(&rule_id);
  DeleteRuleReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRuleActivatorTimeRange {
  rule_id: Uuid,
  policy_id: Uuid,
  new_time_range: TimeRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateRuleActivatorTimeRangeReturn {
  NoSuchPolicy,
  NoSuchRule,
  MayNotMakeRuleLessRestrictive,
  WrongActivatorType,
  Success,
  InternalError,
}

// TODO: Refuse to execute this operation if it would result in making 
// the user blocked for too long or for most of the time.
//
// This is crucial for safety to prevent the app user from accidently 
// blocking himself outside of his account forever or most of the time.
fn update_rule_activator_time_range(
  context: impl ExecuteOperationContext,
  operation: UpdateRuleActivatorTimeRange,
  regulation: &mut Regulation,
) 
  -> UpdateRuleActivatorTimeRangeReturn
{
  let Some(policy) = regulation.find_policy_by_id_mut(&operation.policy_id) else {
    return UpdateRuleActivatorTimeRangeReturn::NoSuchPolicy;
  };

  let Some(rule) = policy.find_rule_by_id_mut(&operation.rule_id) else {
    return UpdateRuleActivatorTimeRangeReturn::NoSuchRule;
  };

  let RuleActivator::InTimeRange(time_range) = &mut rule.activator else {
    return UpdateRuleActivatorTimeRangeReturn::WrongActivatorType;
  };
  
  if operation.new_time_range.is_narrower_than(time_range) {
    return UpdateRuleActivatorTimeRangeReturn::MayNotMakeRuleLessRestrictive;
  }

  if let Err(error) = rule_db::update_activator_time_range(
    context.database(),
    &operation.rule_id,
    &operation.new_time_range,
  ) {
    context.internal_logger().log(error);
    return UpdateRuleActivatorTimeRangeReturn::InternalError;
  }

  *time_range = operation.new_time_range;
  UpdateRuleActivatorTimeRangeReturn::Success
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRuleActivatorWeekdayRange {
  rule_id: Uuid,
  policy_id: Uuid,
  new_weekday_range: WeekdayRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateRuleActivatorWeekdayRangeReturn {
  NoSuchPolicy,
  NoSuchRule,
  WrongActivatorType,
  MayNotMakeRuleLessRestrictive,
  Success,
  InternalError,
}
// TODO: Refuse to execute this operation if it would result in making 
// the user blocked for too long or for most of the time.
//
// This is crucial for safety to prevent the app user from accidently 
// blocking himself outside of his account forever or most of the time.

fn update_rule_activator_weekday_range(
  context: impl ExecuteOperationContext,
  operation: UpdateRuleActivatorWeekdayRange,
  regulation: &mut Regulation,
) 
  -> UpdateRuleActivatorWeekdayRangeReturn 
{
  let Some(policy) = regulation.find_policy_by_id_mut(&operation.policy_id) else {
    return UpdateRuleActivatorWeekdayRangeReturn::NoSuchPolicy;
  };

  let Some(rule) = policy.find_rule_by_id_mut(&operation.rule_id) else {
    return UpdateRuleActivatorWeekdayRangeReturn::NoSuchRule;
  };

  let RuleActivator::InWeekdayRange(weekday_range) = &mut rule.activator else {
    return UpdateRuleActivatorWeekdayRangeReturn::WrongActivatorType;
  };
  
  if operation.new_weekday_range.is_narrower_than(weekday_range) {
    return UpdateRuleActivatorWeekdayRangeReturn::MayNotMakeRuleLessRestrictive;
  }

  if let Err(error) = rule_db::update_activator_weekday_range(
    context.database(), 
    &operation.rule_id, 
    &operation.new_weekday_range,
  ) {
    context.internal_logger().log(error);
    return UpdateRuleActivatorWeekdayRangeReturn::InternalError;
  }

  *weekday_range = operation.new_weekday_range;
  UpdateRuleActivatorWeekdayRangeReturn::Success
}