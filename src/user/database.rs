use super::{
  UserName, User
};

use crate::{
  user_screen_access_regulation, GenericError, Uuid,
  OperatingSystemPassword, OperatingSystemUserId, OperatingSystemUsername,
};

use crate::database::{
  Column, WriteColumns, WriteColumnsContext,
  UpdateStatement, CompoundValueSerializer, SerializeContext,
  CompoundValueDeserializer, DeserializeContext, Connection,
  DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext,
  ColumnValue, Table, DatabaseNamespace, InitializeTableStatement,
  generate_sql_insert_row
};

impl SerializableScalarValue for UserName {
  fn serialize_into(&self, context: SerializeScalarValueContext) {
    context.as_string(self.as_ref());
  }
}

impl DeserializableScalarValue for UserName {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value
      .as_string()
      .and_then(UserName::new)
      .map_err(|error| error.change_context("deserialize UserName"))
  }
}

pub struct UserSchema {
  table: Table,
  id_column: Column,
  name_column: Column,
  operating_system_user_id_column: Column,
  operating_system_username_column: Column,
  operating_system_password_column: Column,
  pub screen_access_regulator_type: user_screen_access_regulation::database::RegulatorSchema,
}

impl UserSchema {
  pub fn new(database_namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = database_namespace
      .create_table("users")
      .map_err(|error| error.change_context("create UsersSchema"))?;

    let user = UserSchema::new(table.column_namespace())
      .map_err(|error| error.change_context("create UsersSchema"))?;

    Ok(Self {
      table,
      
      id_column: table.column_namespace()
        .create_column_builder("id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      name_column: table.column_namespace()
        .create_column_builder("name")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      operating_system_user_id_column: table.column_namespace()
        .create_column_builder("operating_system_user_id")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      operating_system_username_column: table.column_namespace()
        .create_column_builder("operating_system_username")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      operating_system_password_column: table.column_namespace()
        .create_column_builder("operating_system_password")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      screen_access_regulator_type: user_screen_access_regulation
        ::database
        ::RegulatorSchema
        ::new(&table.column_namespace().create_namespace("screen_access_regulator"))        
        .map_err(|error| error.change_context("create UserSchema"))?,
    })
  }

  pub fn set_name(
    &self, 
    statement: &mut UpdateStatement, 
    new_value: &UserName,
  ) {
    statement.set(&self.name_column, new_value);
  }
}

impl CompoundValueSerializer for UserSchema {
  type Input = UserSchema;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.id_column, &value.id_column);  
    context.serializable_scalar(&self.name_column, &value.name_column);  
    context.serializable_scalar(&self.operating_system_user_id_column, &value.operating_system_user_id_column);  
    context.serializable_scalar(&self.operating_system_username_column, &value.operating_system_username_column);  
    context.serializable_scalar(&self.operating_system_password_column, &value.operating_system_password_column);  
    context.serializable_compound(&self.screen_access_regulator_type, &value.screen_access_regulator_type);  
  }
}

pub struct NormalizedUser {
  id: Uuid,
  name: UserName,
  operating_system_user_id: OperatingSystemUserId,
  operating_system_username: OperatingSystemUsername,
  operating_system_password: OperatingSystemPassword,
  screen_access_regulator: user_screen_access_regulation
    ::database
    ::NormalizedRegulator
}

impl CompoundValueDeserializer for UserSchema {
  type Output = NormalizedUser;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedUser {
      id: context
        .deserializable_scalar("id")
        .map_err(|error| error
          .change_context("deserialize NormalizedUser")
          .add_error("failed to deserialize the 'id' field")
        )?,

      name: context
        .deserializable_scalar("name")
        .map_err(|error| error
          .change_context("deserialize NormalizedUser")
          .add_error("failed to deserialize the 'name' field")
        )?,

      operating_system_user_id: context
        .deserializable_scalar("operating_system_user_id")
        .map_err(|error| error
          .change_context("deserialize NormalizedUser")
          .add_error("failed to deserialize the 'operating_system_user_id' field")
        )?,

      operating_system_username: context
        .deserializable_scalar("operating_system_username")
        .map_err(|error| error
          .change_context("deserialize NormalizedUser")
          .add_error("failed to deserialize the 'operating_system_username' field")
        )?,

      operating_system_password: context
        .deserializable_scalar("operating_system_password")
        .map_err(|error| error
          .change_context("deserialize NormalizedUser")
          .add_error("failed to deserialize the 'operating_system_password' field")
        )?,

      screen_access_regulator: context
        .deserialize_compound("screen_access_regulator")
        .map_err(|error| error
          .change_context("deserialize NormalizedUser")
          .add_error("failed to deserialize the 'screen_access_regulator' field")
        )?,
    })
  }
}

impl WriteColumns for UserSchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write_scalar_type(&self.id_column)?;
    context.write_scalar_type(&self.name_column)?;
    context.write_scalar_type(&self.operating_system_user_id_column)?;
    context.write_scalar_type(&self.operating_system_username_column)?;
    context.write_scalar_type(&self.operating_system_password_column)?;
    context.write_compound_type(&self.screen_access_regulator_type)?;
    Ok(())
  }
}

impl UserSchema {
  pub fn generate_sql_initialize(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError>
  {
    let mut statement = InitializeTableStatement::new(&self.table);
    statement
      .add_compound_type(&self.user)
      .map_err(|error| error.change_context("generate sql code that initializes everything related to the 'users' table in the database"))
  }

  pub fn generate_sql_add(
    &self,
    into: &mut String,
    user: &User,
  ) -> 
    Result<(), GenericError>
  {
    generate_sql_insert_row(
      into, 
      &self.table, 
      &self, 
      user,
    )
    .map_err(|error| error
      .change_context("generate sql that adds a user to the 'users' table")
    )
  }

  pub fn add(
    &self,
    connection: &Connection,
    user: &User,
  ) ->
    Result<(), GenericError>
  {
    let mut sql = String::new();
    self
      .generate_sql_add(&mut sql, user)
      .and(connection.execute(&sql))
      .map_err(|error| error.change_context("add a user to the 'users' table"))
  }


  pub fn create_updater(&self, user_id: &Uuid) -> UpdateStatement {
    UpdateStatement::new_given_one_where_columns(
      &self.id_column, 
      user_id
    )
  }
}