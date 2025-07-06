use super::*;
use crate::common_types::*;

impl SerializableScalarValue for Uuid {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(&self.to_string());
  }
}

impl DeserializableScalarValue for Uuid {
  fn deserialize(value: ScalarValue) -> Result<Self, crate::GenericError> {
    value
      .as_string()
      .and_then(|string|
        Uuid::try_parse(&string).map_err(|error|
          GenericError::new("creating a Uuid from a string")
            .add_attachment("string", string)
            .add_attachment("error", error.to_string())
            .change_context("deserializing a Uuid")
        )
      )
  }
}
