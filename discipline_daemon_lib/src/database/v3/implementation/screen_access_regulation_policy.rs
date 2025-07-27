use crate::operating_system_integration::UserId;
use crate::screen_access_regulation::*;
use super::screen_access_regulation_rule::NormalizedRule;
use crate::{Uuid, CountdownTimer, Duration, DateTime};
use super::*;

impl SerializableScalarValue for PolicyName {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(self.as_ref());
  }
}

impl DeserializableScalarValue for PolicyName {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_string()
      .and_then(PolicyName::new)
      .map_err(|error| error.change_context("deserializing PolicyName"))    
  }
}

pub struct PolicyFields {
  id: String,
  user_id: String,
  name: String,
  is_enabled: String,
  protection_duration: String,  
  protection_remaining_duration: String,  
  protection_previous_synchronization_time: String,
  position: String,
}

#[derive(Debug, Clone)]
pub struct NormalizedPolicy {
  pub(super) id: Uuid,
  pub(super) name: PolicyName,
  pub(super) user_id: UserId,
  pub(super) is_enabled: bool,
  pub(super) protection_duration: Duration,
  pub(super) protection_remaining_duration: Duration,
  pub(super) protection_previous_synchronization_time: DateTime,
  pub(super) position: usize,
}

impl NormalizedPolicy {
  pub fn denormalize(self, rules: &Vec<NormalizedRule>) -> Policy {
    let rules = rules
      .iter()
      .filter(|rule| rule.user_id == self.user_id && rule.policy_id == self.id)
      .map(|rule| rule.clone().denormalize())
      .collect();
    
    Policy::from_fields(
      self.id,
      self.name,
      rules,
      self.is_enabled,
      self.protection_duration,
      self.protection_remaining_duration,
      self.protection_previous_synchronization_time,
    )
  }
}

fn serialize_policy(
  context: &mut SerializeCompoundValueContext,
  policy: &Policy,
  user_id: &Uuid,
  position: usize,
  fields: &PolicyFields,
) {
  context.write_scalar(&fields.id, policy.id());
  context.write_scalar(&fields.name, policy.name());
  context.write_scalar(&fields.user_id, user_id);
  context.write_usize(&fields.position, position);
  context.write_scalar(&fields.is_enabled, &policy.is_enabled());
  context.write_scalar(&fields.protection_duration, &policy.protector().duration());
  context.write_scalar(&fields.protection_remaining_duration, &policy.protector().remaining_duration());
  context.write_scalar(&fields.protection_previous_synchronization_time, &policy.protector().previous_synchronization_time());
}

fn deserialize_policy(
  context: &mut DeserializeCompoundValueContext,
  fields: &PolicyFields,
)
  -> Result<NormalizedPolicy, GenericError>
{
  let id = context.deserializable_scalar(&fields.id)?;
  let name = context.deserializable_scalar(&fields.name)?;
  let user_id = context.deserializable_scalar(&fields.user_id)?;
  let position = context.deserializable_scalar(&fields.position)?;
  let is_enabled = context.deserializable_scalar(&fields.is_enabled)?;
  let protection_duration = context.deserializable_scalar(&fields.protection_duration)?;
  let protection_remaining_duration = context.deserializable_scalar(&fields.protection_remaining_duration)?;
  let protection_previous_synchronization_time = context.deserializable_scalar(&fields.protection_previous_synchronization_time)?;

  Ok(NormalizedPolicy {
    id, 
    name, 
    user_id,
    position,
    is_enabled,
    protection_duration,
    protection_remaining_duration,
    protection_previous_synchronization_time,
  })
}

pub struct PolicyCollection {
  name: String,
  fields: PolicyFields,
}

impl PolicyCollection {
  pub fn new(
    collection_name: String,
  ) -> Self {
    Self {
      name: collection_name,
      fields: PolicyFields {
        id: "Id".into(),
        name: "Name".into(),
        is_enabled: "IsEnabled".into(),
        protection_duration: "ProtectionDuration".into(),
        protection_remaining_duration: "ProtectionRemainingDuration".into(),
        protection_previous_synchronization_time: "ProtectionPreviousSynchronizationTime".into(),
        user_id: "UserId".into(),
        position: "Position".into(),

      }
    }
  }
}


