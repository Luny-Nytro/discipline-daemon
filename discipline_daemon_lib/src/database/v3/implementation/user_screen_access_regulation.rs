use super::*;
use super::chronic::*;
use crate::{user_screen_access_regulation::*, TimeRange, WeekdayRange};
use crate::chronic_types::*;
use crate::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleActivatorVariant {
  AllTheTime,
  OnWeekday,
  InTimeRange,
  InWeekdayRange,
}

impl SerializableScalarValue for RuleActivatorVariant {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    match self {
      RuleActivatorVariant::AllTheTime     => 0.serialize(context),
      RuleActivatorVariant::OnWeekday      => 1.serialize(context),
      RuleActivatorVariant::InTimeRange    => 2.serialize(context),
      RuleActivatorVariant::InWeekdayRange => 3.serialize(context),
    }
  }
}

impl DeserializableScalarValue for RuleActivatorVariant {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    let number = value.as_u8()?;

    match number {
      0 => Ok(RuleActivatorVariant::AllTheTime), 
      1 => Ok(RuleActivatorVariant::OnWeekday), 
      2 => Ok(RuleActivatorVariant::InTimeRange), 
      3 => Ok(RuleActivatorVariant::InWeekdayRange), 
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

pub struct RuleActivatorIntegration;

impl RuleActivatorIntegration {
  pub fn write_full_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &RuleActivator,
    variant_field: &String,
    a_field: &String,
    b_field: &String,
  ) {
    match new_value {
      RuleActivator::AllTheTime => {
        updates.write_update(variant_field, &RuleActivatorVariant::AllTheTime);
        updates.write_null(a_field);
        updates.write_null(b_field);
      }
      RuleActivator::OnWeekday(weekday) => {
        updates.write_update(variant_field, &RuleActivatorVariant::OnWeekday);
        updates.write_update(a_field, weekday);
        updates.write_null(b_field);
      }
      RuleActivator::InTimeRange(range) => {
        updates.write_update(variant_field, &RuleActivatorVariant::InTimeRange);
        TimeRangeIntegration::write_full_update(updates, range, a_field, b_field);
      }
      RuleActivator::InWeekdayRange(range) => {
        updates.write_update(variant_field, &RuleActivatorVariant::InWeekdayRange);
        WeekdayRangeIntegration::write_full_update(updates, range, a_field, b_field);
      }
    }
  }

  pub fn write_weekday_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &Weekday,
    variant_field: &String,
    a_field: &String,
    b_field: &String,
  ) {
    updates.write_update(variant_field, &RuleActivatorVariant::OnWeekday);
    updates.write_update(a_field, new_value);
    updates.write_null(b_field);
  }

  pub fn write_time_range_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &TimeRange,
    variant_field: &String,
    a_field: &String,
    b_field: &String,
  ) {
    updates.write_update(variant_field, &RuleActivatorVariant::InTimeRange);
    TimeRangeIntegration::write_full_update(updates, new_value, a_field, b_field);
  }

  pub fn write_weekday_range_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &WeekdayRange,
    variant_field: &String,
    a_field: &String,
    b_field: &String,
  ) {
    updates.write_update(variant_field, &RuleActivatorVariant::InWeekdayRange);
    WeekdayRangeIntegration::write_full_update(updates, new_value, a_field, b_field);
  }
}

fn serialize_rule_activator(
  context: &mut SerializeCompoundValueContext,
  activator: &RuleActivator,
  variant_field: &String,
  a_field: &String,
  b_field: &String,
) {
  match activator {
    RuleActivator::AllTheTime => {
      context.write_serializable_scalar_value(variant_field, &RuleActivatorVariant::AllTheTime);
    }
    RuleActivator::OnWeekday(weekday) => {
      context.write_serializable_scalar_value(variant_field, &RuleActivatorVariant::OnWeekday);
      context.write_serializable_scalar_value(a_field, weekday);
    }
    RuleActivator::InTimeRange(range) => {
      context.write_serializable_scalar_value(variant_field, &RuleActivatorVariant::InTimeRange);
      serialize_time_range(context, range, a_field, b_field);
    }
    RuleActivator::InWeekdayRange(range) => {
      context.write_serializable_scalar_value(variant_field, &RuleActivatorVariant::InWeekdayRange);
      serialize_weekday_range(context, range, a_field, b_field);
    }
  }
}

