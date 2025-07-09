use super::*;

pub struct PolicyFields {
  id: String,
  user_id: String,
  name: String,
  enabler_countdown_timer_duration: String,  
  enabler_countdown_timer_remaining_duration: String,  
  enabler_countdown_timer_previous_synchronization_time: String,
  position: String,
}

pub struct PolicyCollection<'a> {
  name: String,
  fields: PolicyFields,
  database: &'a Database,
}

impl<'a> PolicyCollection<'a> {
  pub fn write_definition_into(&self, into: &mut String) {
    into.push_str("CREATE TABLE IF NOT EXISTS "); 
    into.push_str(&self.name); 
    into.push_str(" (");
    into.push_str(&self.fields.id);
    into.push_str(" TEXT PRIMARY KEY, ");
    into.push_str(&self.fields.name);
    into.push_str(" TEXT NOT NULL, ");
    into.push_str(&self.fields.enabler_countdown_timer_duration);
    into.push_str(" INTEGER NOT NULL, ");
    into.push_str(&self.fields.enabler_countdown_timer_remaining_duration);
    into.push_str(" INTEGER NOT NULL, ");
    into.push_str(&self.fields.enabler_countdown_timer_previous_synchronization_time);
    into.push_str(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
  }
}

fn serialize_policy(
  context: &mut SerializeCompoundValueContext,
  user_id: &Uuid,
  policy: &Policy,
  fields: &PolicyFields,
) {
  context.write_scalar(&fields.id, policy.id());
  context.write_scalar(&fields.name, policy.name());
  context.write_scalar(&fields.user_id, user_id);
  context.write_scalar(&fields.enabler_countdown_timer_duration, &policy.enabler().unpack_ref().duration());
  context.write_scalar(&fields.enabler_countdown_timer_remaining_duration, &policy.enabler().unpack_ref().remaining_duration());
  context.write_scalar(&fields.enabler_countdown_timer_previous_synchronization_time, &policy.enabler().unpack_ref().previous_synchronization_time());
}


#[derive(Debug, Clone)]
pub struct NormalizedPolicy {
  pub(super) id: Uuid,
  pub(super) name: PolicyName,
  pub(super) user_id: Uuid,
  pub(super) enabler: PolicyEnabler,
}

impl NormalizedPolicy {
  pub fn denormalize(
    self, 
    user_id: &Uuid,
    normalized_rules: &Vec<NormalizedRule>,
  ) -> Policy {
    Policy::pack(
      self.id,
      self.name,
      normalized_rules
        .iter()
        .filter(|rule| rule.user_id == *user_id && rule.policy_id == self.id)
        .map(|rule| rule.clone().denormalize())
        .collect(),
      self.enabler,
    )
  }
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
  let enabler_duration = context.deserializable_scalar(&fields.enabler_countdown_timer_duration)?;
  let enabler_remaining_duration = context.deserializable_scalar(&fields.enabler_countdown_timer_remaining_duration)?;
  let enabler_previous_synchronization_time = context.deserializable_scalar(&fields.enabler_countdown_timer_previous_synchronization_time)?;

  Ok(NormalizedPolicy {
    id, 
    name, 
    user_id,
    enabler: PolicyEnabler::pack(CountdownTimer::new_with_state(
      enabler_duration, 
      enabler_remaining_duration, 
      enabler_previous_synchronization_time,
    )),
  })
}

pub struct PolicyUpdateDraft<'a> {
  draft: CollectionItemUpdateDraft,
  collection: &'a PolicyCollection<'a>,
}

impl<'a> PolicyUpdateDraft<'a> {
  pub fn new(collection: &'a PolicyCollection) -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new(),
      collection,
    }
  }

  pub fn update_name(&mut self, new_value: &PolicyName) {
    self.draft.write_scalar(&self.collection.name, new_value);
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

  pub fn commit(&self, policy_id: &Uuid) -> Result<(), GenericError> {
    let Some(updates) = self.draft.finish() else {
      return Ok(());
    };

    let mut code = String::new();
    code.push_str("UPDATE ");
    code.push_str(&self.collection.name);
    code.push_str(" ");
    code.push_str(&updates);
    code.push_str(" WHERE ");

    code.push_str(&self.collection.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, &mut code);

    code.push_str(";");

    self.collection.database.execute(&code)
  }
}

impl<'a> PolicyCollection<'a> {
  pub fn create_update_draft(&self) -> PolicyUpdateDraft {
    PolicyUpdateDraft::new(self)
  }

  pub fn add_policy(
    &self,
    code: &mut String,
    user_id: &Uuid,
    policy: &Policy,
  ) {
    code.push_str("INSERT INTO ");
    code.push_str(&self.name);

    let mut context = SerializeCompoundValueContext::new();
    serialize_policy(&mut context, user_id, policy, &self.fields);

    code.push_str("(");
    code.push_str(&context.column_names);
    code.push_str(") VALUES (");
    code.push_str(&context.column_values);
    code.push_str(");");
  }

  pub fn delete_policy(&self, code: &mut String, policy_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);
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
    let Some(updates) = policy_update_draft.draft.finish() else {
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
}