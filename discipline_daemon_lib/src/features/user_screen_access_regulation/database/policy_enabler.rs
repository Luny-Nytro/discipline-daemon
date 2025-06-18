use super::{
  CompoundTypeDefiner, CompoundTypeSerializer, 
  CompoundValueDeserializer, CompoundValueDeserializerContext, 
  CompoundTypeSerializerContext, CountdownTimerSpecification,
  PolicyEnabler, GenericError, CompoundTypeNamespace,
};

pub struct PolicyEnablerSpecification {
  timer: CountdownTimerSpecification,
}

impl PolicyEnablerSpecification {
  pub fn new(
    namespace: &mut CompoundTypeNamespace,
    definer: &mut CompoundTypeDefiner,
  ) -> Result<Self, GenericError> {
    Ok(Self {
      timer: CountdownTimerSpecification::new(namespace, definer)?,
    })
  }

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
      timer: self.timer.deserialize(context).map_err(|error|
        error
          .change_context("deserializing PolicyEnabler")
          .add_error("failed to deserialize the 'Timer' field")
      )?,
    })
  }
}