fn deserialize_rule_activator(
  context: &mut DeserializeCompoundValueContext,
  variant_field: &String,
  a_field: &String,
  b_field: &String,
) -> 
  Result<RuleActivator, GenericError>
{
  let variant = context.deserializable_scalar(variant_field)?;

  Ok(match variant {
    RuleActivatorVariant::AllTheTime => {
      RuleActivator::AllTheTime
    }
    RuleActivatorVariant::InTimeRange => {
      RuleActivator::InTimeRange(deserialize_time_range(
        context,
        a_field, 
        b_field, 
      )?)
    }
    RuleActivatorVariant::InWeekdayRange => {
      RuleActivator::InWeekdayRange(deserialize_weekday_range(
        context,
        a_field, 
        b_field, 
      )?)
    }
    RuleActivatorVariant::OnWeekday => {
      RuleActivator::OnWeekday(context.deserializable_scalar(
        a_field,
      )?)
    }
  })
}

pub struct RuleFields {
  id: String,
  user_id: String,
  policy_id: String,
  activator_variant: String,
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

fn deserialize_rule(
  context: &mut DeserializeCompoundValueContext,
  fields: &RuleFields,
) 
  -> Result<Rule, GenericError> 
{
  let id = context.deserializable_scalar(
    &fields.id
  )?;

  let activator = deserialize_rule_activator(
    context, 
    &fields.activator_variant, 
    &fields.activator_enum_data_1, 
    &fields.activator_enum_data_2, 
  )?;

  Ok(Rule {
    id,
    activator,
  })
}

pub struct RuleCollection {
  name: String,
  rule_fields: RuleFields,
}

pub struct RuleUpdateDraft<'a> {
  draft: CollectionItemUpdateDraft,
  collection: &'a RuleCollection,
}

impl<'a> RuleUpdateDraft<'a> {
  pub fn new(collection: &'a RuleCollection) -> Self {
    Self {
      draft: CollectionItemUpdateDraft::new(),
      collection,
    }
  }

  pub fn update_activator(&mut self, new_value: &RuleActivator) {
    RuleActivatorIntegration::write_full_update(
      &mut self.draft, 
      new_value, 
      &self.collection.rule_fields.activator_variant, 
      &self.collection.rule_fields.activator_enum_data_1, 
      &self.collection.rule_fields.activator_enum_data_2
    );
  }

  pub fn update_activator_weekday(&mut self, new_value: &Weekday) {
    RuleActivatorIntegration::write_weekday_update(
      &mut self.draft, 
      new_value, 
      &self.collection.rule_fields.activator_variant, 
      &self.collection.rule_fields.activator_enum_data_1, 
      &self.collection.rule_fields.activator_enum_data_2
    );
  }
  
  pub fn update_activator_time_range(&mut self, new_value: &TimeRange) {
    RuleActivatorIntegration::write_time_range_update(
      &mut self.draft, 
      new_value, 
      &self.collection.rule_fields.activator_variant, 
      &self.collection.rule_fields.activator_enum_data_1, 
      &self.collection.rule_fields.activator_enum_data_2
    );
  }

  pub fn update_activator_weekday_range(&mut self, new_value: &WeekdayRange) {
    RuleActivatorIntegration::write_weekday_range_update(
      &mut self.draft, 
      new_value, 
      &self.collection.rule_fields.activator_variant, 
      &self.collection.rule_fields.activator_enum_data_1, 
      &self.collection.rule_fields.activator_enum_data_2,
    );
  }
}