pub fn write_define(database: &Database, code: &mut DatabaseCode) {
  let collection = &database.screen_access_regulation_policy;

  code.write("CREATE TABLE IF NOT EXISTS "); 
  code.write(&collection.name); 
  code.write(" (");
  code.write(&collection.fields.id);
  code.write(" TEXT PRIMARY KEY, ");
  code.write(&collection.fields.user_id);
  code.write(" TEXT NOT NULL, ");
  code.write(&collection.fields.name);
  code.write(" TEXT NOT NULL, ");
  code.write(&collection.fields.protection_duration);
  code.write(" INTEGER NOT NULL, ");
  code.write(&collection.fields.protection_remaining_duration);
  code.write(" INTEGER NOT NULL, ");
  code.write(&collection.fields.protection_previous_synchronization_time);
  code.write(" INTEGER NOT NULL, ");
  code.write(&collection.fields.position);
  code.write(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
}

pub fn write_add_policy(
  database: &Database,
  draft: &mut DatabaseCode,
  policy: &Policy,
  user_id: &Uuid,
  position: usize,
) {
  let collection = &database.screen_access_regulation_policy;

  draft.write("UPDATE ");
  draft.write(&collection.name);
  draft.write(" SET ");
  draft.write(&collection.fields.position);
  draft.write(" = ");
  draft.write(&collection.fields.position);
  draft.write(" + 1 WHERE ");
  draft.write(&collection.fields.position);
  draft.write(" >= ");
  serialize_scalar_value_into(&position, draft.as_mut());
  draft.write(" AND ");
  draft.write(&collection.fields.user_id);
  draft.write(" = ");
  serialize_scalar_value_into(user_id, draft.as_mut());
  draft.write(";");


  draft.write("INSERT INTO ");
  draft.write(&collection.name);
  let mut context = SerializeCompoundValueContext::new();
  serialize_policy(&mut context, policy, user_id, position, &database.screen_access_regulation_policy.fields);
  draft.write("(");
  draft.write(&context.column_names);
  draft.write(") VALUES (");
  draft.write(&context.column_values);
  draft.write(");");
}

pub fn add_policy(
  database: &Database,
  policy: &Policy,
  user_id: &Uuid,
  // position: usize,
) -> Result<(), GenericError> {
  todo!()
  // let mut draft = DatabaseCode::new();
  // write_add_policy(database, &mut draft, policy, user_id, position);
  // database.execute(draft.as_str())
}

pub fn write_delete_policy(
  database: &Database, 
  draft: &mut DatabaseCode,
  user_id: &Uuid,
  policy_id: &Uuid,
  position: usize,
) {
  let collection = &database.screen_access_regulation_policy;

  draft.write("DELETE FROM ");
  draft.write(&collection.name);
  draft.write(" WHERE ");
  draft.write(&collection.fields.id);
  draft.write(" = ");
  serialize_scalar_value_into(policy_id, draft.as_mut());
  draft.write(";");

  draft.write("UPDATE ");
  draft.write(&collection.name);
  draft.write(" SET ");
  draft.write(&collection.fields.position);
  draft.write(" = ");
  draft.write(" - 1 WHERE ");
  draft.write(&collection.fields.position);
  draft.write(" > ");
  serialize_scalar_value_into(&position, draft.as_mut());
  draft.write(" AND ");
  draft.write(&collection.fields.user_id);
  serialize_scalar_value_into(user_id, draft.as_mut());
  draft.write(";");
}

pub fn delete_policy(
  database: &Database, 
  // user_id: &Uuid,
  policy_id: &Uuid,
  // position: usize,
) -> Result<(), GenericError> {
  todo!()
  // let mut draft = DatabaseCode::new();
  // write_delete_policy(database, &mut draft, policy, user_id, position);
  // database.execute(draft.as_str())
}

pub fn write_delete_policies_of_user(database: &Database, draft: &mut DatabaseCode, user_id: &Uuid) {
  let collection = &database.screen_access_regulation_policy;

  draft.write("DELETE FROM ");
  draft.write(&collection.name);
  draft.write(" WHERE ");
  draft.write(&collection.fields.user_id);
  draft.write(" = ");
  serialize_scalar_value_into(user_id, draft.as_mut());
  draft.write(";");
}

pub fn write_retrieve_all_policies(database: &Database, code: &mut DatabaseCode) {
  let collection = &database.screen_access_regulation_policy;

  code.write("SELECT * FROM ");
  code.write(&collection.name);
  code.write(";");
}

pub fn retrieve_all_policies(database: &Database) -> Result<Vec<NormalizedPolicy>, GenericError> {
  let mut code = DatabaseCode::new();
  write_retrieve_all_policies(database, &mut code);

  let connection = database.connection.lock().unwrap();
  let mut statement = connection.prepare(code.as_str()).map_err(|error| 
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
    rules.push(deserialize_policy(&mut context, &database.screen_access_regulation_policy.fields)?);
  }
}

pub struct PolicyUpdateDraft {
  draft: CollectionItemUpdateDraft
}

impl PolicyUpdateDraft {
  pub fn new() -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new(),
    }
  }
}

