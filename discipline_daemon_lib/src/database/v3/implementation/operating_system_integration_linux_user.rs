use crate::logic::screen_access_regulation;
use crate::operating_system_integration::{
  UserId, 
  User,
  UserPassword,
  UserName,
};
use crate::operating_system_integration as os;

use crate::chronic::Duration;
use super::*;

impl SerializableScalarValue for os::screen_access_regulation::ApplicationStatus {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    match self {
      Self::Allowed => 0.serialize(context),
      Self::LoginBlocked => 1.serialize(context),
      Self::LoginBlockedAndSessionTerminated => 2.serialize(context),
      Self::Unknown => 3.serialize(context),
    }
  }
}

impl DeserializableScalarValue for os::screen_access_regulation::ApplicationStatus {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    match value.as_u8()? {
      0 => Ok(Self::Allowed),
      1 => Ok(Self::LoginBlocked),
      2 => Ok(Self::LoginBlockedAndSessionTerminated),
      3 => Ok(Self::Unknown),
      invalid_value => Err(
        GenericError::new("deserializing screen access regulation application status")
          .add_error("invalid value, expected a number in this range 0..=3")
          .add_attachment("found value", invalid_value.to_string())
      )
    }
  }
}

impl SerializableScalarValue for os::internet_access_regulation::ApplicationStatus {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    match self {
      Self::Allowed => 0.serialize(context),
      Self::Blocked => 1.serialize(context),
      Self::Unknown => 2.serialize(context),
    }
  }
}

impl DeserializableScalarValue for os::internet_access_regulation::ApplicationStatus {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    match value.as_u8()? {
      0 => Ok(Self::Allowed),
      1 => Ok(Self::Blocked),
      2 => Ok(Self::Unknown),
      invalid_value => Err(
        GenericError::new("deserializing screen access regulation application status")
          .add_error("invalid value, expected a number in this range 0..=2")
          .add_attachment("found value", invalid_value.to_string())
      )
    }
  }
}

impl SerializableScalarValue for UserId {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_u32(self.as_raw());
  }
}

impl DeserializableScalarValue for UserId {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_u32()
      .map(UserId::new)
      .map_err(|error|
        error.change_context("deserializing an OperatingSystemUserId")
      )
  }
}

impl SerializableScalarValue for UserName {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(self.as_ref());
  }
}

impl DeserializableScalarValue for UserName {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_string()
      .and_then(UserName::new_or_generic_error)
      .map_err(|error|
        error.change_context("deserializing an OperatingSystemUsername")
      )
  }
}

impl SerializableScalarValue for UserPassword {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(self.as_ref());
  }
}

impl DeserializableScalarValue for UserPassword {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_string()
      .and_then(UserPassword::new_or_generic_error)
      .map_err(|error|
        error.change_context("deserializing an OperatingSystemPassword")
      )
  }
}

pub struct UserSchema {
  user_id: String,
  user_name: String,
  user_password: String,
  user_screen_access_regulation_application_enabled: String,
  user_screen_access_regulation_application_status: String,
  user_screen_access_regulation_application_interval: String,
  user_internet_access_regulation_application_enabled: String,
  user_internet_access_regulation_application_status: String,
  user_internet_access_regulation_application_interval: String,
}

pub struct NormalizedUser {
  user_id: UserId,
  user_name: UserName,
  user_password: UserPassword,
  user_screen_access_regulation_application_status: os::screen_access_regulation::ApplicationStatus,
  user_screen_access_regulation_application_enabled: bool,
  user_screen_access_regulation_application_interval: Duration,
  user_internet_access_regulation_application_status: os::internet_access_regulation::ApplicationStatus,
  user_internet_access_regulation_application_enabled: bool,
  user_internet_access_regulation_application_interval: Duration,
}

