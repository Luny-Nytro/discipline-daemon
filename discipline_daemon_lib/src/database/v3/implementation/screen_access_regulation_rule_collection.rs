use crate::*;
use crate::user_screen_access_regulation::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleActivatorType {
  AllTheTime,
  OnWeekday,
  InTimeRange,
  InWeekdayRange,
}

impl SerializableScalarValue for RuleActivatorType {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    match self {
      RuleActivatorType::AllTheTime => 0.serialize(context),
      RuleActivatorType::OnWeekday => 1.serialize(context),
      RuleActivatorType::InTimeRange => 2.serialize(context),
      RuleActivatorType::InWeekdayRange => 3.serialize(context),
    }
  }
}

impl DeserializableScalarValue for RuleActivatorType {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    let number = value.as_u8()?;

    match number {
      0 => Ok(RuleActivatorType::AllTheTime), 
      1 => Ok(RuleActivatorType::OnWeekday), 
      2 => Ok(RuleActivatorType::InTimeRange), 
      3 => Ok(RuleActivatorType::InWeekdayRange), 
      _ => {
        Err(
          GenericError::new("deserializing RuleActivatorVariant")
            .add_error("unknown variant number")
            .add_attachment("variant", number.to_string())
            .add_attachment("known variant numbers", "0, 1, 2, and 3")
        )
      }
    }  
  }
}

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

fn serialize_rule(
  context: &mut SerializeCompoundValueContext,
  rule: &Rule,
  user_id: &Uuid,
  policy_id: &Uuid,
  position: usize,
  rule_fields: &RuleFields,
) {
  context.write_scalar(&rule_fields.id, rule.id());
  context.write_scalar(&rule_fields.user_id, user_id);
  context.write_scalar(&rule_fields.policy_id, policy_id);
  context.write_usize(&rule_fields.position, position);
  
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
  let position = context.deserializable_scalar(&fields.position)?;
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
    position,
  })
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) id: Uuid,
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
  pub(super) activator: RuleActivator,
  pub(super) position: usize,
}

impl NormalizedRule {
  pub fn denormalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
    }
  }
}

pub struct RuleCollection {
  name: String,
  fields: RuleFields,
}

impl RuleCollection {
  pub fn new(
    collection_name: String,
    rule_id: String,
    rule_user_id: String,
    rule_policy_id: String,
    rule_activator_enum_type: String,
    rule_activator_enum_data_1: String,
    rule_activator_enum_data_2: String,
    rule_position: String,
  ) -> Self {
    Self {
      name: collection_name,
      fields: RuleFields {
        id: rule_id,
        user_id: rule_user_id,
        policy_id: rule_policy_id,
        activator_enum_type: rule_activator_enum_type,
        activator_enum_data_1: rule_activator_enum_data_1,
        activator_enum_data_2: rule_activator_enum_data_2,
        position: rule_position,
      }
    }
  }

