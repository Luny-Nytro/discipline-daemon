use crate::user::*;
use crate::*;
use super::*;

impl SerializableScalarValue for UserName {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(self.as_ref());
  }
}

impl DeserializableScalarValue for UserName {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
      value
      .as_string()
      .and_then(UserName::new)
      .map_err(|error| error.change_context("deserializing UserName"))
  }
}

pub struct UserFields {
  id: String,
  name: String,
  operating_system_user_id: String,
  operating_system_user_name: String,
  operating_system_user_password: String,
  screen_access_regulation_is_applying_enabled: String,
  screen_access_regulation_is_user_screen_access_blocked: String,
}

pub struct NormalizedUser {
  pub(super) id: Uuid,
  pub(super) name: UserName,
  pub(super) operating_system_user_id: OperatingSystemUserId,
  pub(super) operating_system_user_name: OperatingSystemUsername,
  pub(super) operating_system_user_password: OperatingSystemPassword,
  pub(super) screen_access_regulation_is_applying_enabled: bool,
  pub(super) screen_access_regulation_is_user_screen_access_blocked: bool,
}

impl NormalizedUser {
  pub fn denormalize(
    self, 
    user_screen_access_regulation_policies: &Vec<UserScreenAccessPolicyNormalized>,
    user_screen_access_regulation_rules: &Vec<UserScreenAccessRuleNormalized>,
  ) -> User {
    let policies = user_screen_access_regulation_policies
      .iter()
      .filter(|policy| policy.user_id == self.id)
      .cloned()
      .map(|policy| policy.denormalize(user_screen_access_regulation_rules))
      .collect();

    User::from_fields(
      self.id,
      self.name,
      self.operating_system_user_id,
      self.operating_system_user_name,
      self.operating_system_user_password,
      user_screen_access_regulation::Regulator::pack(
        policies,
        self.screen_access_regulation_is_applying_enabled,
        self.screen_access_regulation_is_user_screen_access_blocked,
      ),
    )
  }
}

fn serialize_user(
  context: &mut SerializeCompoundValueContext,
  fields: &UserFields,
  user: &User,
) {
  context.write_scalar(&fields.id, user.id());
  context.write_scalar(&fields.name, user.name());
  context.write_scalar(&fields.operating_system_user_id, user.operating_system_user_id());
  context.write_scalar(&fields.operating_system_user_name, user.operating_system_user_name());
  context.write_scalar(&fields.operating_system_user_password, user.operating_system_user_password());
  context.write_scalar(&fields.screen_access_regulation_is_applying_enabled, &user.screen_access_regulator().is_applying_enabled());
  context.write_scalar(&fields.screen_access_regulation_is_user_screen_access_blocked, &user.screen_access_regulator().is_user_screen_access_blocked());
}

fn deserialize_user(
  context: &mut DeserializeCompoundValueContext,
  fields: &UserFields,
) 
  -> Result<NormalizedUser, GenericError> 
{
  let id = context.deserializable_scalar(&fields.id)?;
  let name = context.deserializable_scalar(&fields.name)?;
  let operating_system_user_id = context.deserializable_scalar(&fields.operating_system_user_id)?;
  let operating_system_user_name = context.deserializable_scalar(&fields.operating_system_user_name)?;
  let operating_system_user_password = context.deserializable_scalar(&fields.operating_system_user_password)?;
  let screen_access_regulation_is_applying_enabled = context.deserializable_scalar(&fields.screen_access_regulation_is_applying_enabled)?;
  let screen_access_regulation_is_user_screen_access_blocked = context.deserializable_scalar(&fields.screen_access_regulation_is_user_screen_access_blocked)?;

  Ok(NormalizedUser {
    id,
    name,
    operating_system_user_id,
    operating_system_user_name,
    operating_system_user_password,
    screen_access_regulation_is_applying_enabled,
    screen_access_regulation_is_user_screen_access_blocked,
  })
}

pub struct UserCollection {
  name: String,
  fields: UserFields
}

impl UserCollection {
  pub fn new(
    collection_name: String,
    user_id_field: String,
    user_name_field: String,
    user_operating_system_user_id_field: String,
    user_operating_system_user_name_field: String,
    user_operating_system_user_password_field: String,
    user_screen_access_regulation_is_applying_enabled_field: String,
    user_screen_access_regulation_is_user_screen_access_blocked_field: String,
  ) -> Self {
    Self {
      name: collection_name,
      fields: UserFields {
        id: user_id_field,
        name: user_name_field,
        operating_system_user_id: user_operating_system_user_id_field,
        operating_system_user_name: user_operating_system_user_name_field,
        operating_system_user_password: user_operating_system_user_password_field,
        screen_access_regulation_is_applying_enabled: user_screen_access_regulation_is_applying_enabled_field,
        screen_access_regulation_is_user_screen_access_blocked: user_screen_access_regulation_is_user_screen_access_blocked_field,
      }
    }
  }

