use super::{
  CompoundTypeSpecificationCreator, CompoundValueSerializer, 
  CompoundValueDeserializer, CompoundValueDeserializerContext, 
  SerializeContext, CountdownTimerAdapter,
  PolicyEnabler, GenericError, WriteColumns, WriteColumnsContext,
};

pub struct PolicyEnablerSchema {
  timer: CountdownTimerAdapter,
}

impl PolicyEnablerSchema {
  pub fn new(column_namespace: CompoundTypeSpecificationCreator) -> Result<Self, GenericError> {
    Ok(Self {
      timer: CountdownTimerAdapter::new(column_namespace)?,
    })
  }

  pub fn timer(&self) -> &CountdownTimerAdapter {
    &self.timer
  }
}

impl CompoundValueSerializer for PolicyEnablerSchema {
  type Input = PolicyEnabler;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) {
    context.serializable_compound(&self.timer, &value.timer);
  }
}

impl CompoundValueDeserializer for PolicyEnablerSchema {
  type Output = PolicyEnabler;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(PolicyEnabler {
      timer: self.timer.deserialize(context).map_err(|error|
        error
          .change_context("deserialize PolicyEnabler")
          .add_error("failed to deserialize the 'timer' field")
      )?,
    })
  }
}

impl WriteColumns for PolicyEnablerSchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write_compound_type(&self.timer)
  }
}