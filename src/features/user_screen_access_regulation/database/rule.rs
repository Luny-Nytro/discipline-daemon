use super::{
  Rule, RuleActivator, GenericError, Uuid, RuleActivatorSchema, 
  ScalarFieldSpecification, CompoundValueSerializer, InitializeTableStatement,
  CompoundValueDeserializer, CompoundValueDeserializerContext, SerializeContext,
  UpdateStatement, WriteColumns, WriteColumnsContext,
  CollectionSpecfication, Connection, DatabaseNamespace,
  generate_sql_delete_where_3_columns,
  generate_sql_add_row,
  generate_sql_delete_where_1_column,
};

pub struct RuleSchema {
  pub table_metadata: CollectionSpecfication,
  pub(super) id_column: ScalarFieldSpecification,
  pub(super) user_id_column: ScalarFieldSpecification,
  pub(super) policy_id_column: ScalarFieldSpecification,
  pub(super) position_column: ScalarFieldSpecification,
  pub(super) activator_type: RuleActivatorSchema,
}

impl RuleSchema {
  pub fn new(database_namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = database_namespace
      .create_table("rules")
      .map_err(|error| error.change_context("create rule schema"))?;

    let column_namespace = table.column_namespace();

    Ok(Self {
      id_column: column_namespace
        .create_column_builder("id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create rule schema"))?,

      user_id_column: column_namespace
        .create_column_builder("user_id")
        .build()
        .map_err(|error| error.change_context("create rule schema"))?,

      policy_id_column: column_namespace
        .create_column_builder("policy_id")
        .build()
        .map_err(|error| error.change_context("create rule schema"))?,

      position_column: column_namespace
        .create_column_builder("position")
        .build()
        .map_err(|error| error.change_context("create rule schema"))?,

      activator_type: RuleActivatorSchema
        ::new(column_namespace.create_namespace("activator"))
        .map_err(|error| error.change_context("create rule schema"))?,

      table_metadata: table,
    })
  }

  pub fn activator(&self) -> &RuleActivatorSchema {
    &self.activator_type
  }
  
  pub fn set_position(
    &self, 
    modifications: &mut CollectionItemModifications,
    new_value: u32
  ) {
    modifications.modify_scalar_field(&self.position_column, &new_value);
  }
}

pub struct RuleSerializer<'a> {
  rule_schema: &'a RuleSchema,
  rule_position: usize,
  user_id: &'a Uuid,
  policy_id: &'a Uuid,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    user_id: &'a Uuid,
    policy_id: &'a Uuid,
    rule_schema: &'a RuleSchema,
    rule_position: usize,
  ) -> Self {
    Self {
      user_id,
      policy_id,
      rule_schema,
      rule_position,
    }
  }
}

impl<'a> CompoundValueSerializer for RuleSerializer<'a> {
  type Input = Rule;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) {
    context.serializable_scalar(&self.rule_schema.id_column, &value.id);
    context.serializable_scalar(&self.rule_schema.position_column, &self.rule_position);
    context.serializable_scalar(&self.rule_schema.user_id_column, self.user_id);
    context.serializable_compound(&self.rule_schema.activator_type, &value.activator);
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
  pub(super) id: Uuid,
  pub(super) position: u32,
  pub(super) activator: RuleActivator,
}

impl NormalizedRule {
  pub fn finalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
    }
  }
}

impl CompoundValueDeserializer for RuleSchema {
  type Output = NormalizedRule;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedRule {
      user_id: context.deserializable_scalar(&self.id_column).map_err(|error|
        error
          .change_context("deserialize normalized rule")
          .add_error("failed to deserialize the 'user_id' field")
      )?,
      policy_id: context.deserializable_scalar(&self.id_column).map_err(|error|
        error
          .change_context("deserialize normalized rule")
          .add_error("failed to deserialize the 'policy_id' field")
      )?,
      id: context.deserializable_scalar(&self.id_column).map_err(|error|
        error
          .change_context("deserialize normalized rule")
          .add_error("failed to deserialize the 'id' field")
      )?,
      position: context.deserializable_scalar(&self.position_column).map_err(|error|
        error
          .change_context("deserialize normalized rule")
          .add_error("failed to deserialize the 'position' field")
      )?,
      activator: context.deserialize_compound(&self.activator_type).map_err(|error|
        error
          .change_context("deserialize normalized rule")
          .add_error("failed to deserialize the 'activator' field")
      )?,
    })
  }
}

