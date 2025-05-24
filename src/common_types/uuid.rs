pub mod database_serde {
  use uuid::Uuid;
  use crate::database::{ColumnValue, DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext};
  use crate::GenericError;

  impl SerializableScalarValue for Uuid {
    fn serialize_into(&self, ctx: SerializeScalarValueContext) {
      ctx.as_string(&self.to_string());
    }
  }

  impl DeserializableScalarValue for Uuid {
    fn deserialize(value: ColumnValue) -> Result<Self, crate::GenericError> {
      let string = value.as_string().map_err(|error|
        error.change_context("Failed to create a Uuid from ColumnValue: Failed to cast ColumnValue as string")
      )?;

      Uuid::try_parse(&string).map_err(|error|
        GenericError::new("Failed to create a uuid from a string ColumnValue: String is not a uuid")
          .add_attachment("string", string)
          .add_attachment("error", error.to_string())
      )
    }
  }
}