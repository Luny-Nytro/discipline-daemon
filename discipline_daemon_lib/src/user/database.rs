use super::{
  UserName, User
};

use crate::{
  user_screen_access_regulation, GenericError, Uuid,
  OperatingSystemPassword, OperatingSystemUserId, OperatingSystemUsername,
};

use crate::database::{
  ScalarFieldSpecification, CollectionItemDefiner, Database,
  CompoundValueSerializer, CompoundValueSerializerContext,
  CompoundValueDeserializer, CompoundValueDeserializerContext,
  FromScalarValue, IntoScalarValue, SerializeScalarValueContext,
  ScalarValue, CollectionSpecification, Namespace, CollectionItemModificationsDraft,
  CollectionItemMatcher,
};

impl IntoScalarValue for UserName {
  fn write_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.write_string(self.as_ref())
  }
}

impl FromScalarValue for UserName {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_string()
      .and_then(UserName::new)
      .map_err(|error| error.change_context("deserializing UserName"))
  }
}

pub struct Specification {
  pub collection_specification: CollectionSpecification,
  id_field_specification: ScalarFieldSpecification,
  name_field_specification: ScalarFieldSpecification,
  operating_system_user_id_field_specification: ScalarFieldSpecification,
  operating_system_username_field_specification: ScalarFieldSpecification,
  operating_system_password_field_specification: ScalarFieldSpecification,
  screen_access_regulator_field_specification: user_screen_access_regulation::database::RegulatorSpecification,
}

impl Specification {
  pub fn new(namespace: &mut Namespace) -> Result<Self, GenericError> {
    let mut fields_namespace = CollectionItemDefiner::new();

    let id_field_specification = fields_namespace
    .define_primary_scalar_field("Id")
    .build()
    .map_err(|error| error.change_context("creating UserSpecification"))?;

  let name_field_specification = fields_namespace
    .scalar_field_specification("Name")
    .build()
    .map_err(|error| error.change_context("creating UserSpecification"))?;

  let operating_system_user_id_field_specification = fields_namespace
    .scalar_field_specification("OperatingSystemUserId")
    .build()
    .map_err(|error| error.change_context("creating UserSpecification"))?;

  let operating_system_username_field_specification = fields_namespace
    .scalar_field_specification("OperatingSystemUsername")
    .build()
    .map_err(|error| error.change_context("creating UserSpecification"))?;

  let operating_system_password_field_specification = fields_namespace
    .scalar_field_specification("OperatingSystemPassword")
    .build()
    .map_err(|error| error.change_context("creating UserSpecification"))?;

    let screen_access_regulator_field_specification = user_screen_access_regulation
      ::database
      ::RegulatorSpecification
      ::new(&mut fields_namespace.compound_field_specification("ScreenAccessRegulator")?)        
      .map_err(|error| error.change_context("creating UserSpecification"))?;

    let collection_specification = namespace
      .define_collection("Users", fields_namespace)
      .map_err(|error| error.change_context("creating UserSpecification"))?;

    Ok(Self {
      id_field_specification,
      name_field_specification,
      operating_system_password_field_specification,
      operating_system_user_id_field_specification,
      operating_system_username_field_specification,
      screen_access_regulator_field_specification,
      collection_specification,
    })
  }

  pub fn screen_access_regulator_field_specification(&self) -> &user_screen_access_regulation::database::RegulatorSpecification {
    &self.screen_access_regulator_field_specification
  }

  pub fn update_name(
    &self, 
    modifications: &mut CollectionItemModificationsDraft, 
    new_value: &UserName,
  ) ->
    Result<(), GenericError>
  {
    modifications.modify_scalar_field(&self.name_field_specification, new_value)
  }

  pub fn create_modifications_draft(&self) -> CollectionItemModificationsDraft {
    CollectionItemModificationsDraft::new()
  }

  pub fn apply_modifications_draft(
    &self,
    database: &Database,
    modifications_draft: &CollectionItemModificationsDraft,
    user_id: &Uuid
  ) -> 
    Result<(), GenericError>
  {
    database.update_collection_items(
      &self.collection_specification, 
      &CollectionItemMatcher::match_by_scalar_field(&self.id_field_specification, user_id)?, 
      modifications_draft,
    )
  }

  pub fn add_user(
    &self,
    database: &Database,
    user: &User,
  ) ->
    Result<(), GenericError>
  {
    database.add_collection_item(
      &self.collection_specification, 
      self, 
      user,
    )
  }

