use std::collections::HashMap;
use crate::operating_system_integration::*;
use crate::*;
use super::*;

pub struct DataSchema {
  id: String,
  screen_access_regulation_application_blocked_user_password: String,
}

static ID_FIELD_VALUE: u8 = 0;

pub struct NormalizedData {
  id: u8,
  screen_access_regulation_application_blocked_user_password: UserPassword,
}

impl NormalizedData {
  pub fn initial() -> Self {
    NormalizedData { 
      id: ID_FIELD_VALUE, 
      screen_access_regulation_application_blocked_user_password: UserPassword::generate_random_password()
    }
  }

  pub fn denormalize(self, users: HashMap<UserId, UserInfo>) -> OperatingSystemIntegrationData {
    OperatingSystemIntegrationData {
      users,
      screen_access_regulation_application_common_info: screen_access_regulation_application::CommonScreenAccessRegulationApplicationData::new()
    }
  }
}

fn serialize(
  context: &mut SerializeCompoundValueContext,
  schema: &DataSchema,
  data: &OperatingSystemIntegrationData,
) {
  context.write_u8(&schema.id, ID_FIELD_VALUE);
  context.write_scalar(&schema.screen_access_regulation_application_blocked_user_password, data.screen_access_regulation_application_common_info.blocked_user_password());
}

fn deserialize(
  context: &mut DeserializeCompoundValueContext,
  schema: &DataSchema,
)
  -> Result<NormalizedData, GenericError>
{
  Ok(NormalizedData { 
    id: context.deserializable_scalar(&schema.id)?, 
    screen_access_regulation_application_blocked_user_password: context.deserializable_scalar(&schema.screen_access_regulation_application_blocked_user_password)?
  })
}

pub struct DataCollection {
  name: String,
  data_schema: DataSchema,
}

impl DataCollection {
  pub fn new(
    collection_name: String,
  ) -> Self {
    Self {
      name: collection_name,
      data_schema: DataSchema {
        id: "Id".into(),
        screen_access_regulation_application_blocked_user_password: "ScreenAccessRegulationApplicationBlockedUserPassword".into(),
      }
    }
  }
}

fn collection(database: &Database) -> &DataCollection {
  &database.operating_system_integration_linux_data
}

pub fn write_define(database: &Database, code: &mut DatabaseCode) {
  let collection = collection(database);

  code.write("CREATE TABLE IF NOT EXISTS ");
  code.write(&collection.name);
  code.write(" (");
  code.write(&collection.data_schema.id);
  code.write(" INTEGER PRIMARY KEY, ");
  code.write(&collection.data_schema.screen_access_regulation_application_blocked_user_password);
  code.write(" TEXT NOT NULL) STRICT, WITHOUT ROWID;");
}

fn write_initialize_item(database: &Database, code: &mut DatabaseCode) -> NormalizedData {
  let collection = collection(database);

  code.write("INSERT INTO ");
  code.write(&collection.name);

  let data = NormalizedData::initial();
  let mut context = SerializeCompoundValueContext::new();
  context.write_u8(&collection.data_schema.id, ID_FIELD_VALUE);
  context.write_scalar(&collection.data_schema.screen_access_regulation_application_blocked_user_password, &data.screen_access_regulation_application_blocked_user_password);

  code.write(" (");
  code.write(&context.column_names);
  code.write(") VALUES (");
  code.write(&context.column_values);
  code.write(");");

  data
}

fn initialize(database: &Database) -> Result<NormalizedData, GenericError> {
  let mut code = DatabaseCode::new();
  let app = write_initialize_item(database, &mut code);
  database.execute(code.as_str())?;
  Ok(app)
}

pub fn retrieve_normalized(database: &Database) -> Result<NormalizedData, GenericError> {
  let collection = collection(database);

  let mut code = DatabaseCode::new();
  code.write("SELECT FROM ");
  code.write(&collection.name);
  code.write(" WHERE ");
  code.write(&collection.data_schema.id);
  code.write(" = ");
  serialize_scalar_value_into(&ID_FIELD_VALUE, code.as_mut());
  code.write(";");

  let connection = database.connection.lock().unwrap();
  let mut statement = connection.prepare(code.as_str()).map_err(|error|
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

  let mut context = DeserializeCompoundValueContext(item);
  deserialize(&mut context, &collection.data_schema)
}

pub fn retrieve(database: &Database) -> Result<OperatingSystemIntegrationData, GenericError> {
  let normalized_data = retrieve_normalized(database)?;

  let normalized_users = 
    operating_system_integration_linux_user
    ::retrieve_all(database)?;
  
  let normalized_user_screen_access_regulation_rules = 
    screen_access_regulation_rule
    ::retrieve_all_rules(database)?;

  let normalized_user_screen_access_regulation_policies = 
    screen_access_regulation_policy
    ::retrieve_all_policies(database)?;    

  
  let mut denormalized_users = HashMap::new();
  for normalized_user in normalized_users {
    let denormalized_user = normalized_user.denormalize(
      &normalized_user_screen_access_regulation_policies, 
      &normalized_user_screen_access_regulation_rules,
    );

    denormalized_users.insert(denormalized_user.user_id, denormalized_user);
  }

  let denormalized_data = normalized_data.denormalize(denormalized_users);

  Ok(denormalized_data)
}