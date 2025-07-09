use super::*;
use crate::*;

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

impl SerializableScalarValue for OperatingSystemUserId {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_u32(self.as_raw());
  }
}

impl DeserializableScalarValue for OperatingSystemUserId {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_u32()
      .map(OperatingSystemUserId::new)
      .map_err(|error|
        error.change_context("deserializing an OperatingSystemUserId")
      )
  }
}

impl SerializableScalarValue for OperatingSystemUsername {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(self.as_ref());
  }
}

impl DeserializableScalarValue for OperatingSystemUsername {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_string()
      .and_then(OperatingSystemUsername::new_or_generic_error)
      .map_err(|error|
        error.change_context("deserializing an OperatingSystemUsername")
      )
  }
}

impl SerializableScalarValue for OperatingSystemPassword {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(self.as_ref());
  }
}

impl DeserializableScalarValue for OperatingSystemPassword {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value
      .as_string()
      .and_then(OperatingSystemPassword::new_or_generic_error)
      .map_err(|error|
        error.change_context("deserializing an OperatingSystemPassword")
      )
  }
}