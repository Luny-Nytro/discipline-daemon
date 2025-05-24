use super::RuleActivatorVariant;
use crate::GenericError;
use crate::database::{Column, ColumnNamesapce, CompoundValueDeserializer, CompoundValueSerializer, DeserializeContext, SerializeContext, UpdateStatementSetClause};
use crate::user_screen_time_regulation::RuleActivator;
use crate::time_range::database as time_range;
use crate::weekday_range::database as weekday_range;

pub struct RuleActivatorSchema {
  variant: Column,
  weekday: Column,
  in_time_range: time_range::Schema,
  in_weekday_range: weekday_range::Schema,
}

impl RuleActivatorSchema {
  pub(super) fn new(column_namespace: ColumnNamesapce) -> Result<Self, GenericError> {
    Ok(Self {
      variant: column_namespace
        .create_column_builder("variant")
        .build()?,

      weekday: column_namespace
        .create_column_builder("weekday")
        .optional()
        .build()?,
        
      in_time_range: time_range::Schema::new(
        column_namespace
          .create_namespace("in_time_range")
          .optional()
      )?,
     
      in_weekday_range: weekday_range::Schema::new(
        column_namespace
          .create_namespace("in_weekday_range")
          .optional()
      )?,
    })
  }

  pub fn columns(&self) -> Vec<&Column> {
    let mut columns = vec![&self.variant, &self.weekday];
    columns.extend_from_slice(&self.in_time_range.columns());
    columns.extend_from_slice(&self.in_weekday_range.columns());
    columns
  }
  
  pub fn in_time_range(&self) -> &time_range::Schema {
    &self.in_time_range
  }

  pub fn in_weekday_range(&self) -> &weekday_range::Schema {
    &self.in_weekday_range
  }
}

impl CompoundValueSerializer for RuleActivatorSchema {
  type Input = RuleActivator;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    match value {
      RuleActivator::AllTheTime => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::AllTheTime);
      }
      RuleActivator::OnWeekday(weekday) => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::OnWeekday);
        context.serializable_scalar(&self.weekday, weekday);
      }
      RuleActivator::InTimeRange(range) => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::InTimeRange);
        context.serializable_compound(&self.in_time_range, range);
      }
      RuleActivator::InWeekdayRange(range) => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::InWeekdayRange);
        context.serializable_compound(&self.in_weekday_range, range);
      }
    }
  }
}

impl CompoundValueDeserializer for RuleActivatorSchema {
  type Output = RuleActivator;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    let variant = context.deserializable_scalar(&self.variant)?;

    Ok(match variant {
      RuleActivatorVariant::AllTheTime => {
        RuleActivator::AllTheTime
      }
      RuleActivatorVariant::InTimeRange => {
        RuleActivator::InTimeRange(
          context.deserialize_compound(&self.in_time_range)?
        )
      }
      RuleActivatorVariant::InWeekdayRange => {
        RuleActivator::InWeekdayRange(
          context.deserialize_compound(&self.in_weekday_range)?
        )
      }
      RuleActivatorVariant::OnWeekday => {
        RuleActivator::OnWeekday(
          context.deserializable_scalar(&self.weekday)?
        )
      }
    })
  }
}