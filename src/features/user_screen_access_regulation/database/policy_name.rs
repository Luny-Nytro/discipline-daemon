use super::{
  PolicyName, SerializableScalarValue, DeserializableScalarValue,
  SerializeScalarValueContext, ScalarValue,
};

impl SerializableScalarValue for PolicyName {
  fn write_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), crate::GenericError> {
    context.write_string(self.as_ref())
  }
}

impl DeserializableScalarValue for PolicyName {
  fn deserialize(value: ScalarValue) -> Result<Self, crate::GenericError> {
    value
      .as_string()
      .and_then(PolicyName::new)
      .map_err(|error| error.change_context("deserializing PolicyName"))
  }
}