impl NormalizedUser {
  pub fn denormalize(
    self,
    user_screen_access_regulation_policies: &Vec<screen_access_regulation_policy::NormalizedPolicy>,
    user_screen_access_regulation_rules: &Vec<screen_access_regulation_rule::NormalizedRule>,
    user_internet_access_regulation_policies: &Vec<internet_access_regulation_policy::NormalizedPolicy>,
    user_internet_access_regulation_rules: &Vec<internet_access_regulation_rule::NormalizedRule>,
  ) -> User {
    let screen_access_regulation_policies = user_screen_access_regulation_policies
      .iter()
      .filter(|policy| policy.user_id == self.user_id)
      .cloned()
      .map(|policy| policy.denormalize(user_screen_access_regulation_rules))
      .collect();

    let internet_access_regulation_policies = user_internet_access_regulation_policies
      .iter()
      .filter(|policy| policy.user_id == self.user_id)
      .cloned()
      .map(|policy| policy.denormalize(user_internet_access_regulation_rules))
      .collect();
    
    User {
      user_id: self.user_id,
      user_name: self.user_name,
      user_password: self.user_password,
      user_screen_access_regulation_logic: crate
        ::logic
        ::screen_access_regulation
        ::Regulation
        ::from_fields(screen_access_regulation_policies),
      user_screen_access_regulation_integration: os
        ::screen_access_regulation
        ::UserSpecificInfo
        ::from_fields(
          self.user_screen_access_regulation_application_status, 
          self.user_screen_access_regulation_application_enabled, 
          self.user_screen_access_regulation_application_interval, 
        ),
      user_internet_access_regulation_integration: os
        ::internet_access_regulation
        ::UserSpecificInfo
        ::from_fields(
          self.user_internet_access_regulation_application_status, 
          self.user_internet_access_regulation_application_interval, 
          self.user_internet_access_regulation_application_enabled,
        ),
      user_internet_access_regulation_logic: crate
        ::logic
        ::internet_access_regulation
        ::Regulation
        ::from_fields(internet_access_regulation_policies)
    }
  }
}

fn serialize_user(
  context: &mut SerializeCompoundValueContext,
  schema: &UserSchema,
  user: &User,
) {
  context.write_scalar(
    &schema.user_id, 
    &user.user_id,
  );
  context.write_scalar(
    &schema.user_name, 
    &user.user_name,
  );
  context.write_scalar(
    &schema.user_password, 
    &user.user_password,
  );
  context.write_scalar(
    &schema.user_screen_access_regulation_application_enabled, 
    &user.user_screen_access_regulation_integration.enabled(),
  );
  context.write_scalar(
    &schema.user_screen_access_regulation_application_status, 
    &user.user_screen_access_regulation_integration.application_status(),
  );
  context.write_scalar(
    &schema.user_screen_access_regulation_application_interval, 
    &user.user_screen_access_regulation_integration.application_interval(),
  );
  context.write_scalar(
    &schema.user_internet_access_regulation_application_enabled, 
    &user.user_internet_access_regulation_integration.application_enabled(),
  );
  context.write_scalar(
    &schema.user_internet_access_regulation_application_status, 
    &user.user_internet_access_regulation_integration.application_status(),
  );
  context.write_scalar(
    &schema.user_internet_access_regulation_application_interval, 
    &user.user_internet_access_regulation_integration.application_interval(),
  );
}

fn deserialize_user(
  context: &mut DeserializeCompoundValueContext,
  schema: &UserSchema,
) 
  -> Result<NormalizedUser, GenericError> 
{
  let user_id = context.deserializable_scalar(&schema.user_id)?;
  let user_name = context.deserializable_scalar(&schema.user_name)?;
  let user_password = context.deserializable_scalar(&schema.user_password)?;
  let user_screen_access_regulation_application_enabled = context.deserializable_scalar(&schema.user_screen_access_regulation_application_enabled)?;
  let user_screen_access_regulation_application_status = context.deserializable_scalar(&schema.user_screen_access_regulation_application_status)?;
  let user_screen_access_regulation_application_interval = context.deserializable_scalar(&schema.user_screen_access_regulation_application_interval)?;
  let user_internet_access_regulation_application_enabled = context.deserializable_scalar(&schema.user_internet_access_regulation_application_enabled)?;
  let user_internet_access_regulation_application_status = context.deserializable_scalar(&schema.user_internet_access_regulation_application_status)?;
  let user_internet_access_regulation_application_interval = context.deserializable_scalar(&schema.user_internet_access_regulation_application_interval)?;

  Ok(NormalizedUser {
    user_id,
    user_name,
    user_password,
    user_screen_access_regulation_application_enabled,
    user_screen_access_regulation_application_status,
    user_screen_access_regulation_application_interval,
    user_internet_access_regulation_application_enabled,
    user_internet_access_regulation_application_interval,
    user_internet_access_regulation_application_status,
  })
}

