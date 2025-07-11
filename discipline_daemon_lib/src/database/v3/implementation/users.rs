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
  id: Uuid,
  name: UserName,
  operating_system_user_id: OperatingSystemUserId,
  operating_system_user_name: OperatingSystemUsername,
  operating_system_user_password: OperatingSystemPassword,
  screen_access_regulation_is_applying_enabled: bool,
  screen_access_regulation_is_user_screen_access_blocked: bool,
}

impl NormalizedUser {
  pub fn denormalize(
    self, 
    user_screen_access_regulation_policies: &Vec<UserScreenAccessPolicyNormalized>,
    user_screen_access_regulation_rules: &Vec<UserScreenAccessRuleNormalized>,
  ) -> User {
    User {
      name: self.name,
      operating_system_user_id: self.operating_system_user_id,
      operating_system_username: self.operating_system_user_name,
      operating_system_password: self.operating_system_user_password,
      screen_access_regulator: user_screen_access_regulation::Regulator::pack(
        &self.id,
        user_screen_access_regulation_policies,
        user_screen_access_regulation_rules,
      ),
      id: self.id,
    }
  }
}

pub fn serialize_user(
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

pub fn deserialize_user(
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

  pub fn write_definition_into(&self, code: &mut String) {
    code.push_str("CREATE TABLE IF NOT EXISTS ");
    code.push_str(&self.name);
    code.push_str(" (");
    code.push_str(&self.fields.id);
    code.push_str(" TEXT PRIMARY KEY, ");
    code.push_str(&self.fields.name);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.operating_system_user_id);
    code.push_str(" INTEGER NOT NULL, ");
    code.push_str(&self.fields.operating_system_user_name);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.operating_system_user_password);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.screen_access_regulation_is_applying_enabled);
    code.push_str(" INTEGER NOT NULL, ");
    code.push_str(&self.fields.screen_access_regulation_is_user_screen_access_blocked);
    code.push_str(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
  }

  pub fn add_user(&self, code: &mut String, user: &User) {
    code.push_str("INSERT INTO ");
    code.push_str(&self.name);

    let mut context = SerializeCompoundValueContext::new();
    serialize_user(&mut context, &self.fields, user);

    code.push_str(" (");
    code.push_str(&context.column_names);
    code.push_str(") VALUES (");
    code.push_str(&context.column_values);
    code.push_str(");");
  }

  pub fn delete_user(&self, code: &mut String, user_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, code);
    code.push_str(";");
  }

  pub fn write_retrieve_all_users(&self, code: &mut String) {
    code.push_str("SELECT * FROM ");
    code.push_str(&self.name);
    code.push_str(";");
  }

  pub fn retrieve_all_users(&self, database: &Database) -> Result<Vec<NormalizedUser>, GenericError> {
    let mut code = String::new();
    self.write_retrieve_all_users(&mut code);

    let mut statement = database.connection.prepare(&code).map_err(|error| 
      GenericError::new("")
    )?;
    let mut iterator = statement.query(&code).map_err(|error| 
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
      rules.push(deserialize_user(&mut context, &self.fields)?);
    }
  }

  pub fn update_user(&self, code: &mut String, user_id: &Uuid, user_update_draft: &UserUpdateDraft) {
    user_update_draft.maybe_write_update_statement_into(code, user_id);
  }

  pub fn create_user_update_draft<'a>(&self, database: &'a Database) -> UserUpdateDraft<'a> {
    UserUpdateDraft::new(database)
  }
}
pub struct UserUpdateDraft<'a> {
  draft: CollectionItemUpdateDraft,
  database: &'a Database,
  collection: &'a UserCollection,
}

impl<'a> UserUpdateDraft<'a> {
  pub fn new(database: &'a Database) -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new(),
      database,
      collection: &database.user,
    }
  }

  pub fn update_name(&mut self, new_value: &UserName) {
    self.draft.write_scalar(&self.database.user.fields.name, new_value);
  }

  pub fn update_screen_access_regulation_is_applying_enabled(&mut self, new_value: bool) {
    self.draft.write_scalar(&self.collection.fields.screen_access_regulation_is_applying_enabled, &new_value);
  }
  
  pub fn update_screen_access_regulation_is_user_screen_access_blocked(&mut self, new_value: bool) {
    self.draft.write_scalar(&self.collection.fields.screen_access_regulation_is_user_screen_access_blocked, &new_value);
  }

  pub fn maybe_write_update_statement_into(&self, code: &mut String, user_id: &Uuid) {
    let Some(updates) = self.draft.updates() else {
      return;
    };

    code.push_str("UPDATE ");
    code.push_str(&self.collection.name);
    code.push_str(" SET ");
    code.push_str(&updates);
    code.push_str(" WHERE ");
    code.push_str(&self.collection.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, code);
    code.push_str(";");
  }

  pub fn commit(&self, user_id: &Uuid) -> Result<(), GenericError> {
    if self.draft.is_empty() {
      return Ok(())
    }

    let mut code = String::new();
    self.maybe_write_update_statement_into(&mut code, user_id);
    self.database.execute(&code)
  }
}

pub struct UserCollectionUpdateDraft<'a> {
  code: String,
  database: &'a Database,
}

impl<'a> UserCollectionUpdateDraft<'a> {
  pub fn new(database: &'a Database) -> Self {
    Self {
      code: String::new(),
      database,
    }
  }
  
  pub fn add_user(&mut self, user: &User) {
    self.database.user.add_user(&mut self.code, user);
  }
  
  pub fn delete_user(&mut self, user_id: &Uuid) {
    self.database.user.delete_user(&mut self.code, user_id);
  }
  
  pub fn update_user(&mut self, user_id: &Uuid, user_update_draft: &UserUpdateDraft) {
    self.database.user.update_user(&mut self.code, user_id, user_update_draft);
  }
  
  pub fn commit(&self) -> Result<(), GenericError> {
    self.database.execute(&self.code)
  }
}