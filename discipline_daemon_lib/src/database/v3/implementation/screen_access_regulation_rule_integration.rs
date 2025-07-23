use crate::*;
use crate::screen_access_regulation::*;
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

  pub(super) fn new_with_descriptive_field_names(collection_name: String) -> Self {
    Self {
      name: collection_name,
      fields: RuleFields {
        id: "Id".into(), 
        user_id: "UserId".into(), 
        policy_id: "PolicyId".into(), 
        activator_enum_type: "ActivatorEnumType".into(), 
        activator_enum_data_1: "ActivatorEnumData1".into(), 
        activator_enum_data_2: "ActivatorEnumData2".into(), 
        position: "Position".into(),
      }
    }
  }
}

fn collection(database: &Database) -> &RuleCollection {
  &database.user_screen_access_regulation_rule
}

pub fn write_define(database: &Database, code: &mut DatabaseCode) {
  let collection = collection(database);

  code.write("CREATE TABLE IF NOT EXISTS "); 
  code.write(&collection.name); 
  code.write(" (");
  code.write(&collection.fields.id);
  code.write(" TEXT PRIMARY KEY, ");
  code.write(&collection.fields.activator_enum_type);
  code.write(" INTEGER NOT NULL, ");
  code.write(&collection.fields.activator_enum_data_1);
  code.write(", ");
  code.write(&collection.fields.activator_enum_data_2);
  code.write(", ");
  code.write(&collection.fields.policy_id);
  code.write(" TEXT NOT NULL, ");
  code.write(&collection.fields.user_id);
  code.write(" TEXT NOT NULL, ");
  code.write(&collection.fields.position);
  code.write(" INTEGER NOT NULL) WITHOUT ROWID;");
}

pub fn write_add_rule(
  database: &Database,
  draft: &mut DatabaseCode, 
  rule: &Rule, 
  user_id: &Uuid,
  policy_id: &Uuid,
  position: usize,
) {
  let collection = &database.user_screen_access_regulation_rule;

  draft.write("UPDATE ");
  draft.write(&collection.name);
  draft.write(" SET ");
  draft.write(&collection.fields.position);
  draft.write(" = ");
  draft.write(&collection.fields.position);
  draft.write(" + 1 WHERE ");
  draft.write(&collection.fields.policy_id);
  draft.write(" = ");
  serialize_scalar_value_into(policy_id, draft.as_mut());
  draft.write(" AND ");
  draft.write(&collection.fields.position);
  draft.write(" >= ");
  serialize_scalar_value_into(&position, draft.as_mut());
  draft.write(";");
  
  draft.write("INSERT INTO ");
  draft.write(&collection.name);
  draft.write(" ");
  
  let mut context = SerializeCompoundValueContext::new();
  serialize_rule(
    &mut context, 
    rule, 
    user_id,
    policy_id,
    position, 
    &collection.fields,
  );

  draft.write("(");
  draft.write(&context.column_names);
  draft.write(") VALUES (");
  draft.write(&context.column_values);
  draft.write(");");
}

pub fn add_rule(
  database: &Database,
  rule: &Rule, 
  user_id: &Uuid,
  policy_id: &Uuid,
  position: usize,
) -> Result<(), GenericError> {
  let mut draft = DatabaseCode::new();
  write_add_rule(database, &mut draft, rule, user_id, policy_id, position);
  database.execute(&draft.as_ref())
}

pub(super) fn write_delete_rules_of_user(
  database: &Database,
  draft: &mut DatabaseCode, 
  user_id: &Uuid,
) {
  let collection = &database.user_screen_access_regulation_rule;

  draft.write("DELETE FROM ");
  draft.write(&collection.name);
  draft.write(" WHERE ");
  draft.write(&collection.fields.user_id);
  draft.write(" = ");
  serialize_scalar_value_into(user_id, draft.as_mut());
  draft.write(";");
}

