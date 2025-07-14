use crate::*;
use super::*;

pub struct AppFields {
  id: String,
  user_screen_access_regulation_private_password: String,
  user_screen_access_regulation_applying_interval: String,
}

// We only have a single item in the GlobalAppDataCollection and this 
// is the value of its "id" field.
static ID: u8 = 0;

pub struct NormalizedApp {
  id: u8,
  user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo,
}

impl Default for NormalizedApp {
  fn default() -> Self {
    Self {
      id: ID,
      user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo::default()
    }
  }
}

impl NormalizedApp {
  pub fn denormalize(self, users: Vec<User>) -> AppState {
    AppState {
      users,
      user_screen_access_regulation_common_info: self.user_screen_access_regulation_common_info,
    }
  }
}

fn serialize(
  context: &mut SerializeCompoundValueContext,
  fields: &AppFields,
  app_state: &AppState,
) {
  context.write_u8(&fields.id, ID);
  context.write_scalar(&fields.user_screen_access_regulation_private_password, app_state.user_screen_access_regulation_common_info.private_password());
  context.write_scalar(&fields.user_screen_access_regulation_applying_interval, &app_state.user_screen_access_regulation_common_info.applying_interval());
}

fn deserialize(
  context: &mut DeserializeCompoundValueContext,
  fields: &AppFields,
)
  -> Result<NormalizedApp, GenericError>
{
  Ok(NormalizedApp { 
    id: context.deserializable_scalar(&fields.id)?, 
    user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo::pack(
      context.deserializable_scalar(&fields.user_screen_access_regulation_private_password)?,
      context.deserializable_scalar(&fields.user_screen_access_regulation_applying_interval)?, 
    )
  })
}

pub struct AppCollection {
  name: String,
  fields: AppFields,
}

impl AppCollection {
  pub fn new(
    collection_name: String,
    id_field: String, 
    user_screen_access_regulation_private_password_field: String,
    user_screen_access_regulation_applying_interval_field: String,
  ) -> Self {
    Self {
      name: collection_name,
      fields: AppFields {
        id: id_field,
        user_screen_access_regulation_private_password: user_screen_access_regulation_private_password_field,
        user_screen_access_regulation_applying_interval: user_screen_access_regulation_applying_interval_field,
      }
    }
  }
}

impl Database  {
  fn collection(&self) -> &AppCollection {
    &self.app
  }
}

pub fn write_define(database: &Database, code: &mut DatabaseCode) {
  let collection = database.collection();

  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&collection.name);
  code.write(" (");
  code.write(&collection.fields.id);
  code.write(" INTEGER PRIMARY KEY, ");
  code.write(&collection.fields.user_screen_access_regulation_private_password);
  code.write(" TEXT NOT NULL, ");
  code.write(&collection.fields.user_screen_access_regulation_applying_interval);
  code.write(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
}

fn write_initialize_item(database: &Database, code: &mut DatabaseCode) -> NormalizedApp {
  let collection = database.collection();

  code.write("INSERT INTO ");
  code.write(&collection.name);

  let app = NormalizedApp::default();
  let mut context = SerializeCompoundValueContext::new();
  context.write_u8(&collection.fields.id, ID);
  context.write_scalar(&collection.fields.user_screen_access_regulation_private_password, app.user_screen_access_regulation_common_info.private_password());
  context.write_scalar(&collection.fields.user_screen_access_regulation_applying_interval, &app.user_screen_access_regulation_common_info.applying_interval());

  code.write(" (");
  code.write(&context.column_names);
  code.write(") VALUES (");
  code.write(&context.column_values);
  code.write(");");

  app
}

fn initialize(database: &Database) -> Result<NormalizedApp, GenericError> {
  let mut code = DatabaseCode::new();
  let app = write_initialize_item(database, &mut code);
  database.execute(code.as_str())?;
  Ok(app)
}

pub fn retrieve_normalized(database: &Database) -> Result<NormalizedApp, GenericError> {
  let collection = database.collection();

  let mut code = DatabaseCode::new();
  code.write("SELECT FROM ");
  code.write(&collection.name);
  code.write(" WHERE ");
  code.write(&collection.fields.id);
  code.write(" = ");
  serialize_scalar_value_into(&0, code.as_mut());
  code.write(";");

  let mut statement = database.connection.prepare(code.as_str()).map_err(|error|
    GenericError::new("")
  )?;
  let mut iterator = statement.query(()).map_err(|error|
    GenericError::new("")
  )?;
  let item = iterator.next().map_err(|error|
    GenericError::new("")
  )?;
  let Some(item) = item else {
    return initialize(database);
  };

  let context = DeserializeCompoundValueContext(item);
  Ok(NormalizedApp {
    id: context.deserializable_scalar(&collection.fields.id)?,
    user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo::pack(
      context.deserializable_scalar(&collection.fields.user_screen_access_regulation_private_password)?,
      context.deserializable_scalar(&collection.fields.user_screen_access_regulation_applying_interval)?,
    )
  })
}

pub fn retrieve(database: &Database) -> Result<AppState, GenericError> {
  let normalized_app = retrieve_normalized(database)?;
  let normalized_users = user_collection::retrieve_all(database)?;
  let normalized_user_screen_access_regulation_rules = screen_access_regulation_rule_integration::retrieve_all_rules(database)?;
  let normalized_user_screen_access_regulation_policies = screen_access_regulation_policy_collection::retrieve_all_policies(database)?;    

  let denormalized_users = normalized_users
    .into_iter()
    .map(|user| 
      user.denormalize(
        &normalized_user_screen_access_regulation_policies, 
        &normalized_user_screen_access_regulation_rules,
      )
    )
    .collect();

  let denormalized_app = normalized_app.denormalize(denormalized_users);

  Ok(denormalized_app)
}