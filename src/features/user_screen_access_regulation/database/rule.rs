use super::{
  Rule, RuleActivator, GenericError, Uuid, RuleActivatorSchema, 
  Column, CompoundValueSerializer, 
  CompoundValueDeserializer, DeserializeContext, SerializeContext,
  UpdateStatement, WriteColumns, WriteColumnsContext,
  Table, Connection, DatabaseNamespace,
  generate_sql_delete_where_3_columns,
  generate_sql_insert_row,
  generate_sql_delete_where_1_column,
  generate_sql_initialize_table,
};

pub struct RuleSchema {
  pub table: Table,
  pub(super) id_column: Column,
  pub(super) user_id_column: Column,
  pub(super) policy_id_column: Column,
  pub(super) position_column: Column,
  pub(super) activator_column: RuleActivatorSchema,
}

impl RuleSchema {
  pub fn new(database_namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = database_namespace
      .create_table("rules")
      .map_err(|error| error.change_context("create rule schema"))?;

    let column_namespace = table.column_namespace();

    Ok(Self {
      table,

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

      activator_column: RuleActivatorSchema
        ::new(column_namespace.create_namespace("activator"))
        .map_err(|error| error.change_context("create rule schema"))?,
    })
  }

  pub fn activator(&self) -> &RuleActivatorSchema {
    &self.activator_column
  }
  
  pub fn set_position(
    &self, 
    statement: &mut UpdateStatement,
    new_value: u32
  ) {
    statement.set(&self.position_column, &new_value);
  }
}

pub struct RuleSerializer<'a> {
  rule_adapter: &'a RuleSchema,
  rule_position: u32,
  user_id: &'a Uuid,
  policy_id: &'a Uuid,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    rule_adapter: &'a RuleSchema,
    rule_position: u32,
    user_id: &'a Uuid,
    policy_id: &'a Uuid,
  ) -> Self {
    Self {
      rule_adapter,
      rule_position,
      user_id,
      policy_id,
    }
  }
}

impl<'a> CompoundValueSerializer for RuleSerializer<'a> {
  type Input = Rule;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.rule_adapter.id_column, &value.id);
    context.serializable_scalar(&self.rule_adapter.position_column, &self.rule_position);
    context.serializable_scalar(&self.rule_adapter.user_id_column, self.user_id);
    context.serializable_compound(&self.rule_adapter.activator_column, &value.activator);
    // context.serializable_compound(&self.rule_adapter.deactivator, &value.deactivator);
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) id: Uuid,
  pub(super) position: u32,
  pub(super) activator: RuleActivator,
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
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

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedRule {
      id: context.deserializable_scalar(&self.id_column).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'id' field")
      )?,
      user_id: context.deserializable_scalar(&self.id_column).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'user_id' field")
      )?,
      policy_id: context.deserializable_scalar(&self.id_column).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'policy_id' field")
      )?,
      position: context.deserializable_scalar(&self.position_column).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'position' field")
      )?,
      activator: context.deserialize_compound(&self.activator_column).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'activator' field")
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
    context.write_compound_type(&self.activator_column)?;
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
    generate_sql_initialize_table(
      into, 
      &self.table, 
      self.columns().as_slice(),
    )
    .map_err(|error|
      error
        .change_context("generate sql code that initializes the Rules table")
    )
  }

  pub fn generate_sql_create_rule(
    &self,
    into: &mut String, 
    rule: &Rule,
    rule_position: u32,
    user_id: &Uuid,
    policy_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    let serializer = RuleSerializer::new(
      &self.rule, 
      rule_position, 
      user_id,
      policy_id,
    );

    generate_sql_insert_row(
      into, 
      &self.table, 
      &serializer, 
      rule,
    )
    .map_err(|error|
      error
        .change_context("generate sql code that inserts a Rule into the database")
    )
  }

  pub fn create_rule(
    &self,
    connection: &Connection,
    rule: &Rule, 
    rule_position: u32,
    user_id: &Uuid,
    policy_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    let mut code = String::new();

    self.generate_sql_create_rule(
      &mut code, 
      rule, 
      rule_position, 
      user_id,
      policy_id,
    )
    .map_err(|error|
      error
        .change_context("insert a Rule into the database")
    )?;

    connection.execute(&code).map_err(|error|
      error
        .change_context("insert a Rule into the database")
        .add_attachment("rule", format!("{rule:?}"))
        .add_attachment("rule position", rule_position.to_string())
        .add_attachment("enforcer id", user_id.to_string())
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
      &self.table, 
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
    rule_id: &Uuid,
    user_id: &Uuid,
    policy_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();

    self.generate_sql_delete_rule(
      &mut code, 
      rule_id,
      user_id,
      policy_id,
    );

    connection.execute(&code).map_err(|error| 
      error
        .change_context("delete Rule from database")
        .add_error("faild to execute the sql code that deletes the Rule")
        .add_attachment("rule id", rule_id.to_string())
        .add_attachment("user id", user_id.to_string())
        .add_attachment("policy id", policy_id.to_string())
    )
  }

  pub fn generate_sql_delete_rules_of_user(
    &self,
    into: &mut String, 
    user_id: &Uuid,
  ) {
    generate_sql_delete_where_1_column(
      into,
      &self.table, 
      &self.user_id_column, 
      user_id,
    );
  }

  pub fn retrieve_all(
    &self,
    connection: &Connection,
  ) -> 
    Result<Vec<NormalizedRule>, GenericError> 
  {
    connection.find_all_rows(&self.table, &self.rule).map_err(|error|
      error.change_context("retrieve all rules from database")
    )
  }

  pub fn create_updater(
    &self, 
    rule_id: &Uuid,
    policy_id: &Uuid,
    user_id: &Uuid,
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