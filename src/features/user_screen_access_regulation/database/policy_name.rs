use super::{
  PolicyName, SerializableScalarValue, DeserializableScalarValue,
  SerializeScalarValueContext, ColumnValue
};

impl SerializableScalarValue for PolicyName {
  fn serialize_into(&self, context: SerializeScalarValueContext) {
    context.as_string(self.as_ref());
  }
}

impl DeserializableScalarValue for PolicyName {
  fn deserialize(value: ColumnValue) -> Result<Self, crate::GenericError> {
    value
      .as_string()
      .and_then(PolicyName::new)
      .map_err(|error| error.change_context("deserialize policy name"))
  }
}