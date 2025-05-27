use super::{
  UserName, User
};

use crate::{
  user_screen_access_regulation, GenericError, Uuid,
  OperatingSystemPassword, OperatingSystemUserId, OperatingSystemUsername,
};

use crate::database::{
  Column, ColumnNamespace, WriteColumns, WriteColumnsContext,
  UpdateStatement, CompoundValueSerializer, SerializeContext,
  CompoundValueDeserializer, DeserializeContext,
  DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext,
  ColumnValue, Table, DatabaseNamespace,
  
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
  id: Column,
  name: Column,
  operating_system_user_id: Column,
  operating_system_username: Column,
  operating_system_password: Column,
  screen_access_regulator: user_screen_access_regulation::database::RegulatorSchema,
}

impl UserSchema {
  pub fn new(
    column_namespace: &ColumnNamespace,
  ) ->
    Result<(), GenericError>
  {
    Ok(Self {
      id: column_namespace
        .create_column_builder("id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      name: column_namespace
        .create_column_builder("name")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      operating_system_user_id: column_namespace
        .create_column_builder("operating_system_user_id")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      operating_system_username: column_namespace
        .create_column_builder("operating_system_username")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      operating_system_password: column_namespace
        .create_column_builder("operating_system_password")
        .build()
        .map_err(|error| error.change_context("create UserSchema"))?,

      screen_access_regulator: user_screen_access_regulation
        ::database
        ::RegulatorSchema
        ::new(&column_namespace.create_namespace("screen_access_regulator"))        
        .map_err(|error| error.change_context("create UserSchema"))?,
    })
  }

  pub fn set_name(
    &self, 
    statement: &mut UpdateStatement, 
    new_value: &UserName,
  ) {
    statement.set(&self.name, new_value);
  }
}

impl CompoundValueSerializer for UserSchema {
  type Input = UserSchema;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.id, &value.id);  
    context.serializable_scalar(&self.name, &value.name);  
    context.serializable_scalar(&self.operating_system_user_id, &value.operating_system_user_id);  
    context.serializable_scalar(&self.operating_system_username, &value.operating_system_username);  
    context.serializable_scalar(&self.operating_system_password, &value.operating_system_password);  
    context.serializable_compound(&self.screen_access_regulator, &value.screen_access_regulator);  
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
    Ok(Self {
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
    context.write(&self.id)?;
    context.write(&self.name)?;
    context.write(&self.operating_system_user_id)?;
    context.write(&self.operating_system_username)?;
    context.write(&self.operating_system_password)?;
    context.write_compound_type(&self.screen_access_regulator)?;
    Ok(())
  }
}

pub struct UsersSchema {
  table: Table,
  user: UserSchema,
}

impl UsersSchema {
  pub fn new(database_namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table = database_namespace
      .create_table("users")
      .map_err(|error| error.change_context("create UsersSchema"))?;

    let user = UserSchema::new(table.column_namespace())
      .map_err(|error| error.change_context("create UsersSchema"))?;

    Ok(Self {
      user,
      table,
    })
  }

  pub fn generate_sql_initialize(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError>
  {

  }
}