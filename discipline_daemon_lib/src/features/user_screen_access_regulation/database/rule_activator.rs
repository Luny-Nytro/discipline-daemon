use super::{
  RuleActivatorVariant, GenericError, ScalarFieldSpecification, CompoundTypeDefiner,
  CompoundValueDeserializer, CompoundTypeSerializer, CompoundValueDeserializerContext, 
  RuleActivator, time_range, weekday_range, CompoundTypeSerializerContext,
  CompoundTypeNamespace,
};

pub struct RuleActivatorSpecification {
  variant: ScalarFieldSpecification,
  weekday: ScalarFieldSpecification,
  in_time_range: time_range::database::Specification,
  in_weekday_range: weekday_range::database::Specification,
}

impl RuleActivatorSpecification {
  pub fn new(
    namespace: &mut CompoundTypeNamespace,
    definer: &mut CompoundTypeDefiner,
  ) -> 
    Result<Self, GenericError> 
  {
    let mut in_time_range_definer = definer
      .define_optional_writable_compound_field(namespace, "InTimeRange")?;

    let mut in_weekday_range_definer = definer
      .define_optional_writable_compound_field(namespace, "InWeekdayRange")?;

    Ok(Self {
      variant: definer
        .define_required_writable_scalar_field(namespace, "Variant")?,
        
      weekday: definer
        .define_optional_writable_scalar_field(namespace, "Weekday")?,
        
      in_time_range: time_range::database::Specification::new(
        namespace,
        &mut in_time_range_definer,
      )?,
     
      in_weekday_range: weekday_range::database::Specification::new(
        namespace,
        &mut in_weekday_range_definer,
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

impl CompoundTypeSerializer for RuleActivatorSpecification {
  type CompoundType = RuleActivator;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
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