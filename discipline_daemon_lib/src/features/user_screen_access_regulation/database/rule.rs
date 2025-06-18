use super::{
  Rule, RuleActivator, GenericError, Uuid, CompoundTypeSerializerContext,
  ScalarFieldSpecification, CompoundTypeSerializer, CollectionSpecification,
  CompoundValueDeserializer, CompoundValueDeserializerContext, DatabaseNamespace,
  CollectionItemDefiner, RuleActivatorSpecification, CollectionItemModificationsDraft,
  Database, CollectionItemMatcher, CompoundTypeNamespace,
};

pub struct RuleSpecification {
  pub collection: CollectionSpecification,
  pub(super) id: ScalarFieldSpecification,
  pub(super) user_id: ScalarFieldSpecification,
  pub(super) policy_id: ScalarFieldSpecification,
  pub(super) activator: RuleActivatorSpecification,
}

impl RuleSpecification {
  pub fn new(
    database: &mut Database,
    namespace: &mut DatabaseNamespace,
  ) -> 
    Result<Self, GenericError> 
  {
    let mut rule_namespace = CompoundTypeNamespace::new();
    let mut rule_definer = CollectionItemDefiner::new();

    let id = rule_definer
      .define_primary_scalar_field(&mut rule_namespace, "Id")?;

    let user_id = rule_definer
      .define_primary_scalar_field(&mut rule_namespace, "UserId")?;

    let policy_id = rule_definer
      .define_primary_scalar_field(&mut rule_namespace, "PolicyId")?;

    let mut activator_definer = rule_definer
      .define_required_writable_compound_field(&mut rule_namespace, "Activator")?;

    let activator = RuleActivatorSpecification::new(
      &mut rule_namespace, 
      &mut activator_definer,
    )?;

    let collection = namespace
      .define_collection(
        database,
        "Rules", 
        rule_namespace,
      )?;

    Ok(Self {
      activator,
      id,
      policy_id,
      user_id,
      collection,
    })
  }

  pub fn activator(&self) -> &RuleActivatorSpecification {
    &self.activator
  }
  
  // pub fn update_position(
  //   &self, 
  //   modifications: &mut CollectionItemModificationsDraft,
  //   new_value: u32
  // ) ->
  //   Result<(), GenericError>
  // {
  //   modifications.modify_scalar_field(&self.position_field_specification, &new_value)
  // }

  pub fn create_modifications_draft(&self) -> CollectionItemModificationsDraft {
    CollectionItemModificationsDraft::new()
  }

  pub fn apply_modifications_draft(
    &self,
    database: &Database,
    modifications_draft: &CollectionItemModificationsDraft,
    user_id: &Uuid,
    policy_id: &Uuid,
    rule_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    database.update_collection_items(
      &self.collection, 
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .and_scalar_field_is(&self.user_id, user_id)?
        .and_scalar_field_is(&self.policy_id, policy_id)?
        .and_scalar_field_is(&self.id, rule_id)?
        .finalize()?, 
      modifications_draft,
    )
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
    database.add_collection_item(
      &self.collection, 
      &RuleSerializer::new(
        user_id, 
        policy_id, 
        self, 
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
    database.delete_collection_items(
      &self.collection, 
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .and_scalar_field_is(&self.user_id, user_id)?
        .and_scalar_field_is(&self.policy_id, policy_id)?
        .and_scalar_field_is(&self.id, rule_id)?
        .finalize()?
    )
  }
}

pub struct RuleSerializer<'a> {
  rule_specification: &'a RuleSpecification,
  // rule_position: usize,
  user_id: &'a Uuid,
  policy_id: &'a Uuid,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    user_id: &'a Uuid,
    policy_id: &'a Uuid,
    // rule_position: usize,
    rule_specification: &'a RuleSpecification,
  ) -> Self {
    Self {
      user_id,
      policy_id,
      rule_specification,
      // rule_position,
    }
  }
}

impl<'a> CompoundTypeSerializer for RuleSerializer<'a> {
  type CompoundType = Rule;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
  ) -> 
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.rule_specification.id, &value.id)?;
    // context.serializable_scalar(&self.rule_specification.position_field_specification, &self.rule_position)?;
    context.serializable_scalar(&self.rule_specification.user_id, self.user_id)?;
    context.serializable_scalar(&self.rule_specification.policy_id, self.policy_id)?;
    context.serializable_compound(&self.rule_specification.activator, &value.activator)
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) id: Uuid,
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
  // pub(super) position: u32,
  pub(super) activator: RuleActivator,
}

impl NormalizedRule {
  pub fn denormalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
    }
  }
}

impl CompoundValueDeserializer for RuleSpecification {
  type Output = NormalizedRule;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedRule {
      user_id: context.deserializable_scalar(&self.id).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'UserId' field")
      )?,
      policy_id: context.deserializable_scalar(&self.id).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'PolicyId' field")
      )?,
      id: context.deserializable_scalar(&self.id).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'Id' field")
      )?,
      // position: context.deserializable_scalar(&self.position_field_specification).map_err(|error|
      //   error
      //     .change_context("deserializing NormalizedRule")
      //     .add_error("failed to deserialize the 'Position' field")
      // )?,
      activator: context.deserialize_compound(&self.activator).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'Activator' field")
      )?,
    })
  }
}

