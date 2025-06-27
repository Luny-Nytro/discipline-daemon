use super::{
  UserName, User
};

use crate::{
  GenericError, Uuid, OperatingSystemPassword, 
  OperatingSystemUserId, OperatingSystemUsername,
  user_screen_access_regulation,
};

use crate::database::{
  CollectionItemModificationsDraft, CollectionItemDefiner, 
  CollectionItemMatcher, CompoundTypeSerializer, 
  CompoundTypeSerializerContext, CompoundValueDeserializer, 
  CompoundValueDeserializerContext, Database, 
  Field, FromScalarValue, IntoScalarValue, IsCollectionItem, 
  IsScalarValue, ScalarValue, Collection,
};

impl IntoScalarValue for UserName {
  fn into_scalar_value(&self) -> impl IsScalarValue {
    self.as_ref()
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

pub struct UserSpecification {
  id: Field,
  name: Field,
  screen_access_regulator: user_screen_access_regulation::database::RegulatorSpecification,
  operating_system_user_id: Field,
  operating_system_username: Field,
  operating_system_password: Field,
}

impl IsCollectionItem for UserSpecification {
  fn new(definer: &mut CollectionItemDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      id: definer.primary_scalar_field("Id")?,
      name: definer.writable_required_field("Name")?,
      screen_access_regulator: definer.compound_field("ScreenAccessRegulation")?,
      operating_system_user_id: definer.primary_scalar_field("OperatingSystemUserId")?,
      operating_system_username: definer.primary_scalar_field("OperatingSystemUsername")?,
      operating_system_password: definer.readonly_required_field("OperatingSystemPassword")?,
    })
  }

  fn display_name(&self) -> &str {
    "User"
  }
}

impl UserSpecification {
  pub fn write_name(
    &self, 
    draft: &mut CollectionItemModificationsDraft, 
    new_value: &UserName,
  ) ->
    Result<(), GenericError>
  {
    draft.write_scalar_field(&self.name, new_value)
  }
}

pub struct UserSerializer<'a> {
  user_specification: &'a UserSpecification
}

impl<'a> UserSerializer<'a> {
  pub fn new(user_specification: &'a UserSpecification) -> Self {
    Self {
      user_specification
    }
  }
}

impl<'a> CompoundTypeSerializer for UserSerializer<'a> {
  type CompoundType = User;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.user_specification.id, &value.id)?;  
    context.serializable_scalar(&self.user_specification.name, &value.name)?;  
    context.serializable_scalar(&self.user_specification.operating_system_user_id, &value.operating_system_user_id)?;  
    context.serializable_scalar(&self.user_specification.operating_system_username, &value.operating_system_username)?;  
    context.serializable_scalar(&self.user_specification.operating_system_password, &value.operating_system_password)?;  
    context.serializable_compound(&self.user_specification.screen_access_regulator, &value.screen_access_regulator)
  }
}


pub struct NormalizedUser {
  id: Uuid,
  name: UserName,
  operating_system_user_id: OperatingSystemUserId,
  operating_system_username: OperatingSystemUsername,
  operating_system_password: OperatingSystemPassword,
  screen_access_regulator: user_screen_access_regulation::database::NormalizedRegulator
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

pub struct UserDeserializer<'a> {
  user_specification: &'a UserSpecification
}

impl<'a> UserDeserializer<'a> {
  pub fn new(user_specification: &'a UserSpecification) -> Self {
    Self {
      user_specification
    }
  }
}

impl<'a> CompoundValueDeserializer for UserDeserializer<'a> {
  type Output = NormalizedUser;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedUser {
      id: context.deserializable_scalar(&self.user_specification.id)?,
      name: context.deserializable_scalar(&self.user_specification.name)?,
      screen_access_regulator: context.deserialize_compound(&self.user_specification.screen_access_regulator)?,
      operating_system_user_id: context.deserializable_scalar(&self.user_specification.operating_system_user_id)?,
      operating_system_username: context.deserializable_scalar(&self.user_specification.operating_system_username)?,
      operating_system_password: context.deserializable_scalar(&self.user_specification.operating_system_password)?,
    })
  }
}

pub struct UserCollection {
  user_collection: Collection,
  user: UserSpecification
}

impl UserCollection {
  pub fn add_user(
    &self,
    database: &Database,
    user: &User,
  ) ->
    Result<(), GenericError>
  {
    self.user_collection.add_item(
      database, 
      &UserSerializer::new(&self.user), 
      user
    )
  }

  pub fn delete_user(
    &self,
    database: &Database,
    user_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    self.user_collection.delete_items(
      database, 
      &CollectionItemMatcher::match_by_scalar_field(&self.user.id, user_id)?,
    )
  }
}



impl UserCollection {
  pub fn change_user_name(
    &self,
    database: &Database,
    user_id: &Uuid,
    new_value: &UserName,
  ) ->
    Result<(), GenericError>
  {
    let mut draft = self.user_collection.create_modifications_draft();
    
    self.user.write_name(
      &mut draft, 
      new_value,
    )?;

    self.user_collection.commit_modifications_draft(
      database, 
      &draft, 
      &CollectionItemMatcher::match_by_scalar_field(&self.user.id, user_id)?,
    )
  }

  pub fn change_user_screen_access_is_applying_enabled(
    &self,
    database: &Database,
    user_id: &Uuid,
    new_value: bool,
  ) ->
    Result<(), GenericError>
  {
    let mut draft = self.user_collection.create_modifications_draft();
    
    self.user.screen_access_regulator.write_is_applying_enabled(
      &mut draft, 
      new_value,
    )?;

    self.user_collection.commit_modifications_draft(
      database, 
      &draft, 
      &CollectionItemMatcher::match_by_scalar_field(&self.user.id, user_id)?,
    )
  }

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