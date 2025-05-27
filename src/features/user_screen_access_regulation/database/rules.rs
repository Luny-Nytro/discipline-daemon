use super::{
  Table, RuleSchema, GenericError, Rule, Uuid, NormalizedRule,
  RuleSerializer, Connection, DatabaseNamespace,
  generate_sql_delete_where_3_columns, UpdateStatement,
  generate_sql_insert_row,
  generate_sql_delete_where_1_column,
  generate_sql_initialize_table,
};

pub struct RuleTableSchema {
  table: Table,
  rule: RuleSchema,
}

impl RuleTableSchema {
  pub fn new(namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = namespace.create_table("rules")?;
    let rule = RuleSchema::new(table.column_namespace())?;

    Ok(Self {
      table,
      rule,
    })
  }

  pub fn generate_sql_initialize(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError> 
  {
    generate_sql_initialize_table(
      into, 
      &self.table, 
      self.rule.columns().as_slice(),
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
      &self.rule.id, 
      rule_id,
      &self.rule.user_id,
      user_id,
      &self.rule.policy_id,
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
      &self.rule.user_id, 
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

  pub fn create_update_statement(
    &self, 
    rule_id: &Uuid,
    user_id: &Uuid,
    policy_id: &Uuid,
  ) -> 
    UpdateStatement
  {
    UpdateStatement::new_given_three_where_columns(
      &self.rule.id, 
      rule_id, 
      &self.rule.policy_id, 
      policy_id, 
      &self.rule.user_id, 
      user_id,
    )
  }
}