impl WriteColumns for RuleSchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write_scalar_type(&self.id_column)?;
    context.write_scalar_type(&self.policy_id_column)?;
    context.write_scalar_type(&self.position_column)?;
    context.write_scalar_type(&self.user_id_column)?;
    context.write_compound_type(&self.activator_type)?;
    Ok(())
  }
}

impl RuleSchema {
  pub fn generate_sql_initialize(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError> 
  {
    let mut statement = InitializeTableStatement::new(into, &self.table_metadata);
    statement
      .add_compound_type(self)
      .and_then(|_| statement.finish())
      .map_err(|error|  error.change_context("generate sql code that initializes the rules table"))
  }

  pub fn generate_sql_add_rule(
    &self,
    into: &mut String, 
    user_id: &Uuid,
    policy_id: &Uuid,
    rule: &Rule,
    rule_position: usize,
  ) -> 
    Result<(), GenericError> 
  {
    let serializer = RuleSerializer::new(
      user_id,
      policy_id,
      self, 
      rule_position, 
    );

    generate_sql_add_row(into, &self.table_metadata, &serializer, rule)
      .map_err(|error| 
        error
          .change_context("generate sql code that adds a rule to the rules table")
          .add_attachment("user id", user_id.to_string())
          .add_attachment("policy id", policy_id.to_string())
          .add_attachment("rule", format!("{rule:?}"))
          .add_attachment("rule position", rule_position.to_string())
      )
  }

  pub fn add_rule(
    &self,
    connection: &Connection,
    user_id: &Uuid,
    policy_id: &Uuid,
    rule: &Rule, 
    rule_position: usize,
  ) -> 
    Result<(), GenericError>
  {
    let mut code = String::new();

    self
      .generate_sql_add_rule(&mut code, user_id, policy_id, rule, rule_position)
      .map_err(|error| error.change_context("add a rule to the rules table"))?;

    connection
      .execute(&code)
      .map_err(|error|
        error
          .change_context("add a rule to the rules table")
          .add_attachment("user id", user_id.to_string())
          .add_attachment("policy id", policy_id.to_string())
          .add_attachment("rule", format!("{rule:?}"))
          .add_attachment("rule position", rule_position.to_string())
      )
  }

  pub fn generate_sql_delete_rule(
    &self,
    into: &mut String, 
    rule_id: &Uuid,
    user_id: &Uuid,
    policy_id: &Uuid,
  ) {
    generate_sql_delete_where_3_columns(
      into,
      &self.table_metadata, 
      &self.id_column, 
      rule_id,
      &self.user_id_column,
      user_id,
      &self.policy_id_column,
      policy_id,
    )
  }

  pub fn delete_rule(
    &self,
    connection: &Connection, 
    user_id: &Uuid,
    policy_id: &Uuid,
    rule_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();

    self.generate_sql_delete_rule(
      &mut code, 
      user_id,
      policy_id,
      rule_id,
    );

    connection.execute(&code).map_err(|error| 
      error
        .change_context("delete a rule from the rules table")
        .add_attachment("user id", user_id.to_string())
        .add_attachment("policy id", policy_id.to_string())
        .add_attachment("rule id", rule_id.to_string())
    )
  }

  pub fn generate_sql_delete_rules_of_user(
    &self,
    into: &mut String, 
    user_id: &Uuid,
  ) {
    generate_sql_delete_where_1_column(
      into,
      &self.table_metadata, 
      &self.user_id_column, 
      user_id,
    );
  }

  pub fn load_all_rules_normalized(
    &self,
    connection: &Connection,
  ) -> 
    Result<Vec<NormalizedRule>, GenericError> 
  {
    connection
      .find_all_rows(&self.table_metadata, self)
      .map_err(|error| error.change_context("load all rules from the rules table in normalized form"))
  }

  pub fn create_updater(
    &self, 
    user_id: &Uuid,
    policy_id: &Uuid,
    rule_id: &Uuid,
  ) -> 
    UpdateStatement
  {
    UpdateStatement::new_given_three_where_columns(
      &self.id_column, 
      rule_id, 
      &self.policy_id_column, 
      policy_id, 
      &self.user_id_column, 
      user_id,
    )
  }
}