impl RuleSpecification {
  // pub fn generate_sql_initialize(
  //   &self,
  //   into: &mut String,
  // ) -> 
  //   Result<(), GenericError> 
  // {
  //   let mut statement = InitializeTableStatement::new(into, &self.collection_specification);
  //   statement
  //     .add_compound_type(self)
  //     .and_then(|_| statement.finish())
  //     .map_err(|error|  error.change_context("generate sql code that initializes the rules table"))
  // }

  // pub fn generate_code_add_rule(
  //   &self,
  //   into: &mut String, 
  //   user_id: &Uuid,
  //   policy_id: &Uuid,
  //   rule: &Rule,
  //   rule_position: usize,
  // ) -> 
  //   Result<(), GenericError> 
  // {
  //   let serializer = RuleSerializer::new(
  //     user_id,
  //     policy_id,
  //     self, 
  //     rule_position, 
  //   );

  //   generate_sql_add_row(into, &self.collection_specification, &serializer, rule)
  //     .map_err(|error| 
  //       error
  //         .change_context("generate sql code that adds a rule to the rules table")
  //         .add_attachment("user id", user_id.to_string())
  //         .add_attachment("policy id", policy_id.to_string())
  //         .add_attachment("rule", format!("{rule:?}"))
  //         .add_attachment("rule position", rule_position.to_string())
  //     )
  // }

  // pub fn add_rule(
  //   &self,
  //   connection: &Connection,
  //   user_id: &Uuid,
  //   policy_id: &Uuid,
  //   rule: &Rule, 
  //   rule_position: usize,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   let mut code = String::new();

  //   self
  //     .generate_code_add_rule(&mut code, user_id, policy_id, rule, rule_position)
  //     .map_err(|error| error.change_context("add a rule to the rules table"))?;

  //   connection
  //     .execute(&code)
  //     .map_err(|error|
  //       error
  //         .change_context("add a rule to the rules table")
  //         .add_attachment("user id", user_id.to_string())
  //         .add_attachment("policy id", policy_id.to_string())
  //         .add_attachment("rule", format!("{rule:?}"))
  //         .add_attachment("rule position", rule_position.to_string())
  //     )
  // }

  // pub fn generate_sql_delete_rule(
  //   &self,
  //   into: &mut String, 
  //   rule_id: &Uuid,
  //   user_id: &Uuid,
  //   policy_id: &Uuid,
  // ) {
  //   generate_sql_delete_where_3_field_specifications(
  //     into,
  //     &self.collection_specification, 
  //     &self.id_field_specification, 
  //     rule_id,
  //     &self.user_id_field_specification,
  //     user_id,
  //     &self.policy_id_field_specification,
  //     policy_id,
  //   )
  // }

  // pub fn delete_rule(
  //   &self,
  //   connection: &Connection, 
  //   user_id: &Uuid,
  //   policy_id: &Uuid,
  //   rule_id: &Uuid,
  // ) -> 
  //   Result<(), GenericError> 
  // {
  //   let mut code = String::new();

  //   self.generate_sql_delete_rule(
  //     &mut code, 
  //     user_id,
  //     policy_id,
  //     rule_id,
  //   );

  //   connection.execute(&code).map_err(|error| 
  //     error
  //       .change_context("delete a rule from the rules table")
  //       .add_attachment("user id", user_id.to_string())
  //       .add_attachment("policy id", policy_id.to_string())
  //       .add_attachment("rule id", rule_id.to_string())
  //   )
  // }

  // pub fn generate_sql_delete_rules_of_user(
  //   &self,
  //   into: &mut String, 
  //   user_id: &Uuid,
  // ) {
  //   generate_sql_delete_where_1_field_specification(
  //     into,
  //     &self.collection_specification, 
  //     &self.user_id_field_specification, 
  //     user_id,
  //   );
  // }

  // pub fn load_all_rules_normalized(
  //   &self,
  //   connection: &Connection,
  // ) -> 
  //   Result<Vec<NormalizedRule>, GenericError> 
  // {
  //   connection
  //     .find_all_rows(&self.collection_specification, self)
  //     .map_err(|error| error.change_context("load all rules from the rules table in normalized form"))
  // }

  // pub fn create_updater(
  //   &self, 
  //   user_id: &Uuid,
  //   policy_id: &Uuid,
  //   rule_id: &Uuid,
  // ) -> 
  //   UpdateStatement
  // {
  //   UpdateStatement::new_given_three_where_field_specifications(
  //     &self.id_field_specification, 
  //     rule_id, 
  //     &self.policy_id_field_specification, 
  //     policy_id, 
  //     &self.user_id_field_specification, 
  //     user_id,
  //   )
  // }
}