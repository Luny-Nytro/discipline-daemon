use super::{
  CompoundTypeDefiner, CompoundValueSerializer, 
  CompoundValueDeserializer, CompoundValueDeserializerContext, 
  CompoundValueSerializerContext, CountdownTimerSpecification,
  PolicyEnabler, GenericError, IsCompoundType,
};

pub struct PolicyEnablerSpecification {
  timer: CountdownTimerSpecification,
}

impl IsCompoundType for PolicyEnablerSpecification {
  fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      timer: definer.compound_field("Timer")?,
    })
  }

  fn display_name(&self) -> &str {
    "PolicyEnabler"
  }
}

impl PolicyEnablerSpecification {
  pub fn timer(&self) -> &CountdownTimerSpecification {
    &self.timer
  }
}

impl CompoundValueSerializer for PolicyEnablerSpecification {
  type CompoundValue = PolicyEnabler;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_compound(&self.timer, &value.timer)
  }
}

impl CompoundValueDeserializer for PolicyEnablerSpecification {
  type CompoundValue = PolicyEnabler;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
    Ok(PolicyEnabler {
      timer: self.timer.deserialize(context)?,
    })
  }
}