use super::{
  RuleActivatorVariant, GenericError, Column, ColumnNamespace,
  CompoundValueDeserializer, CompoundValueSerializer, DeserializeContext, 
  SerializeContext, RuleActivator, time_range, weekday_range, WriteColumns, 
  WriteColumnsContext,
};
pub struct RuleActivatorSchema {
  variant: Column,
  weekday: Column,
  in_time_range: time_range::database::Schema,
  in_weekday_range: weekday_range::database::Schema,
}

impl RuleActivatorSchema {
  pub fn new(column_namespace: ColumnNamespace) -> Result<Self, GenericError> {
    Ok(Self {
      variant: column_namespace
        .create_column_builder("variant")
        .build()?,

      weekday: column_namespace
        .create_column_builder("weekday")
        .optional()
        .build()?,
        
      in_time_range: time_range::database::Schema::new(
        column_namespace
          .create_namespace("in_time_range")
          .optional()
      )?,
     
      in_weekday_range: weekday_range::database::Schema::new(
        column_namespace
          .create_namespace("in_weekday_range")
          .optional()
      )?,
    })
  }
  
  pub fn in_time_range(&self) -> &time_range::database::Schema {
    &self.in_time_range
  }

  pub fn in_weekday_range(&self) -> &weekday_range::database::Schema {
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

impl WriteColumns for RuleActivatorSchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write_scalar_type(&self.variant)?;
    context.write_scalar_type(&self.weekday)?;
    context.write_compound_type(&self.in_time_range)?;
    context.write_compound_type(&self.in_weekday_range)?;
    Ok(())
  }
}