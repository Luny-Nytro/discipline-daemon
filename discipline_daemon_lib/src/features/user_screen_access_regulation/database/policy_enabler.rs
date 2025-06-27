use super::{
  CompoundTypeDefiner, CompoundTypeSerializer, 
  CompoundValueDeserializer, CompoundValueDeserializerContext, 
  CompoundTypeSerializerContext, CountdownTimerSpecification,
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

impl CompoundTypeSerializer for PolicyEnablerSpecification {
  type CompoundType = PolicyEnabler;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_compound(&self.timer, &value.timer)
  }
}

impl CompoundValueDeserializer for PolicyEnablerSpecification {
  type Output = PolicyEnabler;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(PolicyEnabler {
      timer: self.timer.deserialize(context)?,
    })
  }
}