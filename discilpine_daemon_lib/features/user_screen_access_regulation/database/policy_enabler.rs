use super::{
  CompoundTypeFieldsScope, CompoundValueSerializer, 
  CompoundValueDeserializer, CompoundValueDeserializerContext, 
  CompoundValueSerializerContext, CountdownTimerSpecification,
  PolicyEnabler, GenericError
};

pub struct PolicyEnablerSpecification {
  timer: CountdownTimerSpecification,
}

impl PolicyEnablerSpecification {
  pub fn new(scope: &mut CompoundTypeFieldsScope) -> Result<Self, GenericError> {
    Ok(Self {
      timer: CountdownTimerSpecification::new(scope)?,
    })
  }

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