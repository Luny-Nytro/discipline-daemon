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

pub struct AppNormalized {
  id: u8,
  user_screen_access_regulation: user_screen_access_regulation::CommonInfo,
}

impl Default for AppNormalized {
  fn default() -> Self {
    Self {
      id: 0,
      user_screen_access_regulation: user_screen_access_regulation::CommonInfo::default()
    }
  }
}

pub fn serialize(
  context: &mut SerializeCompoundValueContext,
  fields: &AppFields,
  app: &AppState,
) {
  context.write_u8(&fields.id, ID);
  context.write_scalar(&fields.user_screen_access_regulation_private_password, app.user_screen_access_regulation_common_info.private_password());
  context.write_scalar(&fields.user_screen_access_regulation_applying_interval, &app.user_screen_access_regulation_common_info.applying_interval());
}

pub fn deserialize(
  context: &mut DeserializeCompoundValueContext,
  fields: &AppFields,
)
  -> Result<AppNormalized, GenericError>
{
  Ok(AppNormalized { 
    id: context.deserializable_scalar(&fields.id)?, 
    user_screen_access_regulation: user_screen_access_regulation::CommonInfo::pack(
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

  pub fn write_definition_into(&self, code: &mut String) {
    code.push_str("CREATE TABLE IF NOT EXISTS ");
    code.push_str(&self.name);
    code.push_str(" (");
    code.push_str(&self.fields.id);
    code.push_str(" INTEGER PRIMARY KEY, ");
    code.push_str(&self.fields.user_screen_access_regulation_private_password);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.user_screen_access_regulation_applying_interval);
    code.push_str(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
  }

  pub fn write_initialization_statement_into(&self, code: &mut String) -> AppNormalized {
    code.push_str("INSERT INTO ");
    code.push_str(&self.name);

    let app = AppNormalized::default();
    let mut context = SerializeCompoundValueContext::new();
    context.write_u8(&self.fields.id, ID);
    context.write_scalar(&self.fields.user_screen_access_regulation_private_password, app.user_screen_access_regulation.private_password());
    context.write_scalar(&self.fields.user_screen_access_regulation_applying_interval, &app.user_screen_access_regulation.applying_interval());

    code.push_str(" (");
    code.push_str(&context.column_names);
    code.push_str(") VALUES (");
    code.push_str(&context.column_values);
    code.push_str(");");

    app
  }

  pub fn initialize(&self, database: &Database) -> Result<AppNormalized, GenericError> {
    let mut code = String::new();
    let app = self.write_initialization_statement_into(&mut code);
    database.execute(&code)?;
    Ok(app)
  }
}

pub struct Retriever<'a> {
  database: &'a Database,
  collection: &'a AppCollection,
}

impl<'a> Retriever<'a> {
  pub fn retrieve_app_normalized(&self) -> Result<AppNormalized, GenericError> {
    let mut code = String::new();
    code.push_str("SELECT FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(&0, &mut code);
    code.push_str(";");

    let mut statement = self.database.connection.prepare(&code).map_err(|error|
      GenericError::new("")
    )?;
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("")
    )?;
    let item = iterator.next().map_err(|error|
      GenericError::new("")
    )?;
    let Some(item) = item else {
      return self.collection.initialize(self.database);
    };

    let context = DeserializeCompoundValueContext(item);
    Ok(AppNormalized {
      id: context.deserializable_scalar(&self.collection.fields.id)?,
      user_screen_access_regulation: user_screen_access_regulation::CommonInfo::pack(
        context.deserializable_scalar(&self.collection.fields.user_screen_access_regulation_private_password)?,
        context.deserializable_scalar(&self.collection.fields.user_screen_access_regulation_applying_interval)?,
      )
    })
  }

  pub fn retrieve_app(&self) -> Result<AppState, GenericError> {
    let mut app = self.retrieve_app_normalized()?;
    let mut users = self.database.user.retrieve_all_users(self.database);
    let mut user_screen_access_regulation_rules = self.database.user_screen_access_regulation_rule.retrieve_all_rules(self.database)?;
    let mut user_screen_access_regulation_policies = self.database.user_screen_access_regulation_policy.retrieve_all_policies(self.database)?;
    
  }
}