use super::{
  Rule, RuleActivator, GenericError, Uuid, CompoundValueSerializerContext,
  ScalarFieldSpecification, CompoundValueSerializer, CollectionSpecification,
  CompoundValueDeserializer, CompoundValueDeserializerContext, Namespace,
  CollectionItemFieldsScope, RuleActivatorSpecification, CollectionItemModifications
};

pub struct RuleSpecification {
  pub collection_specification: CollectionSpecification,
  pub(super) id_field_specification: ScalarFieldSpecification,
  pub(super) user_id_field_specification: ScalarFieldSpecification,
  pub(super) policy_id_field_specification: ScalarFieldSpecification,
  pub(super) position_field_specification: ScalarFieldSpecification,
  pub(super) activator_field_specification: RuleActivatorSpecification,
}

impl RuleSpecification {
  pub fn new(namespace: &mut Namespace) -> Result<Self, GenericError> {
    let mut fields_namespace = CollectionItemFieldsScope::new();

    let id_field_specification = fields_namespace
      .scalar_field_specification("Id")
      .build()
      .map_err(|error| error.change_context("creating RuleSpecification"))?;

    let user_id_field_specification = fields_namespace
      .scalar_field_specification("UserId")
      .build()
      .map_err(|error| error.change_context("creating RuleSpecification"))?;

    let policy_id_field_specification = fields_namespace
      .scalar_field_specification("PolicyId")
      .build()
      .map_err(|error| error.change_context("creating RuleSpecification"))?;

    let position_field_specification = fields_namespace
      .scalar_field_specification("Position")
      .build()
      .map_err(|error| error.change_context("creating RuleSpecification"))?;

    let activator_field_specification = RuleActivatorSpecification
      ::new(&mut fields_namespace.compound_field_specification("activator")?)
      .map_err(|error| error.change_context("creating RuleSpecification"))?;

    let collection_specification = namespace
      .collection("Rules", fields_namespace)
      .map_err(|error| error.change_context("creating RuleSpecification"))?;

    Ok(Self {
      activator_field_specification,
      id_field_specification,
      policy_id_field_specification,
      position_field_specification,
      user_id_field_specification,
      collection_specification,
    })
  }

  pub fn activator(&self) -> &RuleActivatorSpecification {
    &self.activator_field_specification
  }
  
  pub fn update_position(
    &self, 
    modifications: &mut CollectionItemModifications,
    new_value: u32
  ) ->
    Result<(), GenericError>
  {
    modifications.modify_scalar_field(&self.position_field_specification, &new_value)
  }
}

pub struct RuleSerializer<'a> {
  rule_specification: &'a RuleSpecification,
  rule_position: usize,
  user_id: &'a Uuid,
  policy_id: &'a Uuid,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    user_id: &'a Uuid,
    policy_id: &'a Uuid,
    rule_specification: &'a RuleSpecification,
    rule_position: usize,
  ) -> Self {
    Self {
      user_id,
      policy_id,
      rule_specification,
      rule_position,
    }
  }
}

impl<'a> CompoundValueSerializer for RuleSerializer<'a> {
  type CompoundValue = Rule;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) -> 
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.rule_specification.id_field_specification, &value.id)?;
    context.serializable_scalar(&self.rule_specification.position_field_specification, &self.rule_position)?;
    context.serializable_scalar(&self.rule_specification.user_id_field_specification, self.user_id)?;
    context.serializable_compound(&self.rule_specification.activator_field_specification, &value.activator)
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) id: Uuid,
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
  pub(super) position: u32,
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
      user_id: context.deserializable_scalar(&self.id_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'UserId' field")
      )?,
      policy_id: context.deserializable_scalar(&self.id_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'PolicyId' field")
      )?,
      id: context.deserializable_scalar(&self.id_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'Id' field")
      )?,
      position: context.deserializable_scalar(&self.position_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedRule")
          .add_error("failed to deserialize the 'Position' field")
      )?,
      activator: context.deserialize_compound(&self.activator_field_specification).map_err(|error|
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