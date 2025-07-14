use crate::user_screen_access_regulation::*;
use super::screen_access_regulation_rule_integration::NormalizedRule;
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
  enabled_duration: String,  
  remaining_enabled_duration: String,  
  previous_synchronization_time: String,
  position: String,
}

#[derive(Debug, Clone)]
pub struct NormalizedPolicy {
  pub(super) id: Uuid,
  pub(super) name: PolicyName,
  pub(super) user_id: Uuid,
  pub(super) enabler: PolicyEnabler,
  pub(super) position: usize,
}

impl NormalizedPolicy {
  pub fn denormalize(self, rules: &Vec<NormalizedRule>) -> Policy {
    let rules = rules
      .iter()
      .filter(|rule| rule.user_id == self.user_id && rule.policy_id == self.id)
      .map(|rule| rule.clone().denormalize())
      .collect();
    
    Policy::pack(
      self.id,
      self.name,
      rules,
      self.enabler,
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
  context.write_scalar(&fields.enabled_duration, &policy.enabler().enabled_duration());
  context.write_scalar(&fields.remaining_enabled_duration, &policy.enabler().remaining_enabled_duration());
  context.write_scalar(&fields.previous_synchronization_time, &policy.enabler().previous_synchronization_time());
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
  let enabler_duration = context.deserializable_scalar(&fields.enabled_duration)?;
  let enabler_remaining_duration = context.deserializable_scalar(&fields.remaining_enabled_duration)?;
  let enabler_previous_synchronization_time = context.deserializable_scalar(&fields.previous_synchronization_time)?;

  Ok(NormalizedPolicy {
    id, 
    name, 
    user_id,
    position,
    enabler: PolicyEnabler::pack(
      enabler_duration, 
      enabler_remaining_duration, 
      enabler_previous_synchronization_time,
    ),
  })
}

pub struct PolicyCollection {
  name: String,
  fields: PolicyFields,
}

impl PolicyCollection {
  pub fn new(
    collection_name: String,
    policy_id_field: String,
    policy_name_field: String,
    policy_user_id_field: String,
    policy_enabler_duration_field: String,
    policy_enabler_remaining_duration_field: String,
    policy_enabler_previous_synchronization_time_field: String,
  ) -> Self {
    Self {
      name: collection_name,
      fields: PolicyFields {
        id: policy_id_field,
        name: policy_name_field,
        user_id: policy_user_id_field,
        enabled_duration: policy_enabler_duration_field,
        remaining_enabled_duration: policy_enabler_remaining_duration_field,
        previous_synchronization_time: policy_enabler_previous_synchronization_time_field,
        position: String::new(),
      }
    }
  }
}


pub fn write_define(database: &Database, code: &mut DatabaseCode) {
  let collection = &database.user_screen_access_regulation_policy;

  code.write("CREATE TABLE IF NOT EXISTS "); 
  code.write(&collection.name); 
  code.write(" (");
  code.write(&collection.fields.id);
  code.write(" TEXT PRIMARY KEY, ");
  code.write(&collection.fields.user_id);
  code.write(" TEXT NOT NULL, ");
  code.write(&collection.fields.name);
  code.write(" TEXT NOT NULL, ");
  code.write(&collection.fields.enabled_duration);
  code.write(" INTEGER NOT NULL, ");
  code.write(&collection.fields.remaining_enabled_duration);
  code.write(" INTEGER NOT NULL, ");
  code.write(&collection.fields.previous_synchronization_time);
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
  let collection = &database.user_screen_access_regulation_policy;

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
  serialize_policy(&mut context, policy, user_id, position, &database.user_screen_access_regulation_policy.fields);
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
  let collection = &database.user_screen_access_regulation_policy;

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
  let collection = &database.user_screen_access_regulation_policy;

  draft.write("DELETE FROM ");
  draft.write(&collection.name);
  draft.write(" WHERE ");
  draft.write(&collection.fields.user_id);
  draft.write(" = ");
  serialize_scalar_value_into(user_id, draft.as_mut());
  draft.write(";");
}

pub fn write_retrieve_all_policies(database: &Database, code: &mut DatabaseCode) {
  let collection = &database.user_screen_access_regulation_policy;

  code.write("SELECT * FROM ");
  code.write(&collection.name);
  code.write(";");
}

pub fn retrieve_all_policies(database: &Database) -> Result<Vec<NormalizedPolicy>, GenericError> {
  let mut code = DatabaseCode::new();
  write_retrieve_all_policies(database, &mut code);

  let mut statement = database.connection.prepare(code.as_str()).map_err(|error| 
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
    rules.push(deserialize_policy(&mut context, &database.user_screen_access_regulation_policy.fields)?);
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
  draft.draft.write_scalar(&database.user_screen_access_regulation_policy.fields.name, new_value);
}

pub fn update_name(database: &Database, policy_id: &Uuid, new_value: &PolicyName) -> Result<(), GenericError> {
  let mut draft = PolicyUpdateDraft::new();
  write_name(database, &mut draft, new_value);
  commit_policy_update_draft(database, &draft, policy_id)
}

pub fn enabled_condition(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &CountdownTimer) {
  draft.draft.write_scalar(&database.user_screen_access_regulation_policy.fields.enabled_duration, &new_value.duration());
  draft.draft.write_scalar(&database.user_screen_access_regulation_policy.fields.remaining_enabled_duration, &new_value.remaining_duration());
  draft.draft.write_scalar(&database.user_screen_access_regulation_policy.fields.previous_synchronization_time, &new_value.previous_synchronization_time());
}

pub fn write_enabled_duration(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &Duration) {
  draft.draft.write_scalar(&database.user_screen_access_regulation_policy.fields.enabled_duration, new_value);
}

pub fn update_enabled_duration(database: &Database, policy_id: &Uuid, new_value: Duration) -> Result<(), GenericError> {
  let mut draft = PolicyUpdateDraft::new();
  write_enabled_duration(database, &mut draft, &new_value);
  commit_policy_update_draft(database, &draft, policy_id)
}

pub fn write_remaining_enabled_duration(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &Duration) {
  draft.draft.write_scalar(&database.user_screen_access_regulation_policy.fields.remaining_enabled_duration, new_value);
}

pub fn write_previous_synchronization_time(database: &Database, draft: &mut PolicyUpdateDraft, new_value: &DateTime) {
  draft.draft.write_scalar(&database.user_screen_access_regulation_policy.fields.previous_synchronization_time, new_value);
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

  let collection = &database.user_screen_access_regulation_policy;

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