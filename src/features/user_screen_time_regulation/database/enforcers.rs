use super::{
  Table, EnforcerAdapter, GenericError, Regulator, Connection,
  Uuid, NormalizedEnforcer, DatabaseNamespace, RuleTableSchema,
  generate_sql_initialize_table,
  generate_create_row_statement,
  generate_sql_where_1_column,
  generate_update_column_where_column_statement,
};

pub struct EnforcersAdapter {
  table: Table,
  enforcer: EnforcerAdapter,
}

impl EnforcersAdapter {
  pub(super) fn new(namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = namespace.create_table("enforcers")?;
    let enforcer = EnforcerAdapter::new(table.column_namespace())?;

    Ok(Self {
      table,
      enforcer,
    })
  }

  pub(super) fn generate_ensure_table_created_sql(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError> 
  {
    generate_sql_initialize_table(
      into, 
      &self.table, 
      &self.enforcer.columns(),
    )
  }
  
  fn generate_create_enforcer_sql(
    &self,
    into: &mut String, 
    enforcer: &Regulator,
  ) -> 
    Result<(), GenericError> 
  {
    generate_create_row_statement(
      into, 
      &self.table, 
      &self.enforcer, 
      enforcer,
    )
  }
  
  pub fn create_enforcer(
    &self,
    connection: &Connection, 
    enforcer: &Regulator,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
    
    self.generate_create_enforcer_sql(&mut code, enforcer).map_err(|error|
      error.change_context("Failed to create an Enforcer: Failed to generate SQL")
        .add_attachment("enforcer", format!("{enforcer:?}"))
    )?;
  
    connection.execute(&code).map_err(|error| 
      error.change_context("Failed to create an Enforcer: Sqlite error")
        .add_attachment("enforcer", format!("{enforcer:?}"))
    )
  }
  
  pub(super) fn generate_delete_enforcer_by_id_statement(
    &self,
    into: &mut String, 
    enforcer_id: &Uuid,
  ) -> Result<(), GenericError> {
    generate_sql_where_1_column(
      into,
      &self.table, 
      &self.enforcer.id, 
      enforcer_id,
    )
  }
  
  /// Generates a statement that updates the 'is_enforcing_enabled' column
  /// of the Enforcer row whose id matches 'id'.
  fn generate_update_is_enforcing_enabled_by_id_statement(
    &self,
    into: &mut String, 
    enforcer_id: &Uuid, 
    new_value: bool,
  ) -> 
    Result<(), GenericError> 
  {
    generate_update_column_where_column_statement(
      into, 
      &self.table, 
      &self.enforcer.is_enforcing_enabled, 
      &new_value, 
      &self.enforcer.id, 
      enforcer_id,
    )
  }
  
  pub fn update_enforcer_is_user_access_blocked(
    &self,
    connection: &Connection, 
    enforcer_id: &Uuid,
    new_value: bool,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
  
    self.generate_update_enforcer_is_user_access_blocked_by_id_statement(
      &mut code, 
      enforcer_id, 
      new_value,
    )?;
  
    connection.execute(&code).map_err(|error| 
      error.change_context("Failed to update the 'is_user_access_blocked' field of an Enforcer: Sqlite error")
        .add_attachment("rule user enforcer id", enforcer_id.to_string())
        .add_attachment("new 'is_user_access_blocked' value", new_value.to_string())
    )
  }
  
  /// Generates a statement that updates the 'is_user_access_blocked' column
  /// of the Enforcer row whose id matches 'id'.
  fn generate_update_enforcer_is_user_access_blocked_by_id_statement(
    &self, 
    into: &mut String, 
    enforcer_id: &Uuid, 
    new_value: bool,
  ) -> 
    Result<(), GenericError> 
  {
    generate_update_column_where_column_statement(
      into, 
      &self.table, 
      &self.enforcer.is_user_access_blocked, 
      &new_value, 
      &self.enforcer.id, 
      enforcer_id,
    )
  }
  
  pub fn get_all(
    &self,
    connection: &Connection,
  ) 
    -> Result<Vec<NormalizedEnforcer>, GenericError> 
  {
    connection.find_all_rows(&self.table, &self.enforcer).map_err(|error| 
      error.change_context("Failed to get all Enforcers")
    )
  }

  pub(super) fn generate_update_after_synchronize_sql(
    &self,
    into: &mut String,
    rule_adapter: &RuleTableSchema,
    enforcer: &Regulator,
  ) -> 
    Result<(), GenericError>
  {
    for rule in &enforcer.rules {
      rule_adapter.generate_update_after_synchronize_sql(
        into,
        rule,
      )?;
    }

    Ok(())
  }

  pub(super) fn update_is_enforcing_enabled(
    &self, 
    connection: &Connection,
    enforcer_id: &Uuid,
    new_value: bool,
  ) -> 
    Result<(), GenericError>
  {
    let mut code = String::new();
    
    self.generate_update_is_enforcing_enabled_by_id_statement(
      &mut code, 
      enforcer_id, 
      new_value,
    )?;

    connection.execute(&code)?;

    Ok(())
  }
}
// generate_sql_update_enforcer_after_synchronization