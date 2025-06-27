use super::{
  Field, CompoundTypeDefiner, GenericError, Duration,
  Regulator, CollectionItemModificationsDraft, CompoundTypeSerializerContext,
  CompoundValueDeserializer, CompoundValueDeserializerContext,
  NormalizedPolicy, NormalizedRule, CompoundTypeSerializer,
  OperatingSystemCalls, Uuid, IsCompoundType, PolicySpecification, 
  Collection, RuleSpecification, Database, PolicySerializer, RuleSerializer,
  Policy, Rule, CollectionItemMatcher, PolicyName, WeekdayRange, TimeRange,
};

pub struct RegulatorSpecification {
  pub is_applying_enabled: Field,
  pub is_user_screen_access_blocked: Field,
  pub policy: PolicySpecification,
  pub policy_collection: Collection,
  pub rule: RuleSpecification,
  pub rule_collection: Collection,
}

impl IsCompoundType for RegulatorSpecification {
  fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
    let (policy_collection, policy) = definer.collection("Policies")?;
    let (rule_collection, rule) = definer.collection("Rules")?;

    Ok(Self {
      is_applying_enabled: definer.writable_required_field("IsApplyingEnabled")?,
      is_user_screen_access_blocked: definer.writable_required_field("IsUserScreenAccessBlocked")?,
      policy,
      policy_collection,
      rule,
      rule_collection,
    })
  }

  fn display_name(&self) -> &str {
    "Regulator"
  }
}

impl RegulatorSpecification {
  pub fn write_is_applying_enabled(
    &self, 
    draft: &mut CollectionItemModificationsDraft,
    new_value: bool,
  ) ->
    Result<(), GenericError>
  {
    draft.write_scalar_field(&self.is_applying_enabled, &new_value)
  }

  pub fn set_is_user_screen_access_blocked(
    &self, 
    draft: &mut CollectionItemModificationsDraft,
    new_value: bool,
  ) ->
    Result<(), GenericError>
  {
    draft.write_scalar_field(&self.is_user_screen_access_blocked, &new_value)
  }
}

impl CompoundTypeSerializer for RegulatorSpecification {
  type CompoundType = Regulator;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
  ) -> 
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.is_applying_enabled, &value.is_applying_enabled)?;
    context.serializable_scalar(&self.is_user_screen_access_blocked, &value.is_user_screen_access_blocked)
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRegulator {
  pub(super) is_applying_enabled: bool,
  pub(super) is_user_screen_access_blocked: bool,
}

impl CompoundValueDeserializer for RegulatorSpecification {
  type Output = NormalizedRegulator;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedRegulator {
      is_applying_enabled: context.deserializable_scalar(&self.is_applying_enabled)?,
      is_user_screen_access_blocked: context.deserializable_scalar(&self.is_user_screen_access_blocked)?,
    })
  }
}

impl NormalizedRegulator {
  pub fn denormalize(
    self, 
    user_id: &Uuid,
    normalized_policies: &Vec<NormalizedPolicy>,
    normalized_rules: &Vec<NormalizedRule>,
  ) -> Regulator {
    // normalized_policies.sort_by(|a, b| a.position.cmp(&b.position));
    // normalized_rules.sort_by(|a, b| a.position.cmp(&b.position));

    Regulator {
      policies: normalized_policies
        .iter()
        .filter(|policy| policy.user_id == *user_id)
        .map(|policy| policy.clone().denormalize(user_id, &normalized_rules))
        .collect(),
      is_applying_enabled: self.is_applying_enabled,
      operating_system_calls: OperatingSystemCalls::new(),
      is_user_screen_access_blocked: self.is_user_screen_access_blocked,
    }
  }
}

impl RegulatorSpecification {
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