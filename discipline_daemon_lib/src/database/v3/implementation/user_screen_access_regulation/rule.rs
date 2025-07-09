use super::*;

pub struct RuleFields {
  id: String,
  user_id: String,
  policy_id: String,
  activator_enum_type: String,
  activator_enum_data_1: String,
  activator_enum_data_2: String,
  position: String,
}

impl RuleFields {
  fn activator_weekday(&self) -> &String {
    &self.activator_enum_data_1
  }

  fn activator_weekday_range_from(&self) -> &String {
    &self.activator_enum_data_1
  }

  fn activator_weekday_range_till(&self) -> &String {
    &self.activator_enum_data_2
  }

  fn activator_time_range_from(&self) -> &String {
    &self.activator_enum_data_1
  }

  fn activator_time_range_till(&self) -> &String {
    &self.activator_enum_data_2
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) id: Uuid,
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
  pub(super) activator: RuleActivator,
}

impl NormalizedRule {
  pub fn denormalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
    }
  }
}

fn serialize_rule(
  context: &mut SerializeCompoundValueContext,
  rule: &Rule,
  rule_fields: &RuleFields
) {
  context.write_scalar(&rule_fields.id, rule.id());
  
  match rule.activator() {
    RuleActivator::AllTheTime => {
      context.write_scalar(&rule_fields.activator_enum_type, &RuleActivatorType::AllTheTime);
      context.write_null(&rule_fields.activator_enum_data_1);
      context.write_null(&rule_fields.activator_enum_data_2);
    }
    RuleActivator::OnWeekday(weekday) => {
      context.write_scalar(&rule_fields.activator_enum_type, &RuleActivatorType::OnWeekday);
      context.write_scalar(&rule_fields.activator_enum_data_1, weekday);
      context.write_null(&rule_fields.activator_enum_data_2);
    }
    RuleActivator::InTimeRange(range) => {
      context.write_scalar(&rule_fields.activator_enum_type, &RuleActivatorType::InTimeRange);
      context.write_u32(&rule_fields.activator_enum_data_1, range.from_as_timestamp());
      context.write_u32(&rule_fields.activator_enum_data_2, range.till_as_timestamp());
    }
    RuleActivator::InWeekdayRange(range) => {
      context.write_scalar(&rule_fields.activator_enum_type, &RuleActivatorType::InWeekdayRange);
      context.write_u32(&rule_fields.activator_enum_data_1, range.from_as_timestamp());
      context.write_u32(&rule_fields.activator_enum_data_2, range.till_as_timestamp());
    }
  }
}

fn deserialize_rule(
  context: &mut DeserializeCompoundValueContext,
  fields: &RuleFields,
) 
  -> Result<NormalizedRule, GenericError> 
{
  let id = context.deserializable_scalar(&fields.id)?;
  let user_id = context.deserializable_scalar(&fields.user_id)?;
  let policy_id = context.deserializable_scalar(&fields.policy_id)?;
  let activator_type = context.deserializable_scalar(&fields.activator_enum_type)?;
  let activator = match activator_type {
    RuleActivatorType::AllTheTime => {
      RuleActivator::AllTheTime
    }
    RuleActivatorType::OnWeekday => {
      let weekday = context.deserializable_scalar(&fields.activator_enum_type)?;
      RuleActivator::OnWeekday(weekday)
    }
    RuleActivatorType::InTimeRange => {
      let from = context.deserializable_scalar(&fields.activator_enum_data_1)?;
      let till = context.deserializable_scalar(&fields.activator_enum_data_2)?;
      let range = TimeRange::from_timestamps(from, till)?;
      RuleActivator::InTimeRange(range)
    }
    RuleActivatorType::InWeekdayRange => {
      let from = context.deserializable_scalar(&fields.activator_enum_data_1)?;
      let till = context.deserializable_scalar(&fields.activator_enum_data_2)?;
      let range = WeekdayRange::from_timestamps(from, till)?;
      RuleActivator::InWeekdayRange(range)
    }
  };

  Ok(NormalizedRule {
    id,
    user_id,
    activator,
    policy_id,
  })
}

pub struct RuleCollection<'a> {
  name: String,
  fields: RuleFields,
  database: &'a Database,
}

pub struct RuleUpdateDraft<'a> {
  draft: CollectionItemUpdateDraft,
  collection: &'a RuleCollection<'a>,
}

