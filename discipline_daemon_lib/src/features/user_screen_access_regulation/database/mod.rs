use crate::database::*;

use crate::{
  Duration, GenericError, Uuid, WeekdayRange, TimeRange
};

use crate::countdown_timer::database::Specification as CountdownTimerSpecification;
use crate::time_range::database::Specification as TimeRangeSpecification;
use crate::weekday_range::database::Specification as WeekdayRangeSpecification;

use super::{
  RuleActivator, PolicyEnabler, 
  OperatingSystemCalls, Policy, PolicyName,
  Regulator, CommonInfo, Rule,
};

mod rule_activator_variant;
pub use rule_activator_variant::RuleActivatorVariant;

mod rule_activator;
pub use rule_activator::RuleActivatorSpecification;

mod policy_enabler;
pub use policy_enabler::PolicyEnablerSpecification;

mod rule;
pub use rule::{RuleSpecification, NormalizedRule, RuleSerializer};

mod regulator;
pub use regulator::{RegulatorSpecification, NormalizedRegulator};

mod common_info;
pub use common_info::CommonInfoSpecification;

mod policy;
pub use policy::*;

mod policy_name;

pub struct Singleton {
  pub common_info: CommonInfoSpecification,
}

impl IsSingleton for Singleton {
  fn new(definer: &mut SingletonDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      common_info: definer.compound_field("CommonInfo")?,
    })
  }

  fn display_name(&self) -> &str {
    "UserScreenAccessRegulation"
  }
}

pub type CompoundType = RegulatorSpecification;
// pub struct CompoundType {
  // regulator: RegulatorSpecification
// }

// impl IsCompoundType for CompoundType {
//   fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
//     Ok(Self {
//       regulator: definer.compound_field("Regulator")?,
//     })
//   }

//   fn display_name(&self) -> &str {
//     "UserAccessRegulation"
//   }
// }

pub struct Module {
  pub policy: PolicySpecification,
  pub policy_collection: Collection,
  pub rule: RuleSpecification,
  pub rule_collection: Collection,
}

impl IsModule for Module {
  fn new(definer: &mut ModuleDefiner) -> Result<Self, GenericError> {
    let (policy_collection, policy) = definer.collection("Policies")?;
    let (rule_collection, rule) = definer.collection("Rules")?;

    Ok(Self {
      policy,
      policy_collection,
      rule,
      rule_collection,
    })
  }
}

impl Module {

  // pub fn apply_modifications_draft(
  //   &self,
  //   database: &Database,
  //   modifications_draft: &CollectionItemChanges,
  //   user_id: &Uuid,
  //   policy_id: &Uuid,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   database.update_collection_items(
  //     &self.collection_specification, 
  //     &CollectionItemMatcher::match_by_multiple_scalar_fields()
  //       .and_scalar_field_is(&self.id, policy_id)?
  //       .and_scalar_field_is(&self.user_id, user_id)?
  //       .finalize()?, 
  //     modifications_draft,
  //   )
  // }

  // pub fn generate_sql_initialize(
  //   &self,
  //   into: &mut String,
  // ) -> Result<(), GenericError> {
  //   generate_sql_initialize_table_given_columns_writer(
  //     into,
  //     &self.collection_specification,
  //     self,
  //   )
  //   .map_err(|error| 
  //     error.change_context("generate sql code that initializes everything related to the policies table")
  //   )
  // }

  // pub fn generate_sql_insert_policy(
  //   &self,
  //   into: &mut String,
  //   policy: &Policy,
  //   user_id: &Uuid,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   let serializer = PolicySerializer::new(user_id, self);
  //   generate_sql_add_row(into, &self.collection_specification, &serializer, policy)
  //     .map_err(|error| 
  //       error.change_context("generate sql code that inserts a policy")
  //     )
  // }

  pub fn add_policy(
    &self,
    database: &Database,
    user_id: &Uuid,
    policy: &Policy,
  ) -> 
    Result<(), GenericError>
  {
    self.policy_collection.add_item(
      database, 
      &PolicySerializer::new(user_id, &self.policy), 
      policy,
    )
  }