  pub fn write_definition_into(&self, code: &mut String) {
    code.push_str("CREATE TABLE IF NOT EXISTS "); 
    code.push_str(&self.name); 
    code.push_str(" (");
    code.push_str(&self.fields.id);
    code.push_str(" TEXT PRIMARY KEY, ");
    code.push_str(&self.fields.activator_enum_type);
    code.push_str(" INTEGER NOT NULL, ");
    code.push_str(&self.fields.activator_enum_data_1);
    code.push_str(", ");
    code.push_str(&self.fields.activator_enum_data_2);
    code.push_str(", ");
    code.push_str(&self.fields.policy_id);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.user_id);
    code.push_str(" TEXT NOT NULL, ");
    code.push_str(&self.fields.position);
    code.push_str(" INTEGER NOT NULL) WITHOUT ROWID;");
  }

  pub fn write_add_rule(
    &self, 
    code: &mut String, 
    rule: &Rule, 
    user_id: &Uuid,
    policy_id: &Uuid,
    position: usize,
  ) {
    code.push_str("UPDATE ");
    code.push_str(&self.name);
    code.push_str(" SET ");
    code.push_str(&self.fields.position);
    code.push_str(" = ");
    code.push_str(&self.fields.position);
    code.push_str(" + 1 WHERE ");
    code.push_str(&self.fields.policy_id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);
    code.push_str(" AND ");
    code.push_str(&self.fields.position);
    code.push_str(" >= ");
    serialize_scalar_value_into(&position, code);
    code.push_str(";");
    
    code.push_str("INSERT INTO ");
    code.push_str(&self.name);
    code.push_str(" ");
    
    let mut context = SerializeCompoundValueContext::new();
    serialize_rule(
      &mut context, 
      rule, 
      user_id,
      policy_id,
      position, 
      &self.fields,
    );

    code.push_str("(");
    code.push_str(&context.column_names);
    code.push_str(") VALUES (");
    code.push_str(&context.column_values);
    code.push_str(");");
  }

  pub fn write_delete_rules_of_user(&self, code: &mut String, user_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.user_id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, code);
    code.push_str(";");
  }

  pub fn write_delete_rules_of_policy(&self, code: &mut String, policy_id: &Uuid) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");

    code.push_str(&self.fields.policy_id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);

    code.push_str(";");
  }

  pub fn write_delete_rule(
    &self, 
    code: &mut String, 
    policy_id: &Uuid,
    rule_id: &Uuid,
    rule_position: usize,
  ) {
    code.push_str("DELETE FROM ");
    code.push_str(&self.name);
    code.push_str(" WHERE ");
    code.push_str(&self.fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(rule_id, code);
    code.push_str(";");

    code.push_str("UPDATE ");
    code.push_str(&self.name);
    code.push_str(" SET ");
    code.push_str(&self.fields.position);
    code.push_str(" = ");
    code.push_str(&self.fields.position);
    code.push_str(" - 1 WHERE ");
    code.push_str(&self.fields.policy_id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, code);
    code.push_str(" AND ");
    code.push_str(&self.fields.position);
    code.push_str(" > ");
    serialize_scalar_value_into(&rule_position, code);
    code.push_str(";");
  }

  pub fn write_update_rule(&self, code: &mut String, rule_id: &Uuid, rule_update_draft: &RuleUpdateDraft) {
    rule_update_draft.maybe_write_update_statement_into(code, rule_id);
  }

  pub fn write_retrieve_all_rules(&self, code: &mut String) {
    code.push_str("SELECT * FROM ");
    code.push_str(&self.name);
    code.push_str(";");
  }

  pub fn retrieve_all_rules(&self, database: &Database) -> Result<Vec<NormalizedRule>, GenericError> {
    let mut code = String::new();
    self.write_retrieve_all_rules(&mut code);

    let mut statement = database.connection.prepare(&code).map_err(|error| 
      GenericError::new("retrieving all user screen access regulation rules")
        .add_error("failed to prepare statement")
        .add_attachment("code", &code)
        .add_attachment("sqlite error", error.to_string())
    )?;
    let mut iterator = statement.query(()).map_err(|error| 
      GenericError::new("retrieving all user screen access regulation rules")
        .add_error("failed to run query code")
        .add_attachment("code", &code)
        .add_attachment("sqlite error", error.to_string())
    )?;
    let mut rules = Vec::new();
    loop {
      let item = iterator.next().map_err(|error| 
        GenericError::new("retrieving all user screen access regulation rules")
        .add_error("retreiving the next item of sqlite iterator")
        .add_attachment("code", &code)
        .add_attachment("sqlite error", error.to_string())
      )?;
      let Some(item) = item else {
        return Ok(rules);
      };
      let mut context = DeserializeCompoundValueContext(item);
      rules.push(deserialize_rule(&mut context, &self.fields)?);
    }
  }

  pub fn create_rule_update_draft<'a>(&self, database: &'a Database) -> RuleUpdateDraft<'a> {
    RuleUpdateDraft::new(database)
  }

  pub fn create_collection_update_draft<'a>(&self, database: &'a Database) -> RuleCollectionUpdateDraft<'a> {
    RuleCollectionUpdateDraft::new(database)
  }
}

pub struct RuleUpdateDraft<'a> {
  draft: CollectionItemUpdateDraft,
  database: &'a Database,
  collection: &'a RuleCollection,
}

impl<'a> RuleUpdateDraft<'a> {
  pub fn new(database: &'a Database) -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new(),
      database,
      collection: &database.user_screen_access_regulation_rule,
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

  pub fn maybe_write_update_statement_into(&self, code: &mut String, rule_id: &Uuid) {
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
    serialize_scalar_value_into(rule_id, code);
    code.push_str(";");
  }

  pub fn commit(&self, rule_id: &Uuid) -> Result<(), GenericError> {
    let mut code = String::new();
    self.maybe_write_update_statement_into(&mut code, rule_id);
    self.database.execute(&code)
  }
}

pub struct RuleCollectionUpdateDraft<'a> {
  code: String,
  database: &'a Database,
  collection: &'a RuleCollection,
}

impl<'a> RuleCollectionUpdateDraft<'a> {
  pub fn new(database: &'a Database) -> Self {
    Self {
      code: String::new(),
      database,
      collection: &database.user_screen_access_regulation_rule,
    }
  }

  pub fn add_rule(&mut self, rule: &Rule, user_id: &Uuid, policy_id: &Uuid, rule_position: usize) {
    self.collection.write_add_rule(&mut self.code, rule, user_id, policy_id, rule_position);
  }

  pub fn delete_rules_of_user(&mut self, user_id: &Uuid) {
    self.collection.write_delete_rules_of_user(&mut self.code, user_id);
  }

  pub fn delete_rules_of_policy(&mut self, policy_id: &Uuid) {
    self.collection.write_delete_rules_of_policy(&mut self.code, policy_id);
  }

  pub fn delete_rule(&mut self, policy_id: &Uuid, rule_id: &Uuid, rule_position: usize) {
    self.collection.write_delete_rule(&mut self.code, policy_id, rule_id, rule_position);
  }

  pub fn update_rule(&mut self, rule_id: &Uuid, rule_update_draft: &RuleUpdateDraft) {
    self.collection.write_update_rule(&mut self.code, rule_id, rule_update_draft);
  }

  pub fn commit(&self) -> Result<(), GenericError> {
    self.database.execute(&self.code)
  }
}