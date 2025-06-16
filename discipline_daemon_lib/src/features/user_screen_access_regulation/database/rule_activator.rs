use super::{
  RuleActivatorVariant, GenericError, ScalarFieldSpecification, CompoundTypeDefiner,
  CompoundValueDeserializer, CompoundValueSerializer, CompoundValueDeserializerContext, 
  RuleActivator, time_range, weekday_range, CompoundValueSerializerContext,
};

pub struct RuleActivatorSpecification {
  variant: ScalarFieldSpecification,
  weekday: ScalarFieldSpecification,
  in_time_range: time_range::database::Specification,
  in_weekday_range: weekday_range::database::Specification,
}

impl RuleActivatorSpecification {
  pub fn new(namespace: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      variant: namespace
        .scalar_field_specification("Variant")
        .build()?,

      weekday: namespace
        .scalar_field_specification("Weekday")
        .optional()
        .build()?,
        
      in_time_range: time_range::database::Specification::new(
        &mut namespace.optional_compound_field_specification("InTimeRange")?
      )?,
     
      in_weekday_range: weekday_range::database::Specification::new(
        &mut namespace.optional_compound_field_specification("InWeekdayRange")?
      )?,
    })
  }
  
  pub fn in_time_range(&self) -> &time_range::database::Specification {
    &self.in_time_range
  }

  pub fn in_weekday_range(&self) -> &weekday_range::database::Specification {
    &self.in_weekday_range
  }
}

impl CompoundValueSerializer for RuleActivatorSpecification {
  type CompoundValue = RuleActivator;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) -> 
    Result<(), GenericError>
  {
    match value {
      RuleActivator::AllTheTime => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::AllTheTime)
      }
      RuleActivator::OnWeekday(weekday) => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::OnWeekday)?;
        context.serializable_scalar(&self.weekday, weekday)
      }
      RuleActivator::InTimeRange(range) => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::InTimeRange)?;
        context.serializable_compound(&self.in_time_range, range)
      }
      RuleActivator::InWeekdayRange(range) => {
        context.serializable_scalar(&self.variant, &RuleActivatorVariant::InWeekdayRange)?;
        context.serializable_compound(&self.in_weekday_range, range)
      }
    }
  }
}

impl CompoundValueDeserializer for RuleActivatorSpecification {
  type Output = RuleActivator;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
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