impl RuleCollection {
  pub fn write_definition_into(&self, into: &mut String) {
    into.push_str("CREATE TABLE IF NOT EXISTS "); 
    into.push_str(&self.name); 
    into.push_str(" (");
    into.push_str(&self.rule_fields.id);
    into.push_str(" TEXT PRIMARY KEY, ");
    into.push_str(&self.rule_fields.activator_variant);
    into.push_str(" INTEGER NOT NULL, ");
    into.push_str(&self.rule_fields.activator_enum_data_1);
    into.push_str(", ");
    into.push_str(&self.rule_fields.activator_enum_data_2);
    into.push_str(") WITHOUT ROWID;");
  }

    // user_id: &Uuid,
    // policy_id: &Uuid,
    // rule_id: &Uuid,
  pub fn create_update_draft(&self) -> RuleUpdateDraft {
    RuleUpdateDraft::new(self)
  }
}

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

fn serialize_policy_enabler(
  context: &mut SerializeCompoundValueContext,
  value: &PolicyEnabler,
  timer_duration_field: &String,
  timer_remaining_duration_field: &String,
  timer_previous_synchronization_time_field: &String,
) {
  serialize_countdown_timer(
    context, 
    value.unpack_ref(), 
    timer_duration_field, 
    timer_remaining_duration_field, 
    timer_previous_synchronization_time_field,
  );
}

fn deserialize_policy_enabler(
  context: &mut DeserializeCompoundValueContext,
  timer_duration_field: &String,
  timer_remaining_duration_field: &String,
  timer_previous_synchronization_time_field: &String,
)
  -> Result<PolicyEnabler, GenericError>
{
  Ok(PolicyEnabler::pack(
    deserialize_countdown_timer(
      context, 
      timer_duration_field, 
      timer_remaining_duration_field, 
      timer_previous_synchronization_time_field,
    )?
  ))
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

fn serialize_policy(
  context: &mut SerializeCompoundValueContext,
  value: &Policy,
  fields: &PolicyFields,
) {
  context.write_serializable_scalar_value(&fields.id, value.id());
  context.write_serializable_scalar_value(&fields.name, value.name());
  serialize_policy_enabler(
    context, 
    value.enabler(), 
    &fields.enabler_countdown_timer_duration, 
    &fields.enabler_countdown_timer_remaining_duration, 
    &fields.enabler_countdown_timer_previous_synchronization_time,
  );
}

fn deserialize_policy(
  context: &mut DeserializeCompoundValueContext,
  fields: &PolicyFields,
)
  -> Result<Policy, GenericError>
{
  let id = context.deserializable_scalar(&fields.id)?;
  let name = context.deserializable_scalar(&fields.name)?;
  let enabler = deserialize_policy_enabler(
    context, 
    &fields.enabler_countdown_timer_duration, 
    &fields.enabler_countdown_timer_remaining_duration, 
    &fields.enabler_countdown_timer_previous_synchronization_time,
  )?;

  Ok(Policy::pack(
    id, 
    name, 
    Vec::new(), 
    enabler,
  ))
}

pub struct PolicyUpdates<'a> {
  fields: &'a PolicyFields,
  updates: CollectionItemUpdateDraft,
}

impl<'a> PolicyUpdates<'a> {
  pub fn new(fields: &'a PolicyFields) -> Self {
    Self {
      fields,
      updates: CollectionItemUpdateDraft::new(),
    }
  }

  pub fn update_name(&mut self, new_value: &PolicyName) {
    self.updates.write_update(&self.fields.name, new_value);
  }

  pub fn update_enabler_timer(&mut self, new_value: &CountdownTimer) {
    CountdownTimerIntegration::write_full_update(
      &mut self.updates, 
      new_value, 
      &self.fields.enabler_countdown_timer_duration, 
      &self.fields.enabler_countdown_timer_remaining_duration, 
      &self.fields.enabler_countdown_timer_previous_synchronization_time,
    );
  }

  pub fn update_enabler_timer_duration(&mut self, new_value: &Duration) {
    CountdownTimerIntegration::write_duration_update(
      &mut self.updates, 
      new_value, 
      &self.fields.enabler_countdown_timer_duration, 
      &self.fields.enabler_countdown_timer_remaining_duration, 
      &self.fields.enabler_countdown_timer_previous_synchronization_time,
    );
  }

