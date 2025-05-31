use crate::database::{
  Column, CompoundValueDeserializer, CompoundValueSerializer, 
  Connection, DatabaseNamespace, DeserializeContext, 
  SerializeContext, Table, InitializeTableStatement, WriteColumns,
  WriteColumnsContext,
};

use crate::{GenericError, user};
use super::{user_screen_access_regulation, State,};

pub struct StateSchema {
  id_column: Column,
  table_metadata: Table,
  pub user: user::database::UserSchema,
  pub user_screen_access_regulation: user_screen_access_regulation::database::FeatureSchema,
  // pub user_screen_access_regulation_policy: user_screen_access_regulation::database::PolicySchema,
  // pub user_screen_access_regulation_rule: user_screen_access_regulation::database::RuleSchema,
}

impl StateSchema {
  pub fn new(database_namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table_metadata = database_namespace
      .create_table("app")
      .map_err(|error| error.change_context("create state schema"))?;
    
    let id_column = table_metadata
      .column_namespace()
      .create_column_builder("id")
      .primary()
      .build()
      .map_err(|error| error.change_context("create state schema"))?;

    let user_screen_access_regulation = user_screen_access_regulation
      ::database
      ::FeatureSchema
      ::new(
        &database_namespace
          .create_namespace("user_screen_access_regulation")?, 
        &table_metadata
          .column_namespace()
          .create_namespace("user_screen_access_regulation")
        )?;

    let user = user
      ::database
      ::UserSchema
      ::new(
        &database_namespace.create_namespace("user")?
      )?;

    Ok(StateSchema {
      table_metadata, 
      id_column, 
      user,
      user_screen_access_regulation,
    })
  }
}

impl CompoundValueSerializer for StateSchema {
  type Input = State;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.id_column, &0);
    context.serializable_compound(self.user_screen_access_regulation.singleton(), &value.user_screen_access_regulation_common_info);
  }
}

pub struct NormaizedState {
  id: u8,
  user_access: user_screen_access_regulation::CommonInfo,
}

impl Default for NormaizedState {
  fn default() -> Self {
    Self {
      id: 0,
      user_access: user_screen_access_regulation::CommonInfo::default(),
    }
  }
}

impl CompoundValueDeserializer for StateSchema {
  type Output = NormaizedState;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormaizedState {
      id: context.deserializable_scalar(&self.id_column)?,
      user_access: context.deserialize_compound(self.user_screen_access_regulation.singleton())?,
    })
  }
}

impl WriteColumns for StateSchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write_scalar_type(&self.id_column)?;
    context.write_compound_type(self.user_screen_access_regulation.singleton())?;
    Ok(())
  }
}

impl StateSchema {
  pub fn generate_sql_initialize(
    &self, 
    into: &mut String,
  ) ->
    Result<(), GenericError>
  {
    let mut statement = InitializeTableStatement::new(into, &self.table_metadata);
    statement
      .add_compound_type(self)
      .map_err(|error| 
        error
          .change_context("generate sql code that initializes the app state singleton table")
      )?;

    self
      .user_screen_access_regulation
      .generate_sql_initialize(into)
      .map_err(|error| 
        error
          .change_context("generate sql code that initializes the app state singleton table")
      )?;

    Ok(())
  }

  pub fn initialize(
    &self,
    connection: &Connection
  ) -> 
    Result<(), GenericError>
  {
    let mut code = String::new();
    self
      .generate_sql_initialize(&mut code)
      .map_err(|error|
        error.change_context("initialize database schema")
      )
  }
  
  fn load_normalized_state(
    &self,
    connection: &Connection,
  ) -> 
    Result<NormaizedState, GenericError>
  {
    connection.find_some_row(
      &self.table_metadata, 
      self,
    )
  }

  fn load_denormalized_state(
    &self, 
    connection: &Connection,
  ) -> 
    Result<State, GenericError> 
  {
    let users_in_normalized_form = self
      .user
      .retrieve_all_normalized(connection)
      .map_err(|error| 
        error
          .change_context("load all users from the database")
          .change_context("load denormalized state")
      )?;
    
    let user_screen_access_regulation_policies_in_normalized_form = self
      .user_screen_access_regulation
      .policy
      .load_all_normalized_policies(connection)
      .map_err(|error| 
        error
          .change_context("load all user screen access regulation policies from the database in normalized form")
          .change_context("load denormalized state")
      )?;

        
    let user_screen_access_regulation_rules_in_normalized_form = self
      .user_screen_access_regulation
      .rule
      .load_all_rules_normalized(connection)
      .map_err(|error| 
        error
          .change_context("load all user screen access regulation rules from the database in normalized form")
          .change_context("load denormalized state")
      )?;

    let state_in_normalized_form = self
      .load_normalized_state(connection)
      .map_err(|error| 
        error
          .change_context("load state from the database in normalized form")
          .change_context("load denormalized state")
      )?;

    let users_in_denormalized_form = users_in_normalized_form
      .into_iter()
      .map(|user| user.denormalize(
        &user_screen_access_regulation_policies_in_normalized_form, 
        &user_screen_access_regulation_rules_in_normalized_form,
      ))
      .collect();

    let denormalized_state = State {
      users: users_in_denormalized_form,
      user_screen_access_regulation_common_info: state_in_normalized_form.user_access,
    };

    Ok(denormalized_state)
  }

  pub fn load(
    &self, 
    connection: &Connection,
  ) -> 
    Result<State, GenericError>
  {
    self.load_denormalized_state(connection)
  }
}