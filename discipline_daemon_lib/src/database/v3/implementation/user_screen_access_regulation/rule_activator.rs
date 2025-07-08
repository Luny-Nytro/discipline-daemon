use super::*;

pub struct RuleActivatorIntegration;

impl RuleActivatorIntegration {
  pub fn write_full_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &RuleActivator,
    enum_type_field: &String,
    enum_data_1_field: &String,
    enum_data_2_field: &String,
  ) {
    match new_value {
      RuleActivator::AllTheTime => {
        updates.write_scalar(enum_type_field, &RuleActivatorType::AllTheTime);
        updates.write_null(enum_data_1_field);
        updates.write_null(enum_data_2_field);
      }
      RuleActivator::OnWeekday(weekday) => {
        updates.write_scalar(enum_type_field, &RuleActivatorType::OnWeekday);
        updates.write_scalar(enum_data_1_field, weekday);
        updates.write_null(enum_data_2_field);
      }
      RuleActivator::InTimeRange(range) => {
        updates.write_scalar(enum_type_field, &RuleActivatorType::InTimeRange);
        TimeRangeIntegration::write_full_update(updates, range, enum_data_1_field, enum_data_2_field);
      }
      RuleActivator::InWeekdayRange(range) => {
        updates.write_scalar(enum_type_field, &RuleActivatorType::InWeekdayRange);
        WeekdayRangeIntegration::write_full_update(updates, range, enum_data_1_field, enum_data_2_field);
      }
    }
  }

  pub fn write_weekday_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &Weekday,
    enum_type_field: &String,
    enum_data_1_field: &String,
    enum_data_2_field: &String,
  ) {
    updates.write_scalar(enum_type_field, &RuleActivatorType::OnWeekday);
    updates.write_scalar(enum_data_1_field, new_value);
    updates.write_null(enum_data_2_field);
  }

  pub fn write_time_range_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &TimeRange,
    enum_type_field: &String,
    enum_data_1_field: &String,
    enum_data_2_field: &String,
  ) {
    updates.write_scalar(enum_type_field, &RuleActivatorType::InTimeRange);
    TimeRangeIntegration::write_full_update(updates, new_value, enum_data_1_field, enum_data_2_field);
  }

  pub fn write_weekday_range_update(
    updates: &mut CollectionItemUpdateDraft,
    new_value: &WeekdayRange,
    enum_type_field: &String,
    enum_data_1_field: &String,
    enum_data_2_field: &String,
  ) {
    updates.write_scalar(enum_type_field, &RuleActivatorType::InWeekdayRange);
    WeekdayRangeIntegration::write_full_update(updates, new_value, enum_data_1_field, enum_data_2_field);
  }
}

pub fn serialize_rule_activator(
  context: &mut SerializeCompoundValueContext,
  activator: &RuleActivator,
  enum_type_field: &String,
  enum_data_1_field: &String,
  enum_data_2_field: &String,
) {
  match activator {
    RuleActivator::AllTheTime => {
      context.write_scalar(enum_type_field, &RuleActivatorType::AllTheTime);
    }
    RuleActivator::OnWeekday(weekday) => {
      context.write_scalar(enum_type_field, &RuleActivatorType::OnWeekday);
      context.write_scalar(enum_data_1_field, weekday);
    }
    RuleActivator::InTimeRange(range) => {
      context.write_scalar(enum_type_field, &RuleActivatorType::InTimeRange);
      serialize_time_range(context, range, enum_data_1_field, enum_data_2_field);
    }
    RuleActivator::InWeekdayRange(range) => {
      context.write_scalar(enum_type_field, &RuleActivatorType::InWeekdayRange);
      serialize_weekday_range(context, range, enum_data_1_field, enum_data_2_field);
    }
  }
}

pub fn deserialize_rule_activator(
  context: &mut DeserializeCompoundValueContext,
  enum_type_field: &String,
  enum_data_1_field: &String,
  enum_data_2_field: &String,
) -> 
  Result<RuleActivator, GenericError>
{
  let variant = context.deserializable_scalar(enum_type_field)?;

  Ok(match variant {
    RuleActivatorType::AllTheTime => {
      RuleActivator::AllTheTime
    }
    RuleActivatorType::InTimeRange => {
      RuleActivator::InTimeRange(deserialize_time_range(
        context,
        enum_data_1_field, 
        enum_data_2_field, 
      )?)
    }
    RuleActivatorType::InWeekdayRange => {
      RuleActivator::InWeekdayRange(deserialize_weekday_range(
        context,
        enum_data_1_field, 
        enum_data_2_field, 
      )?)
    }
    RuleActivatorType::OnWeekday => {
      RuleActivator::OnWeekday(context.deserializable_scalar(
        enum_data_1_field,
      )?)
    }
  })
}