pub(super) fn write_delete_rules_of_policy(
  database: &Database,
  draft: &mut DatabaseCode, 
  policy_id: &Uuid,
) {
  let collection = &database.user_screen_access_regulation_rule;

  draft.write("DELETE FROM ");
  draft.write(&collection.name);
  draft.write(" WHERE ");

  draft.write(&collection.fields.policy_id);
  draft.write(" = ");
  serialize_scalar_value_into(policy_id, draft.as_mut());

  draft.write(";");
}

pub fn write_delete_rule(
  database: &Database,
  draft: &mut DatabaseCode, 
  policy_id: &Uuid,
  rule_id: &Uuid,
  rule_position: usize,
) {
  let collection = &database.user_screen_access_regulation_rule;

  draft.write("DELETE FROM ");
  draft.write(&collection.name);
  draft.write(" WHERE ");
  draft.write(&collection.fields.id);
  draft.write(" = ");
  serialize_scalar_value_into(rule_id, draft.as_mut());
  draft.write(";");

  draft.write("UPDATE ");
  draft.write(&collection.name);
  draft.write(" SET ");
  draft.write(&collection.fields.position);
  draft.write(" = ");
  draft.write(&collection.fields.position);
  draft.write(" - 1 WHERE ");
  draft.write(&collection.fields.policy_id);
  draft.write(" = ");
  serialize_scalar_value_into(policy_id, draft.as_mut());
  draft.write(" AND ");
  draft.write(&collection.fields.position);
  draft.write(" > ");
  serialize_scalar_value_into(&rule_position, draft.as_mut());
  draft.write(";");
}

pub fn delete_rule(
  database: &Database,
  rule_id: &Uuid,
) -> Result<(), GenericError> {
  todo!()
  // let mut draft = DatabaseCode::new();
  // write_delete_rule(database, &mut draft, policy_id, rule_id, rule_position);  
}

pub fn write_retrieve_all_rules(database: &Database, code: &mut DatabaseCode) {
  let collection = collection(database);

  code.write("SELECT * FROM ");
  code.write(&collection.name);
  code.write(";");
}

pub fn retrieve_all_rules(database: &Database) -> Result<Vec<NormalizedRule>, GenericError> {
  let mut code = DatabaseCode::new();
  write_retrieve_all_rules(database, &mut code);

  let mut statement = database.connection.prepare(&code.as_ref()).map_err(|error| 
    GenericError::new("retrieving all user screen access regulation rules")
      .add_error("failed to prepare statement")
      .add_attachment("code", code.as_ref())
      .add_attachment("sqlite error", error.to_string())
  )?;
  let mut iterator = statement.query(()).map_err(|error| 
    GenericError::new("retrieving all user screen access regulation rules")
      .add_error("failed to run query code")
      .add_attachment("code", code.as_ref())
      .add_attachment("sqlite error", error.to_string())
  )?;
  let mut rules = Vec::new();
  loop {
    let item = iterator.next().map_err(|error| 
      GenericError::new("retrieving all user screen access regulation rules")
      .add_error("retreiving the next item of sqlite iterator")
      .add_attachment("code", code.as_ref())
      .add_attachment("sqlite error", error.to_string())
    )?;
    let Some(item) = item else {
      return Ok(rules);
    };
    let mut context = DeserializeCompoundValueContext(item);
    rules.push(deserialize_rule(&mut context, &collection(database).fields)?);
  }
}

pub struct RuleUpdateDraft {
  draft: CollectionItemUpdateDraft,
}

impl RuleUpdateDraft {
  pub fn new() -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new()
    }
  }
}