impl<'a> RuleUpdateDraft<'a> {
  pub fn new(collection: &'a RuleCollection) -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new(),
      collection,
    }
  }

  pub fn update_activator(&mut self, new_value: &RuleActivator) {
    match new_value {
      RuleActivator::AllTheTime => {
        self.draft.write_scalar(&self.collection.fields.activator_enum_type, &RuleActivatorType::AllTheTime);
      }
      RuleActivator::OnWeekday(weekday) => {
        self.draft.write_scalar(&self.collection.fields.activator_enum_type, &RuleActivatorType::OnWeekday);
        self.draft.write_scalar(&self.collection.fields.activator_enum_data_1, weekday);
      }
      RuleActivator::InTimeRange(range) => {
        self.draft.write_scalar(&self.collection.fields.activator_enum_type, &RuleActivatorType::InTimeRange);
        self.draft.write_u32(&self.collection.fields.activator_enum_data_1, range.from_as_timestamp());
        self.draft.write_u32(&self.collection.fields.activator_enum_data_2, range.till_as_timestamp());
      }
      RuleActivator::InWeekdayRange(range) => {
        self.draft.write_scalar(&self.collection.fields.activator_enum_type, &RuleActivatorType::InWeekdayRange);
        self.draft.write_u32(&self.collection.fields.activator_enum_data_1, range.from_as_timestamp());
        self.draft.write_u32(&self.collection.fields.activator_enum_data_2, range.till_as_timestamp());
      }
    }
  }

  pub fn update_activator_weekday(&mut self, new_value: &Weekday) {
    self.draft.write_scalar(&self.collection.fields.activator_enum_type, &RuleActivatorType::OnWeekday);
    self.draft.write_scalar(&self.collection.fields.activator_enum_data_1, new_value);
  }
  
  pub fn update_activator_time_range(&mut self, new_value: &TimeRange) {
    self.draft.write_scalar(&self.collection.fields.activator_enum_type, &RuleActivatorType::InTimeRange);
    self.draft.write_u32(&self.collection.fields.activator_enum_data_1, new_value.from_as_timestamp());
    self.draft.write_u32(&self.collection.fields.activator_enum_data_2, new_value.till_as_timestamp());
  }

  pub fn update_activator_weekday_range(&mut self, new_value: &WeekdayRange) {
    self.draft.write_scalar(&self.collection.fields.activator_enum_type, &RuleActivatorType::InWeekdayRange);
    self.draft.write_u32(&self.collection.fields.activator_enum_data_1, new_value.from_as_timestamp());
    self.draft.write_u32(&self.collection.fields.activator_enum_data_2, new_value.till_as_timestamp());
  }

  pub fn commit(
    &self, 
    rule_id: &Uuid,
  ) 
    -> Result<(), GenericError> 
  {
    let Some(updates) = self.draft.finish() else {
      return Ok(());
    };

    let mut code = String::new();
    code.push_str("UPDATE ");
    code.push_str(&self.collection.name);
    code.push_str(" ");
    code.push_str(&updates);
    code.push_str(" WHERE ");

    // code.push_str(&self.collection.fields.user_id);
    // code.push_str(" = ");
    // serialize_scalar_value_into(user_id, &mut code);

    // code.push_str(" AND ");
    // code.push_str(&self.collection.fields.policy_id);
    // code.push_str(" = ");
    // serialize_scalar_value_into(policy_id, &mut code);

    // code.push_str(" AND ");
    code.push_str(&self.collection.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(rule_id, &mut code);
    code.push_str(";");

    self.collection.database.execute(&code)
  }
}

pub struct RuleCollectionUpdateDraft<'a> {
  code: String,
  collection: &'a RuleCollection<'a>,
}

impl<'a> RuleCollectionUpdateDraft<'a> {
  pub fn new(collection: &'a RuleCollection) -> Self {
    Self {
      code: String::new(),
      collection,
    }
  }

  pub fn add_rule(&mut self, rule: &Rule) {
    self.collection.add_rule(&mut self.code, rule);
  }

  pub fn delete_rules_of_user(&mut self, user_id: &Uuid) {
    self.collection.delete_rules_of_user(&mut self.code, user_id);
  }

  pub fn delete_rules_of_policy(&mut self, policy_id: &Uuid) {
    self.collection.delete_rules_of_policy(&mut self.code, policy_id);
  }

  pub fn delete_rule(&mut self, rule_id: &Uuid) {
    self.collection.delete_rule(&mut self.code, rule_id);
  }

  pub fn update_rule(&mut self, rule_id: &Uuid, rule_update_draft: &RuleUpdateDraft) {
    self.collection.update_rule(&mut self.code, rule_id, rule_update_draft);
  }

  pub fn commit(&self) -> Result<(), GenericError> {
    self.collection.database.execute(&self.code)
  }
}

impl<'a> RuleCollection<'a> {
  pub fn write_definition_into(&self, into: &mut String) {
    into.push_str("CREATE TABLE IF NOT EXISTS "); 
    into.push_str(&self.name); 
    into.push_str(" (");
    into.push_str(&self.fields.id);
    into.push_str(" TEXT PRIMARY KEY, ");
    into.push_str(&self.fields.activator_enum_type);
    into.push_str(" INTEGER NOT NULL, ");
    into.push_str(&self.fields.activator_enum_data_1);
    into.push_str(", ");
    into.push_str(&self.fields.activator_enum_data_2);
    into.push_str(") WITHOUT ROWID;");
  }

  pub fn add_rule(&self, code: &mut String, rule: &Rule) {
    code.push_str("INSERT INTO ");
    code.push_str(&self.name);
    code.push_str(" ");
    
    let mut context = SerializeCompoundValueContext::new();
    serialize_rule(&mut context, rule, &self.fields);

    code.push_str("(");
    code.push_str(&context.column_names);
    code.push_str(") VALUES (");
    code.push_str(&context.column_values);
    code.push_str(");");
  }

  pub fn delete_rules_of_user(&self, code: &mut String, user_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.user_id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, code);
    code.push_str(";");
  }

  pub fn delete_rules_of_policy(&self, code: &mut String, policy_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");

    code.push_str(&self.fields.policy_id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);

    code.push_str(";");
  }

  pub fn delete_rule(&self, code: &mut String, rule_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(rule_id, code);
    code.push_str(";");
  }

  pub fn update_rule(&self, code: &mut String, rule_id: &Uuid, rule_update_draft: &RuleUpdateDraft) {
    let Some(updates) = rule_update_draft.draft.finish() else {
      return;
    };

    code.push_str("UPDATE ");
    code.push_str(&self.name);
    code.push_str(" ");
    code.push_str(&updates);
    code.push_str(" WHERE ");

    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(rule_id, code);

    code.push_str(";");
  }

  pub fn create_rule_update_draft(&self) -> RuleUpdateDraft {
    RuleUpdateDraft::new(self)
  }

  pub fn create_collection_update_draft(&self) -> RuleCollectionUpdateDraft {
    RuleCollectionUpdateDraft::new(self)
  }
}