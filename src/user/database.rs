use super::{
  UserName, User
};

use crate::{
  user_screen_access_regulation, GenericError, Uuid,
  OperatingSystemPassword, OperatingSystemUserId, OperatingSystemUsername,
};

use crate::database::{
  ScalarFieldSpecification, WriteColumns, WriteColumnsContext,
  UpdateStatement, CompoundValueSerializer, SerializeContext,
  CompoundValueDeserializer, CompoundValueDeserializerContext, Connection,
  DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext,
  ColumnValue, CollectionSpecfication, DatabaseNamespace, InitializeTableStatement,
  generate_sql_add_row,
};

impl SerializableScalarValue for UserName {
  fn serialize_into(&self, context: SerializeScalarValueContext) {
    context.write_string(self.as_ref());
  }
}

impl DeserializableScalarValue for UserName {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_string()
      .and_then(UserName::new)
      .map_err(|error| error.change_context("deserialize user name"))
  }
}

pub struct UserSchema {
  table_metadata: CollectionSpecfication,
  id_column: ScalarFieldSpecification,
  name_column: ScalarFieldSpecification,
  operating_system_user_id_column: ScalarFieldSpecification,
  operating_system_username_column: ScalarFieldSpecification,
  operating_system_password_column: ScalarFieldSpecification,
  pub screen_access_regulator_type: user_screen_access_regulation::database::RegulatorSchema,
}

impl UserSchema {
  pub fn new(database_namespace: &DatabaseNamespace) -> Result<Self, GenericError> {
    let table_metadata = database_namespace
      .create_table("users")
      .map_err(|error| error.change_context("create user schema"))?;

    Ok(Self {      
      id_column: table_metadata.column_namespace()
        .create_column_builder("id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create user schema"))?,

      name_column: table_metadata.column_namespace()
        .create_column_builder("name")
        .build()
        .map_err(|error| error.change_context("create user schema"))?,

      operating_system_user_id_column: table_metadata.column_namespace()
        .create_column_builder("operating_system_user_id")
        .build()
        .map_err(|error| error.change_context("create user schema"))?,

      operating_system_username_column: table_metadata.column_namespace()
        .create_column_builder("operating_system_username")
        .build()
        .map_err(|error| error.change_context("create user schema"))?,

      operating_system_password_column: table_metadata.column_namespace()
        .create_column_builder("operating_system_password")
        .build()
        .map_err(|error| error.change_context("create user schema"))?,

      screen_access_regulator_type: user_screen_access_regulation
        ::database
        ::RegulatorSchema
        ::new(&table_metadata.column_namespace().create_namespace("screen_access_regulator"))        
        .map_err(|error| error.change_context("create user schema"))?,

      table_metadata,
    })
  }

  pub fn set_name(
    &self, 
    modifications: &mut CollectionItemModifications, 
    new_value: &UserName,
  ) {
    modifications.modify_scalar_field(&self.name_column, new_value);
  }
}

impl CompoundValueSerializer for UserSchema {
  type Input = User;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) {
    context.serializable_scalar(&self.id_column, &value.id);  
    context.serializable_scalar(&self.name_column, &value.name);  
    context.serializable_scalar(&self.operating_system_user_id_column, &value.operating_system_user_id);  
    context.serializable_scalar(&self.operating_system_username_column, &value.operating_system_username);  
    context.serializable_scalar(&self.operating_system_password_column, &value.operating_system_password);  
    context.serializable_compound(&self.screen_access_regulator_type, &value.screen_access_regulator);  
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

impl NormalizedUser {
  pub fn denormalize(
    self, 
    user_screen_access_regulation_policies: &Vec<user_screen_access_regulation::database::NormalizedPolicy>,
    user_screen_access_regulation_rules: &Vec<user_screen_access_regulation::database::NormalizedRule>,
  ) -> User {
    User {
      id: self.id,
      name: self.name,
      operating_system_user_id: self.operating_system_user_id,
      operating_system_username: self.operating_system_username,
      operating_system_password: self.operating_system_password,
      screen_access_regulator: self.screen_access_regulator.denormalize(
        user_screen_access_regulation_policies,
        user_screen_access_regulation_rules,
      ),
    }
  }
}

impl CompoundValueDeserializer for UserSchema {
  type Output = NormalizedUser;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedUser {
      id: context
        .deserializable_scalar(&self.id_column)
        .map_err(|error| error
          .change_context("deserialize normalized user")
          .add_error("failed to deserialize the 'id' field")
        )?,

      name: context
        .deserializable_scalar(&self.name_column)
        .map_err(|error| error
          .change_context("deserialize normalized user")
          .add_error("failed to deserialize the 'name' field")
        )?,

      operating_system_user_id: context
        .deserializable_scalar(&self.operating_system_user_id_column)
        .map_err(|error| error
          .change_context("deserialize normalized user")
          .add_error("failed to deserialize the 'operating_system_user_id' field")
        )?,

      operating_system_username: context
        .deserializable_scalar(&self.operating_system_username_column)
        .map_err(|error| error
          .change_context("deserialize normalized user")
          .add_error("failed to deserialize the 'operating_system_username' field")
        )?,

      operating_system_password: context
        .deserializable_scalar(&self.operating_system_password_column)
        .map_err(|error| error
          .change_context("deserialize normalized user")
          .add_error("failed to deserialize the 'operating_system_password' field")
        )?,

      screen_access_regulator: context
        .deserialize_compound(&self.screen_access_regulator_type)
        .map_err(|error| error
          .change_context("deserialize normalized user")
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
    let mut statement = InitializeTableStatement::new(into, &self.table_metadata);
    statement
      .add_compound_type(self)
      .map_err(|error| error.change_context("generate sql code that initializes everything related to the users table"))
  }

  pub fn generate_sql_add(
    &self,
    into: &mut String,
    user: &User,
  ) -> 
    Result<(), GenericError>
  {
    generate_sql_add_row(into, &self.table_metadata, self, user)
    .map_err(|error| error
      .change_context("generate sql that adds a user to the users table")
      .add_attachment("user", format!("{user:?}"))
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
      .map_err(|error| error.change_context("add a user to the users table"))
  }

  pub fn retrieve_all_normalized(
    &self,
    connection: &Connection
  ) -> 
    Result<Vec<NormalizedUser>, GenericError>
  {
    connection
      .find_all_rows(&self.table_metadata, self)
      .map_err(|error| error.change_context("retrieve all users from the users table"))
  }

  pub fn create_updater(&self, user_id: &Uuid) -> UpdateStatement {
    UpdateStatement::new_given_one_where_columns(
      &self.id_column, 
      user_id
    )
  }
}