use crate::user_screen_access_regulation::*;
use super::screen_access_regulation_rule_collection::NormalizedRule;
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
  enabler_countdown_timer_duration: String,  
  enabler_countdown_timer_remaining_duration: String,  
  enabler_countdown_timer_previous_synchronization_time: String,
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
  context.write_scalar(&fields.enabler_countdown_timer_duration, &policy.enabler().unpack_ref().duration());
  context.write_scalar(&fields.enabler_countdown_timer_remaining_duration, &policy.enabler().unpack_ref().remaining_duration());
  context.write_scalar(&fields.enabler_countdown_timer_previous_synchronization_time, &policy.enabler().unpack_ref().previous_synchronization_time());
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
  let enabler_duration = context.deserializable_scalar(&fields.enabler_countdown_timer_duration)?;
  let enabler_remaining_duration = context.deserializable_scalar(&fields.enabler_countdown_timer_remaining_duration)?;
  let enabler_previous_synchronization_time = context.deserializable_scalar(&fields.enabler_countdown_timer_previous_synchronization_time)?;

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

pub struct PolicyUpdateDraft<'a> {
  draft: CollectionItemUpdateDraft,
  database: &'a Database,
  collection: &'a PolicyCollection
}

impl<'a> PolicyUpdateDraft<'a> {
  pub fn new(database: &'a Database) -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new(),
      database,
      collection: &database.user_screen_access_regulation_policy,
    }
  }

  pub fn update_name(&mut self, new_value: &PolicyName) {
    self.draft.write_scalar(&self.collection.fields.name, new_value);
  }

  pub fn update_enabler_timer(&mut self, new_value: &CountdownTimer) {
    self.draft.write_scalar(&self.collection.fields.enabler_countdown_timer_duration, &new_value.duration());
    self.draft.write_scalar(&self.collection.fields.enabler_countdown_timer_remaining_duration, &new_value.remaining_duration());
    self.draft.write_scalar(&self.collection.fields.enabler_countdown_timer_previous_synchronization_time, &new_value.previous_synchronization_time());
  }

  pub fn update_enabler_timer_duration(&mut self, new_value: &Duration) {
    self.draft.write_scalar(&self.collection.fields.enabler_countdown_timer_duration, new_value);
  }

  pub fn update_enabler_timer_remaining_duration(&mut self, new_value: &Duration) {
    self.draft.write_scalar(&self.collection.fields.enabler_countdown_timer_remaining_duration, new_value);
  }

  pub fn update_enabler_timer_previous_synchronization_time(&mut self, new_value: &DateTime) {
    self.draft.write_scalar(&self.collection.fields.enabler_countdown_timer_previous_synchronization_time, new_value);
  }

  pub fn maybe_write_update_statement_into(&self, code: &mut String, policy_id: &Uuid) {
    let Some(updates) = self.draft.updates() else {
      return;
    };

    code.push_str("UPDATE ");
    code.push_str(&self.collection.name);
    code.push_str(" ");
    code.push_str(&updates);
    code.push_str(" WHERE ");

    code.push_str(&self.collection.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);

    code.push_str(";");
  }

  pub fn commit(&self, policy_id: &Uuid) -> Result<(), GenericError> {
    let mut code = String::new();
    self.maybe_write_update_statement_into(&mut code, policy_id);
    self.database.execute(&code)
  }
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
        enabler_countdown_timer_duration: policy_enabler_duration_field,
        enabler_countdown_timer_remaining_duration: policy_enabler_remaining_duration_field,
        enabler_countdown_timer_previous_synchronization_time: policy_enabler_previous_synchronization_time_field,
        position: String::new(),
      }
    }
  }
  
  pub fn write_definition_into(&self, code: &mut String) {
    code.push_str("CREATE TABLE IF NOT EXISTS "); 
    code.push_str(&self.name); 
    code.push_str(" (");
    code.push_str(&self.fields.id);
    code.push_str(" TEXT PRIMARY KEY, ");
    code.push_str(&self.fields.user_id);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.name);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.enabler_countdown_timer_duration);
    code.push_str(" INTEGER NOT NULL, ");
    code.push_str(&self.fields.enabler_countdown_timer_remaining_duration);
    code.push_str(" INTEGER NOT NULL, ");
    code.push_str(&self.fields.enabler_countdown_timer_previous_synchronization_time);
    code.push_str(" INTEGER NOT NULL, ");
    code.push_str(&self.fields.position);
    code.push_str(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
  }

  pub fn add_policy(
    &self,
    code: &mut String,
    policy: &Policy,
    user_id: &Uuid,
    position: usize,
  ) {
    code.push_str("UPDATE ");
    code.push_str(&self.name);
    code.push_str(" SET ");
    code.push_str(&self.fields.position);
    code.push_str(" = ");
    code.push_str(&self.fields.position);
    code.push_str(" + 1 WHERE ");
    code.push_str(&self.fields.position);
    code.push_str(" >= ");
    serialize_scalar_value_into(&position, code);
    code.push_str(" AND ");
    code.push_str(&self.fields.user_id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, code);
    code.push_str(";");


    code.push_str("INSERT INTO ");
    code.push_str(&self.name);
    let mut context = SerializeCompoundValueContext::new();
    serialize_policy(&mut context, policy, user_id, position, &self.fields);
    code.push_str("(");
    code.push_str(&context.column_names);
    code.push_str(") VALUES (");
    code.push_str(&context.column_values);
    code.push_str(");");
  }

  pub fn delete_policy(
    &self, 
    code: &mut String,
    user_id: &Uuid,
    policy_id: &Uuid,
    position: usize,
  ) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);
    code.push_str(";");

    code.push_str("UPDATE ");
    code.push_str(&self.name);
    code.push_str(" SET ");
    code.push_str(&self.fields.position);
    code.push_str(" = ");
    code.push_str(" - 1 WHERE ");
    code.push_str(&self.fields.position);
    code.push_str(" > ");
    serialize_scalar_value_into(&position, code);
    code.push_str(" AND ");
    code.push_str(&self.fields.user_id);
    serialize_scalar_value_into(user_id, code);
    code.push_str(";");
  }

  pub fn delete_policies_of_user(&self, code: &mut String, user_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.user_id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, code);
    code.push_str(";");
  }

  pub fn update_policy(&self, code: &mut String, policy_update_draft: &PolicyUpdateDraft, policy_id: &Uuid) {
    let Some(updates) = policy_update_draft.draft.updates() else {
      return;
    };

    code.push_str("UPDATE FROM ");
    code.push_str(&self.name);
    code.push_str(" ");
    code.push_str(&updates);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);
    code.push_str(";");
  }

  pub fn write_retrieve_all_policies(&self, code: &mut String) {
    code.push_str("SELECT * FROM ");
    code.push_str(&self.name);
    code.push_str(";");
  }

  pub fn retrieve_all_policies(&self, database: &Database) -> Result<Vec<NormalizedPolicy>, GenericError> {
    let mut code = String::new();
    self.write_retrieve_all_policies(&mut code);

    let mut statement = database.connection.prepare(&code).map_err(|error| 
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
      rules.push(deserialize_policy(&mut context, &self.fields)?);
    }
  }

  pub fn create_policy_update_draft<'a>(&self, database: &'a Database) -> PolicyUpdateDraft<'a> {
    PolicyUpdateDraft::new(database)
  }

  pub fn create_collection_update_draft<'a>(&self, database: &'a Database) -> PolicyCollectionUpdateDraft<'a> {
    PolicyCollectionUpdateDraft { 
      database
    }
  }
}

pub struct PolicyCollectionUpdateDraft<'a> {
  database: &'a Database
}