  pub fn update_enabler_timer_remaining_duration(&mut self, new_value: &Duration) {
    CountdownTimerIntegration::write_remaining_duration_update(
      &mut self.updates, 
      new_value, 
      &self.fields.enabler_countdown_timer_duration, 
      &self.fields.enabler_countdown_timer_remaining_duration, 
      &self.fields.enabler_countdown_timer_previous_synchronization_time,
    );
  }

  pub fn update_enabler_timer_previous_synchronization_time(&mut self, new_value: &DateTime) {
    CountdownTimerIntegration::write_previous_synchrinization_time_update(
      &mut self.updates, 
      new_value, 
      &self.fields.enabler_countdown_timer_duration, 
      &self.fields.enabler_countdown_timer_remaining_duration, 
      &self.fields.enabler_countdown_timer_previous_synchronization_time,
    );
  }
}

pub struct PolicyCollection {
  name: String,
  policy_fields: PolicyFields,
}

impl PolicyCollection {
  pub fn write_definition_into(&self, into: &mut String) {
    into.push_str("CREATE TABLE IF NOT EXISTS "); 
    into.push_str(&self.name); 
    into.push_str(" (");
    into.push_str(&self.policy_fields.id);
    into.push_str(" TEXT PRIMARY KEY, ");
    into.push_str(&self.policy_fields.name);
    into.push_str(" TEXT NOT NULL, ");
    into.push_str(&self.policy_fields.enabler_countdown_timer_duration);
    into.push_str(" INTEGER NOT NULL, ");
    into.push_str(&self.policy_fields.enabler_countdown_timer_remaining_duration);
    into.push_str(" INTEGER NOT NULL, ");
    into.push_str(&self.policy_fields.enabler_countdown_timer_previous_synchronization_time);
    into.push_str(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
  }
}

pub struct PolicyActions<'a> {
  collection: &'a PolicyCollection,
  database: &'a Database,
}

impl<'a> PolicyActions<'a> {
  pub fn new(collection: &'a PolicyCollection, database: &'a Database) -> Self {
    Self {
      collection,
      database,
    }
  }

  pub fn update_name(
    &self, 
    user_id: &Uuid,
    policy_id: &Uuid,
    new_value: &PolicyName,
  ) 
    -> Result<(), GenericError>
  {
    let mut code = String::new();
    code.push_str("UPDATE ");
    code.push_str(&self.collection.name);
    code.push_str(" SET ");
    code.push_str(&self.collection.policy_fields.name);
    code.push_str(" = ");
    serialize_scalar_value_into(new_value, &mut code);
    code.push_str(" WHERE ");
    code.push_str(&self.collection.policy_fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, &mut code);
    code.push_str(" AND ");
    code.push_str(&self.collection.policy_fields.user_id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, &mut code);
    code.push_str(";");
    
    self.database.execute(&code)
  }

  pub fn update_enabler_timer_remaining_duration(
    &self, 
    user_id: &Uuid,
    policy_id: &Uuid,
    new_value: &Duration,
  ) 
    -> Result<(), GenericError>
  {
    let mut code = String::new();
    code.push_str("UPDATE ");
    code.push_str(&self.collection.name);
    code.push_str(" SET ");
    code.push_str(&self.collection.policy_fields.enabler_countdown_timer_remaining_duration);
    code.push_str(" = ");
    serialize_scalar_value_into(new_value, &mut code);
    code.push_str(" WHERE ");
    code.push_str(&self.collection.policy_fields.id);
    code.push_str(" = ");
    serialize_scalar_value_into(policy_id, &mut code);
    code.push_str(" AND ");
    code.push_str(&self.collection.policy_fields.user_id);
    code.push_str(" = ");
    serialize_scalar_value_into(user_id, &mut code);
    code.push_str(";");
    
    self.database.execute(&code)
  }
}

pub struct PublicApi {

}

impl PublicApi {
  pub fn create_rule_update_draft(&self) {

  }
}