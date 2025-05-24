use crate::database::{generate_ensure_row_create_statement, generate_sql_initialize_table, Column, CompoundValueDeserializer, CompoundValueSerializer, Connection, DatabaseNamespace, DeserializeContext, SerializeContext, Table};
use crate::GenericError;
use super::{user_screen_time_regulation, State};

pub struct StateAdapter {
  id: Column,
  table: Table,
  pub user_access: user_screen_time_regulation::database::FeatureAdapter,
}

pub struct NormaizedState {
  id: u8,
  user_access: user_screen_time_regulation::database::NormalizedFeature,
}

impl Default for NormaizedState {
  fn default() -> Self {
    Self {
      id: 0,
      user_access: user_screen_time_regulation::database::NormalizedFeature::default(),
    }
  }
}

impl CompoundValueSerializer for StateAdapter {
  type Input = NormaizedState;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.id, &value.id);
    context.serializable_compound(&self.user_access, &value.user_access);
  }
}

impl CompoundValueDeserializer for StateAdapter {
  type Output = NormaizedState;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormaizedState {
      id: context.deserializable_scalar(&self.id)?,
      user_access: context.deserialize_compound(&self.user_access)?,
    })
  }
}

impl StateAdapter {
  pub fn new(database_namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = database_namespace.create_table("app")?;
    let column_namespace = table.column_namespace();
    
    let id = column_namespace
      .create_column_builder("id")
      .primary()
      .build()?;

    let user_access = user_screen_time_regulation::database::FeatureAdapter::new(
      database_namespace, 
      &column_namespace.create_namespace("user_access"),
    )?;

    Ok(StateAdapter {
      id, 
      table, 
      user_access,
    })
  }

  fn columns(&self) -> Vec<&Column> {
    let mut columns = vec![&self.id];
    columns.extend_from_slice(&self.user_access.columns());
    columns
  }

  fn columns_iterator(&self) -> impl Iterator<Item = &Column> {
    [&self.id].into_iter().chain(self.user_access.columns_iterator())
  }

  fn generate_initialize_statements(&self, sql: &mut String) ->
    Result<(), GenericError>
  {
    generate_sql_initialize_table(
      sql, 
      &self.table, 
      &self.columns(),
    )?;

    let default = NormaizedState::default();
    generate_ensure_row_create_statement(
      sql, 
      &self.table, 
      self, 
      &default,
    )?;

    self.user_access.generate_initialize_sql(sql)?;

    Ok(())
  }

  pub fn initialize(
    &self,
    connection: &Connection,
  ) ->
    Result<(), GenericError>
  {
    let mut code = String::new();

    self.generate_initialize_statements(&mut code)?;

    connection.execute(&code).map_err(|error|
      error.change_context("Failed to initialize app state database schema")
    )?;

    Ok(())
  }

  fn load_normalized_state(
    &self,
    connection: &Connection,
  ) -> 
    Result<NormaizedState, GenericError>
  {
    connection.find_some_row(
      &self.table, 
      self,
    )
  }

  fn load_denormalized_state(
    &self, 
    connection: &Connection,
  ) -> 
    Result<State, GenericError> 
  {
    let normalized_app = self.load_normalized_state(connection)?;

    let denormalized_app = State {
      user_access: self.user_access.finalize(connection, normalized_app.user_access)?
    };

    Ok(denormalized_app)
  }

  pub fn load(
    &self, 
    connection: &Connection,
  ) -> 
    Result<State, GenericError>
  {
    self.load_denormalized_state(connection)
  }

  pub fn generate_update_after_synchronize_sql(
    &self,
    into: &mut String,
    state: &mut State,
  ) ->
    Result<(), GenericError>
  {
    self.user_access.generate_update_after_synchronize_sql(
      into, 
      &state.user_access,
    )?;

    Ok(())
  }
}