pub fn write_name(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &PolicyName) {
  draft.draft.write_scalar(&database.screen_access_regulation_policy.fields.name, new_value);
}

pub fn update_name(database: &Database, policy_id: &Uuid, new_value: &PolicyName) -> Result<(), GenericError> {
  let mut draft = PolicyUpdateDraft::new();
  write_name(database, &mut draft, new_value);
  commit_policy_update_draft(database, &draft, policy_id)
}

pub fn enabled_condition(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &CountdownTimer) {
  draft.draft.write_scalar(&database.screen_access_regulation_policy.fields.protection_duration, &new_value.duration());
  draft.draft.write_scalar(&database.screen_access_regulation_policy.fields.protection_remaining_duration, &new_value.remaining_duration());
  draft.draft.write_scalar(&database.screen_access_regulation_policy.fields.protection_previous_synchronization_time, &new_value.previous_synchronization_time());
}

pub fn write_enabled_duration(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &Duration) {
  draft.draft.write_scalar(&database.screen_access_regulation_policy.fields.protection_duration, new_value);
}

pub fn update_enabled_duration(database: &Database, policy_id: &Uuid, new_value: Duration) -> Result<(), GenericError> {
  let mut draft = PolicyUpdateDraft::new();
  write_enabled_duration(database, &mut draft, &new_value);
  commit_policy_update_draft(database, &draft, policy_id)
}

pub fn write_remaining_enabled_duration(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &Duration) {
  draft.draft.write_scalar(&database.screen_access_regulation_policy.fields.protection_remaining_duration, new_value);
}

pub fn write_previous_synchronization_time(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &DateTime) {
  draft.draft.write_scalar(&database.screen_access_regulation_policy.fields.protection_previous_synchronization_time, new_value);
}

// pub fn write_update_policy(database: &Database, database_update_draft: &mut DatabaseCode, policy_update_draft: &PolicyUpdateDraft, policy_id: &Uuid) {
//   let Some(updates) = policy_update_draft.draft.updates() else {
//     return;
//   };

//   let collection = &database.user_screen_access_regulation_policy;

//   database_update_draft.write("UPDATE FROM ");
//   database_update_draft.write(&collection.name);
//   database_update_draft.write(" ");
//   database_update_draft.write(&updates);
//   database_update_draft.write(" WHERE ");
//   database_update_draft.write(&collection.fields.id);
//   database_update_draft.write(" = ");
//   serialize_scalar_value_into(policy_id, database_update_draft.as_mut());
//   database_update_draft.write(";");
// }

pub fn write_update_policy(database: &Database, database_update_draft: &mut DatabaseCode, policy_update_draft: &PolicyUpdateDraft, policy_id: &Uuid) {
  let Some(updates) = policy_update_draft.draft.updates() else {
    return;
  };

  let collection = &database.screen_access_regulation_policy;

  database_update_draft.write("UPDATE ");
  database_update_draft.write(&collection.name);
  database_update_draft.write(" ");
  database_update_draft.write(&updates);
  database_update_draft.write(" WHERE ");

  database_update_draft.write(&collection.fields.id);
  database_update_draft.write(" = ");
  serialize_scalar_value_into(policy_id, database_update_draft.as_mut());

  database_update_draft.write(";");
}

pub fn commit_policy_update_draft(database: &Database, policy_update_draft: &PolicyUpdateDraft, policy_id: &Uuid) -> Result<(), GenericError> {
  let mut database_update_draft = DatabaseCode::new();
  write_update_policy(database, &mut database_update_draft, policy_update_draft, policy_id);
  database.execute(database_update_draft.as_str())
}