pub fn write_activator(database: &Database, draft: &mut RuleUpdateDraft, new_value: &RuleActivator) {
  let fields = &database.user_screen_access_regulation_rule.fields;

  match new_value {
    RuleActivator::AllTheTime => {
      draft.draft.write_scalar(&fields.activator_enum_type, &RuleActivatorType::AllTheTime);
    }
    RuleActivator::OnWeekday(weekday) => {
      draft.draft.write_scalar(&fields.activator_enum_type, &RuleActivatorType::OnWeekday);
      draft.draft.write_scalar(&fields.activator_enum_data_1, weekday);
    }
    RuleActivator::InTimeRange(range) => {
      draft.draft.write_scalar(&fields.activator_enum_type, &RuleActivatorType::InTimeRange);
      draft.draft.write_u32(&fields.activator_enum_data_1, range.from_as_timestamp());
      draft.draft.write_u32(&fields.activator_enum_data_2, range.till_as_timestamp());
    }
    RuleActivator::InWeekdayRange(range) => {
      draft.draft.write_scalar(&fields.activator_enum_type, &RuleActivatorType::InWeekdayRange);
      draft.draft.write_u32(&fields.activator_enum_data_1, range.from_as_timestamp());
      draft.draft.write_u32(&fields.activator_enum_data_2, range.till_as_timestamp());
    }
  }
}

pub fn write_activator_weekday(database: &Database, draft: &mut RuleUpdateDraft, new_value: &Weekday) {
  let fields = &database.user_screen_access_regulation_rule.fields;

  draft.draft.write_scalar(&fields.activator_enum_type, &RuleActivatorType::OnWeekday);
  draft.draft.write_scalar(&fields.activator_enum_data_1, new_value);
}

pub fn update_activator_weekday_range(database: &Database, rule_id: &Uuid, new_value: &WeekdayRange) -> Result<(), GenericError> {
  let mut draft = RuleUpdateDraft::new();
  write_activator_weekday_range(database, &mut draft, new_value);
  commit_rule_update_draft(database, &draft, rule_id)
}

pub fn write_activator_time_range(database: &Database, draft: &mut RuleUpdateDraft, new_value: &TimeRange) {
  let fields = &database.user_screen_access_regulation_rule.fields;

  draft.draft.write_scalar(&fields.activator_enum_type, &RuleActivatorType::InTimeRange);
  draft.draft.write_u32(&fields.activator_enum_data_1, new_value.from_as_timestamp());
  draft.draft.write_u32(&fields.activator_enum_data_2, new_value.till_as_timestamp());
}

pub fn update_activator_time_range(database: &Database, rule_id: &Uuid, new_value: &TimeRange) -> Result<(), GenericError> {
  let mut draft = RuleUpdateDraft::new();
  write_activator_time_range(database, &mut draft, new_value);
  commit_rule_update_draft(database, &draft, rule_id)
}

pub fn write_activator_weekday_range(database: &Database, draft: &mut RuleUpdateDraft, new_value: &WeekdayRange) {
  let fields = &database.user_screen_access_regulation_rule.fields;

  draft.draft.write_scalar(&fields.activator_enum_type, &RuleActivatorType::InWeekdayRange);
  draft.draft.write_u32(&fields.activator_enum_data_1, new_value.from_as_timestamp());
  draft.draft.write_u32(&fields.activator_enum_data_2, new_value.till_as_timestamp());
}

pub fn write_update_rule(database: &Database, database_update_draft: &mut DatabaseCode , rule_update_draft: &RuleUpdateDraft, rule_id: &Uuid) {
  let Some(updates) = rule_update_draft.draft.updates() else {
    return;
  };

  let collection = collection(database);

  database_update_draft.write("UPDATE ");
  database_update_draft.write(&collection.name);
  database_update_draft.write(" ");
  database_update_draft.write(&updates);
  database_update_draft.write(" WHERE ");
  database_update_draft.write(&collection.fields.id);
  database_update_draft.write(" = ");
  serialize_scalar_value_into(rule_id, database_update_draft.as_mut());
  database_update_draft.write(";");
}

pub fn commit_rule_update_draft(database: &Database, rule_update_draft: &RuleUpdateDraft, rule_id: &Uuid) -> Result<(), GenericError> {
  let mut database_update_draft = DatabaseCode::new();
  write_update_rule(database, &mut database_update_draft, rule_update_draft, rule_id);
  database.execute(database_update_draft.as_str())
}