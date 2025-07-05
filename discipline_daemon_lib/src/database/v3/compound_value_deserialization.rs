use rusqlite::Row;
use super::*;
use crate::GenericError;

pub trait CompoundValueDeserializer {
  type CompoundValue;

  fn deserialize(
    &self, 
    context: &CompoundValueDeserializerContext,
  ) -> Result<Self::CompoundValue, GenericError>;
}

pub struct CompoundValueDeserializerContext<'a>(&'a Row<'a>);

impl<'a> CompoundValueDeserializerContext<'a> {
  fn retrieve_column_value(&self, field_identifier: &String) -> Result<ScalarValue, GenericError> {
    self.0.get_ref(field_identifier.as_str())
      .map_err(|error| {
        GenericError::new("retrieving the value of a sqlite column")
          .add_error("sqlite wrapper returned error")
          .add_attachment("column identifier", field_identifier)
          .add_attachment("sqlite error", error.to_string())
      })
      .map(
        ScalarValue::new
      )
  }

  // TODO: rename to deserializable_scalar_field
  pub fn deserializable_scalar<Value>(
    &self, 
    field_identifier: &String,
  ) 
    -> Result<Value, GenericError>
  where 
    Value: DeserializableScalarValue
  {
    self
      .retrieve_column_value(field_identifier)
      .and_then(Value::deserialize)
  }

  pub fn deserialize_compound<Deserializer>(
    &self, 
    deserializer: &Deserializer,
  ) -> 
    Result<Deserializer::CompoundValue, GenericError>
  where
    Deserializer: CompoundValueDeserializer
  {
    deserializer.deserialize(self)
  }
}

pub(super) fn deserialize_compound_value<Deserializer>(
  row: &Row, 
  deserializer: &Deserializer,
) -> 
  Result<Deserializer::CompoundValue, GenericError> 
where 
  Deserializer: CompoundValueDeserializer
{
  CompoundValueDeserializerContext(row).deserialize_compound(deserializer)
}