  pub fn delete_user(
    &self,
    database: &Database,
    user_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    database.delete_collection_items(
      &self.collection_specification, 
      &CollectionItemMatcher::match_by_scalar_field(
        &self.id_field_specification, 
        user_id,
      )?,
    )
  }
}

impl CompoundValueSerializer for Specification {
  type CompoundValue = User;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.id_field_specification, &value.id)?;  
    context.serializable_scalar(&self.name_field_specification, &value.name)?;  
    context.serializable_scalar(&self.operating_system_user_id_field_specification, &value.operating_system_user_id)?;  
    context.serializable_scalar(&self.operating_system_username_field_specification, &value.operating_system_username)?;  
    context.serializable_scalar(&self.operating_system_password_field_specification, &value.operating_system_password)?;  
    context.serializable_compound(&self.screen_access_regulator_field_specification, &value.screen_access_regulator)
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
      name: self.name,
      operating_system_user_id: self.operating_system_user_id,
      operating_system_username: self.operating_system_username,
      operating_system_password: self.operating_system_password,
      screen_access_regulator: self.screen_access_regulator.denormalize(
        &self.id,
        user_screen_access_regulation_policies,
        user_screen_access_regulation_rules,
      ),
      id: self.id,
    }
  }
}

impl CompoundValueDeserializer for Specification {
  type Output = NormalizedUser;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedUser {
      id: context
        .deserializable_scalar(&self.id_field_specification)
        .map_err(|error| error
          .change_context("deserializing NormalizedUser")
          .add_error("failed to deserialize the 'Id' field")
        )?,

      name: context
        .deserializable_scalar(&self.name_field_specification)
        .map_err(|error| error
          .change_context("deserializing NormalizedUser")
          .add_error("failed to deserialize the 'Name' field")
        )?,

      operating_system_user_id: context
        .deserializable_scalar(&self.operating_system_user_id_field_specification)
        .map_err(|error| error
          .change_context("deserializing NormalizedUser")
          .add_error("failed to deserialize the 'OperatingSystemUserId' field")
        )?,

      operating_system_username: context
        .deserializable_scalar(&self.operating_system_username_field_specification)
        .map_err(|error| error
          .change_context("deserializing NormalizedUser")
          .add_error("failed to deserialize the 'OperatingSystemUsername' field")
        )?,

      operating_system_password: context
        .deserializable_scalar(&self.operating_system_password_field_specification)
        .map_err(|error| error
          .change_context("deserializing NormalizedUser")
          .add_error("failed to deserialize the 'OperatingSystemPassword' field")
        )?,

      screen_access_regulator: context
        .deserialize_compound(&self.screen_access_regulator_field_specification)
        .map_err(|error| error
          .change_context("deserializing NormalizedUser")
          .add_error("failed to deserialize the 'ScreenAccessRegulator' field")
        )?,
    })
  }
}

impl Specification {
  // pub fn generate_sql_initialize(
  //   &self,
  //   into: &mut String,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   let mut statement = InitializeTableStatement::new(into, &self.collection_specification);
  //   statement
  //     .add_compound_type(self)
  //     .map_err(|error| error.change_context("generate sql code that initializes everything related to the users table"))
  // }

  // pub fn generate_sql_add(
  //   &self,
  //   into: &mut String,
  //   user: &User,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   generate_sql_add_row(into, &self.collection_specification, self, user)
  //   .map_err(|error| error
  //     .change_context("generate sql that adds a user to the users table")
  //     .add_attachment("user", format!("{user:?}"))
  //   )
  // }

  // pub fn add(
  //   &self,
  //   connection: &Connection,
  //   user: &User,
  // ) ->
  //   Result<(), GenericError>
  // {
  //   let mut sql = String::new();
  //   self
  //     .generate_sql_add(&mut sql, user)
  //     .and(connection.execute(&sql))
  //     .map_err(|error| error.change_context("add a user to the users table"))
  // }

  // pub fn retrieve_all_normalized(
  //   &self,
  //   connection: &Connection
  // ) -> 
  //   Result<Vec<NormalizedUser>, GenericError>
  // {
  //   connection
  //     .find_all_rows(&self.collection_specification, self)
  //     .map_err(|error| error.change_context("retrieve all users from the users table"))
  // }

  // pub fn create_updater(&self, user_id: &Uuid) -> UpdateStatement {
  //   UpdateStatement::new_given_one_where_field_specifications(
  //     &self.id_field_specification, 
  //     user_id
  //   )
  // }
}