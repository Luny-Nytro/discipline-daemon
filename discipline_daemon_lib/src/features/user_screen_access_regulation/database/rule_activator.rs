use super::{
  RuleActivatorVariant, GenericError, Field, CompoundTypeDefiner,
  CompoundValueDeserializer, CompoundValueSerializer, CompoundValueDeserializerContext, 
  RuleActivator, CompoundValueSerializerContext,
  IsCompoundType, TimeRangeSpecification, WeekdayRangeSpecification,
};


pub struct RuleActivatorSpecification {
  variant: Field,
  weekday: Field,
  in_time_range: TimeRangeSpecification,
  in_weekday_range: WeekdayRangeSpecification,
}

impl IsCompoundType for RuleActivatorSpecification {
  fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      variant: definer.writable_required_field("Variant")?,
      weekday: definer.writable_optional_field("Weekday")?,
      in_time_range: definer.optional_compound_field("InTimeRange")?,
      in_weekday_range: definer.optional_compound_field("InWeekdayRange")?,
    })
  }
  
  fn display_name(&self) -> &str {
    "RuleActivator"
  }
}

impl RuleActivatorSpecification {
  pub fn in_time_range(&self) -> &TimeRangeSpecification {
    &self.in_time_range
  }

  pub fn in_weekday_range(&self) -> &WeekdayRangeSpecification {
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
  type CompoundValue = RuleActivator;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
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