  pub fn delete_policy(
    &self,
    database: &Database,
    policy_id: &Uuid,
    user_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    let mut draft = database.create_modifications_draft();

    draft.delete_items(
      &self.policy_collection, 
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .scalar_field_equals(&self.policy.id, policy_id)?
        .scalar_field_equals(&self.policy.user_id, user_id)?
        .finalize()?,
    )?;

    draft.delete_items(
      &self.rule_collection, 
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .scalar_field_equals(&self.rule.policy_id, policy_id)?
        .scalar_field_equals(&self.rule.user_id, user_id)?
        .finalize()?,
    )?;

    draft.commit(database)
  }

  pub fn add_rule(
    &self,
    database: &Database,
    user_id: &Uuid,
    policy_id: &Uuid,
    rule: &Rule,
  ) -> 
    Result<(), GenericError>
  {
    self.rule_collection.add_item(
      database, 
      &RuleSerializer::new(
        user_id, 
        policy_id, 
        &self.rule, 
      ), 
      rule,
    )
  }

  pub fn delete_rule(
    &self,
    database: &Database,
    user_id: &Uuid,
    policy_id: &Uuid,
    rule_id: &Uuid
  ) -> 
    Result<(), GenericError>
  {
    self.rule_collection.delete_items(
      database, 
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .scalar_field_equals(&self.rule.user_id, user_id)?
        .scalar_field_equals(&self.rule.policy_id, policy_id)?
        .scalar_field_equals(&self.rule.id, rule_id)?
        .finalize()?
    )
  }

  pub fn change_policy_name(
    &self,
    database: &Database,
    user_id: &Uuid,
    policy_id: &Uuid,
    new_name: &PolicyName,
  ) ->
    Result<(), GenericError>
  {
    let mut changes = CollectionItemModificationsDraft::new();
    changes.write_scalar_field(&self.policy.name, new_name)?;

    self.policy_collection.update_items(
      database,
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .scalar_field_equals(&self.policy.user_id, user_id)?
        .scalar_field_equals(&self.policy.id, policy_id)?
        .finalize()?, 
      &changes,
    )
  }

  pub fn change_rule_activator_weekday_range(
    &self,
    database: &Database,
    user_id: &Uuid,
    policy_id: &Uuid,
    rule_id: &Uuid,
    new_range: &WeekdayRange
  ) ->
    Result<(), GenericError>
  {
    let mut draft = self.rule_collection.create_modifications_draft();
    
    self
      .rule
      .activator
      .in_weekday_range()
      .change_range(&mut draft, new_range)?;

    self.rule_collection.commit_modifications_draft(
      database, 
      &draft,
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .scalar_field_equals(&self.rule.user_id, user_id)?
        .scalar_field_equals(&self.rule.policy_id, policy_id)?
        .scalar_field_equals(&self.rule.id, rule_id)?
        .finalize()?
    )
  }

  pub fn change_rule_activator_time_range(
    &self,
    database: &Database,
    user_id: &Uuid,
    policy_id: &Uuid,
    rule_id: &Uuid,
    new_range: &TimeRange
  ) ->
    Result<(), GenericError>
  {
    let mut draft = self.rule_collection.create_modifications_draft();
    
    self
      .rule
      .activator
      .in_time_range()
      .write_range(&mut draft, new_range)?;

    self.rule_collection.commit_modifications_draft(
      database, 
      &draft,
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .scalar_field_equals(&self.rule.user_id, user_id)?
        .scalar_field_equals(&self.rule.policy_id, policy_id)?
        .scalar_field_equals(&self.rule.id, rule_id)?
        .finalize()?
    )
  }

  pub fn change_policy_enabled_duration(
    &self,
    database: &Database,
    user_id: &Uuid,
    policy_id: &Uuid,
    new_value: &Duration
  ) ->
    Result<(), GenericError>
  {
    let mut draft = self.policy_collection.create_modifications_draft();
    self
      .policy
      .enabler
      .timer()
      .write_remaining_duration(&mut draft, new_value)?;
    self
      .policy_collection
      .commit_modifications_draft(
        database, 
        &draft, 
        &CollectionItemMatcher::match_by_multiple_scalar_fields()
          .scalar_field_equals(&self.policy.user_id, user_id)?
          .scalar_field_equals(&self.policy.id, policy_id)?
          .finalize()?
      )
  }
}