pub struct UserCollection {
  collection_name: String,
  user_schema: UserSchema,
}

impl UserCollection {
  pub fn new(collection_name: String) -> Self {
    Self {
      collection_name,
      user_schema: UserSchema {
        user_id: "UserId".into(),
        user_name: "UserName".into(),
        user_password: "UserPassword".into(),
        user_screen_access_regulation_application_enabled: "UserScreenAccessRegulationApplicationEnabled".into(),
        user_screen_access_regulation_application_status: "UserScreenAccessRegulationApplicationStatus".into(),
        user_screen_access_regulation_application_interval: "UserScreenAccessRegulationApplicationInterval".into(),
        user_internet_access_regulation_application_enabled: "UserInternetAccessRegulationApplicationEnabled".into(),
        user_internet_access_regulation_application_status: "UserInternetAccessRegulationApplicationStatus".into(),
        user_internet_access_regulation_application_interval: "UserInternetAccessRegulationApplicationInterval".into(),
      }
    }
  }
}

fn collection(database: &Database) -> &UserCollection {
  &database.operating_system_integration_linux_user
}

pub fn write_define(database: &Database, code: &mut DatabaseCode) {
  let me = collection(database);
  
  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&me.collection_name);
  code.write(" (");
  code.write(&me.user_schema.user_id);
  code.write(" INTEGER PRIMARY KEY, ");
  code.write(&me.user_schema.user_name);
  code.write(" TEXT NOT NULL, ");
  code.write(&me.user_schema.user_password);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.user_schema.user_screen_access_regulation_application_interval);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.user_schema.user_screen_access_regulation_application_enabled);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.user_schema.user_screen_access_regulation_application_status);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.user_schema.user_internet_access_regulation_application_interval);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.user_schema.user_internet_access_regulation_application_enabled);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.user_schema.user_internet_access_regulation_application_status);
  code.write(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
}

pub fn write_add_user(database: &Database, code: &mut DatabaseCode, user: &User) {
  let collection = collection(database);

  code.write("INSERT INTO ");
  code.write(&collection.collection_name);

  let mut context = SerializeCompoundValueContext::new();
  serialize_user(&mut context, &collection.user_schema, user);

  code.write(" (");
  code.write(&context.column_names);
  code.write(") VALUES (");
  code.write(&context.column_values);
  code.write(");");
}

pub fn add_user(database: &Database, user: &User) -> Result<(), GenericError> {
  let mut draft = DatabaseCode::new();
  write_add_user(database, &mut draft, user);
  database.execute(draft.as_str())
}

pub fn write_delete_user(database: &Database, code: &mut DatabaseCode, user_id: UserId) {
  let collection = collection(database);

  code.write("DELETE FROM ");
  code.write(&collection.collection_name);
  code.write(" WHERE ");
  code.write(&collection.user_schema.user_id);
  code.write(" = ");
  serialize_scalar_value_into(&user_id, code.as_mut());
  code.write(";");
}

pub fn delete_user(database: &Database, user_id: UserId) -> Result<(), GenericError> {
  let mut draft = DatabaseCode::new();
  write_delete_user(database, &mut draft, user_id);
  database.execute(draft.as_str())
}

pub fn write_retrieve_all(database: &Database, code: &mut DatabaseCode) {
  let collection = collection(database);

  code.write("SELECT * FROM ");
  code.write(&collection.collection_name);
  code.write(";");
}

pub struct UserUpdateDraft {
  draft: CollectionItemUpdateDraft
}

