use super::{
  Table, RuleAdapter, GenericError, Rule, Uuid, RuleNormalized,
  RuleSerializer, Connection, TimeRange, UpdateStatementSetClause,
  WeekdayRange, Duration, DatabaseNamespace,
  generate_create_row_statement,
  generate_delete_where_column_statement,
  generate_ensure_table_created_statement,
  generate_update_where_column_statement_given_set_clause,
};

pub struct RuleTableAdapter {
  table: Table,
  rule: RuleAdapter,
}

impl RuleTableAdapter {
  pub(super) fn new(namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = namespace.create_table("rules")?;
    let rule = RuleAdapter::new(table.column_namespace())?;

    Ok(Self {
      table,
      rule,
    })
  }

  /// Generates SQL that creates the Rules table if it is not already created.
  pub(super) fn generate_ensure_table_created_sql(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError> 
  {
    generate_ensure_table_created_statement(
      into, 
      &self.table, 
      self.rule.columns().as_slice(),
    )
  }

  /// Generates SQL that inserts a new Rule row into the table.
  fn generate_create_rule_statement(
    &self,
    into: &mut String, 
    rule: &Rule,
    position: u32,
    enforcer_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    let serializer = RuleSerializer::new(
      &self.rule, 
      position, 
      enforcer_id,
    );

    generate_create_row_statement(
      into, 
      &self.table, 
      &serializer, 
      rule,
    )
  }

  /// Inserts a new Rule row into the table.
  pub fn create_rule(
    &self,
    connection: &Connection,
    rule: &Rule, 
    rule_position: u32,
    enforcer_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    let mut code = String::new();

    self.generate_create_rule_statement(
      &mut code, 
      rule, 
      rule_position, 
      enforcer_id,
    )?;

    connection.execute(&code).map_err(|error|
      error.change_context("Failed to create a Rule: Sqlite error")
        .add_attachment("rule", format!("{rule:?}"))
        .add_attachment("rule position", rule_position.to_string())
        .add_attachment("enforcer id", enforcer_id.to_string())
    )
  }

  /// Generates SQL that deletes a Rule row whose 'id' column matches the given id.
  fn generate_delete_rule_by_id_statement(
    &self,
    into: &mut String, 
    rule_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    generate_delete_where_column_statement(
      into,
      &self.table, 
      &self.rule.id, 
      rule_id,
    )
  }

  /// Deletes a Rule row whose 'id' column matches the given id.
  pub fn delete_rule(
    &self,
    connection: &Connection, 
    rule_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();

    self.generate_delete_rule_by_id_statement(
      &mut code, 
      rule_id,
    )?;

    connection.execute(&code).map_err(|error| 
      error.change_context("Failed to delete a Rule")
        .add_attachment("rule id", rule_id.to_string())
    )
  }

  /// Generates SQL that deletes EVERY Rule row whose 'enforcer_id' column
  /// matches the provided 'enforcer_id'.
  pub(super) fn generate_delete_rules_by_enforcer_id_statement(
    &self,
    into: &mut String, 
    enforcer_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    generate_delete_where_column_statement(
      into,
      &self.table, 
      &self.rule.enforcer_id, 
      enforcer_id,
    )
  }

  /// Selects all Rule rows.
  pub fn get_all(
    &self,
    connection: &Connection,
  ) -> 
    Result<Vec<RuleNormalized>, GenericError> 
  {
    connection.find_all_rows(&self.table, &self.rule).map_err(|error|
      error.change_context("Failed to find all rules")
    )
  }

  /// Generates SQL that replaces the TimeRange of the 'InTimeRange'
  /// variant of the 'RuleActivator' of the Rule row whose id matches
  /// the given 'rule_id' with the provided 'new_time_range.
  fn generate_replace_activator_in_time_range_by_rule_id_statement(
    &self,
    into: &mut String,
    rule_id: &Uuid,
    new_time_range: &TimeRange,
  ) -> 
    Result<(), GenericError> 
  {
    let mut update_statement_set_clause = UpdateStatementSetClause::new();

    // TODO: Add more context to the error.
    self.rule.activator.in_time_range_update_range(
      &mut update_statement_set_clause, 
      new_time_range,
    )?;

    generate_update_where_column_statement_given_set_clause(
      into, 
      &self.table, 
      &self.rule.id, 
      rule_id, 
      update_statement_set_clause,
    )
  }

  /// Replaces the TimeRange of the 'InTimeRange' variant of the 
  /// 'RuleActivator' of the Rule row whose id matches the provided
  /// 'rule_id' with the provided 'new_time_range'.
  pub fn replace_rule_activator_in_time_range_by_rule_id(
    &self,
    connection: &Connection,
    rule_id: &Uuid,
    new_time_range: &TimeRange,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();

    self.generate_replace_activator_in_time_range_by_rule_id_statement(
      &mut code, 
      rule_id, 
      new_time_range,
    )?;

    connection.execute(&code).map_err(|error|
      error.change_context("Failed to replace the TimeRange of the InTimeRange variant of a RuleActivator")
        .add_attachment("new time range", format!("{new_time_range:?}"))
        .add_attachment("rule id", rule_id.to_string())
    )
  }

  fn generate_replace_activator_in_weekday_range_by_rule_id_statement(
    &self,
    into: &mut String,
    rule_id: &Uuid,
    new_weekday_range: &WeekdayRange,
  ) -> 
    Result<(), GenericError> 
  {
    let mut update_statement_set_clause = UpdateStatementSetClause::new();

    // TODO: Add more context to the error.
    self.rule.activator.in_weekday_range_update_range(
      &mut update_statement_set_clause, 
      new_weekday_range,
    )?;

    generate_update_where_column_statement_given_set_clause(
      into, 
      &self.table, 
      &self.rule.id, 
      rule_id, 
      update_statement_set_clause,
    )
  }

  pub fn replace_rule_activator_in_weekday_range(
    &self,
    connection: &Connection,
    rule_id: &Uuid,
    new_weekday_range: &WeekdayRange,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();

    self.generate_replace_activator_in_weekday_range_by_rule_id_statement(
      &mut code, 
      rule_id, 
      new_weekday_range,
    )?;

    connection.execute(&code).map_err(|error|
      error.change_context("Failed to replace the WeekdayRange of the InWeekdayRange variant of a RuleActivator")
        .add_attachment("new time range", format!("{new_weekday_range:?}"))
        .add_attachment("rule id", rule_id.to_string())
    )
  }

  fn generate_update_deactivator_for_duration_remaining_duration_by_rule_id_statement(
    &self,
    into: &mut String,
    rule_id: &Uuid,
    new_remaining_duration: &Duration,
  ) -> 
    Result<(), GenericError>
  {
    let mut update_statement_set_clause = UpdateStatementSetClause::new();

    // TODO: Add more context to the error.
    self.rule.deactivator.update_remaining_duration(
      &mut update_statement_set_clause, 
      new_remaining_duration,
    )?;

    generate_update_where_column_statement_given_set_clause(
      into, 
      &self.table, 
      &self.rule.id, 
      rule_id, 
      update_statement_set_clause,
    )
  }

  pub fn update_deactivator_remaining_duration(
    &self,
    connection: &Connection,
    rule_id: &Uuid,
    new_remaining_duration: &Duration
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();

    self.generate_update_deactivator_for_duration_remaining_duration_by_rule_id_statement(
      &mut code, 
      rule_id, 
      new_remaining_duration,
    )?;

    connection.execute(&code).map_err(|error|
      error.change_context("Failed to update the 'remaining_duration' of a RuleDeactivator")
        .add_attachment("new remaining duration", new_remaining_duration.to_string())
        .add_attachment("rule id", rule_id.to_string())
    )
  }

  pub(super) fn generate_update_after_synchronize_sql(
    &self, 
    into: &mut String,
    rule: &Rule,
  ) -> 
    Result<(), GenericError>
  {
    let update_statement_set_clause = UpdateStatementSetClause::new();

    generate_update_where_column_statement_given_set_clause(
      into, 
      &self.table, 
      &self.rule.id, 
      &rule.id, 
      update_statement_set_clause
    )
  }
}