  pub fn new_with_descriptive_field_names(collection_name: String) -> Self {
    Self {
      name: collection_name,
      fields: UserFields {
        id: "Id".into(),
        name: "Name".into(),
        operating_system_user_id: "OperatingSystemUserId".into(),
        operating_system_user_name: "OperatingSystemUserName".into(),
        operating_system_user_password: "OperatngSystemUserPassword".into(),
        screen_access_regulation_is_applying_enabled: "UserScreenAccessRegulationIsApplyingEnabled".into(),
        screen_access_regulation_is_user_screen_access_blocked: "UserScreenAccessRegulationIsUserScreenAccessBlocked".into(),
      }
    }
  }
}

fn collection(database: &Database) -> &UserCollection {
  &database.user
}

pub fn write_define(database: &Database, code: &mut DatabaseCode) {
  let me = collection(database);
  
  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&me.name);
  code.write(" (");
  code.write(&me.fields.id);
  code.write(" TEXT PRIMARY KEY, ");
  code.write(&me.fields.name);
  code.write(" TEXT NOT NULL, ");
  code.write(&me.fields.operating_system_user_id);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.fields.operating_system_user_name);
  code.write(" TEXT NOT NULL, ");
  code.write(&me.fields.operating_system_user_password);
  code.write(" TEXT NOT NULL, ");
  code.write(&me.fields.screen_access_regulation_is_applying_enabled);
  code.write(" INTEGER NOT NULL, ");
  code.write(&me.fields.screen_access_regulation_is_user_screen_access_blocked);
  code.write(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
}

pub fn write_add_user(database: &Database, code: &mut DatabaseCode, user: &User) {
  let collection = collection(database);

  code.write("INSERT INTO ");
  code.write(&collection.name);

  let mut context = SerializeCompoundValueContext::new();
  serialize_user(&mut context, &collection.fields, user);

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

pub fn write_delete_user(database: &Database, code: &mut DatabaseCode, user_id: &Uuid) {
  let collection = collection(database);

  code.write("DELETE FROM ");
  code.write(&collection.name);
  code.write(" WHERE ");
  code.write(&collection.fields.id);
  code.write(" = ");
  serialize_scalar_value_into(user_id, code.as_mut());
  code.write(";");
}

pub fn delete_user(database: &Database, user_id: &Uuid) -> Result<(), GenericError> {
  let mut draft = DatabaseCode::new();
  write_delete_user(database, &mut draft, user_id);
  database.execute(draft.as_str())
}

pub fn write_retrieve_all(database: &Database, code: &mut DatabaseCode) {
  let collection = collection(database);

  code.write("SELECT * FROM ");
  code.write(&collection.name);
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
  draft.draft.write_scalar(&collection.fields.name, new_value);
}

pub fn update_name(
  database: &Database,
  user_id: &Uuid,
  new_value: &UserName,
) -> Result<(), GenericError> {
  let mut draft = UserUpdateDraft::new();
  write_name(database, &mut draft, new_value);
  commit_user_update_draft(database, &draft, user_id)
}

pub fn write_screen_access_regulation_is_applying_enabled(
  database: &Database,
  draft: &mut UserUpdateDraft,
  new_value: bool,
) {
  let collection = collection(database);
  draft.draft.write_scalar(&collection.fields.screen_access_regulation_is_applying_enabled, &new_value);
}

pub fn update_screen_access_regulation_is_applying_enabled(
  database: &Database,
  user_id: &Uuid,
  new_value: bool,
) -> Result<(), GenericError> {
  let mut draft = UserUpdateDraft::new();
  write_screen_access_regulation_is_applying_enabled(database, &mut draft, new_value);
  commit_user_update_draft(database, &draft, user_id)
}

pub fn write_screen_access_regulation_is_user_screen_access_blocked(
  database: &Database, 
  draft: &mut UserUpdateDraft,
  new_value: bool,
) {
  let collection = collection(database);
  draft.draft.write_scalar(&collection.fields.screen_access_regulation_is_user_screen_access_blocked, &new_value);
}

pub fn write_update_user(
  database: &Database, 
  database_update_draft: &mut DatabaseCode, 
  user_update_draft: &UserUpdateDraft, 
  user_id: &Uuid,
) {
  let Some(updates) = user_update_draft.draft.updates() else {
    return;
  };

  let collection = collection(database);

  database_update_draft.write("UPDATE ");
  database_update_draft.write(&collection.name);
  database_update_draft.write(" SET ");
  database_update_draft.write(&updates);
  database_update_draft.write(" WHERE ");
  database_update_draft.write(&collection.fields.id);
  database_update_draft.write(" = ");
  serialize_scalar_value_into(user_id, &mut database_update_draft.code);
  database_update_draft.write(";");
}

// pub fn update_user() {

// }

pub fn commit_user_update_draft(
  database: &Database,
  user_update_draft: &UserUpdateDraft,
  user_id: &Uuid,
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

  let mut statement = database.connection.prepare(&code.code).map_err(|error| 
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
  
    rules.push(deserialize_user(&mut context, &collection.fields)?);
  }
}