impl UserUpdateDraft {
  pub fn new() -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new()
    }
  }
}

pub fn write_name(
  database: &Database, 
  draft: &mut UserUpdateDraft, 
  new_value: &UserName,
) {
  let collection = collection(database);
  draft.draft.write_scalar(&collection.user_schema.user_name, new_value);
}

// pub fn update_name(
//   database: &Database,
//   user_id: OperatingSystemUserId,
//   new_value: &UserName,
// ) -> Result<(), GenericError> {
//   let mut draft = UserUpdateDraft::new();
//   write_name(database, &mut draft, new_value);
//   commit_user_update_draft(database, &draft, user_id)
// }

pub fn write_user_screen_access_regulation_application_enabled(
  database: &Database,
  draft: &mut UserUpdateDraft,
  new_value: bool,
) {
  let collection = collection(database);
  draft.draft.write_scalar(&collection.user_schema.user_screen_access_regulation_application_enabled, &new_value);
}

pub fn update_user_screen_access_regulation_application_enabled(
  database: &Database,
  user_id: UserId,
  new_value: bool,
) -> Result<(), GenericError> {
  let mut draft = UserUpdateDraft::new();
  write_user_screen_access_regulation_application_enabled(database, &mut draft, new_value);
  commit_user_update_draft(database, &draft, user_id)
}

pub fn write_screen_access_regulation_application_status(
  database: &Database, 
  draft: &mut UserUpdateDraft,
  new_value: os::screen_access_regulation::ApplicationStatus,
) {
  let collection = collection(database);
  draft.draft.write_scalar(&collection.user_schema.user_screen_access_regulation_application_status, &new_value);
}

pub fn update_screen_access_regulation_application_status(
  database: &Database, 
  user_id: UserId,
  new_value: os::screen_access_regulation::ApplicationStatus,
) -> Result<(), GenericError> {
  let mut draft = UserUpdateDraft::new();
  write_screen_access_regulation_application_status(database, &mut draft, new_value);
  commit_user_update_draft(database, &draft, user_id)
}

pub fn write_update_user(
  database: &Database, 
  database_update_draft: &mut DatabaseCode, 
  user_update_draft: &UserUpdateDraft, 
  user_id: UserId,
) {
  let Some(updates) = user_update_draft.draft.updates() else {
    return;
  };

  let collection = collection(database);

  database_update_draft.write("UPDATE ");
  database_update_draft.write(&collection.collection_name);
  database_update_draft.write(" SET ");
  database_update_draft.write(&updates);
  database_update_draft.write(" WHERE ");
  database_update_draft.write(&collection.user_schema.user_id);
  database_update_draft.write(" = ");
  serialize_scalar_value_into(&user_id, &mut database_update_draft.code);
  database_update_draft.write(";");
}

// pub fn update_user() {

// }

pub fn commit_user_update_draft(
  database: &Database,
  user_update_draft: &UserUpdateDraft,
  user_id: UserId,
) -> Result<(), GenericError> {
  if user_update_draft.draft.is_empty() {
    return Ok(())
  }

  let mut database_update_draft = DatabaseCode::new();
  write_update_user(database, &mut database_update_draft, user_update_draft, user_id);
  database.execute(database_update_draft.as_ref())
}

pub fn retrieve_all(database: &Database) -> Result<Vec<NormalizedUser>, GenericError> {
  let collection = collection(database);

  let mut code = DatabaseCode::new();
  write_retrieve_all(database, &mut code);
  
  let connection = database.connection.lock().unwrap();
  let mut statement = connection.prepare(&code.code).map_err(|error| 
    GenericError::new("")
  )?;

  let mut iterator = statement.query(()).map_err(|error| 
    GenericError::new("")
  )?;
  
  let mut rules = Vec::new();
  
  loop {
    let item = iterator.next().map_err(|error| 
      GenericError::new("")
    )?;
  
    let Some(item) = item else {
      return Ok(rules);
    };
  
    let mut context = DeserializeCompoundValueContext(item);
  
    rules.push(deserialize_user(&mut context, &collection.user_schema)?);
  }
}