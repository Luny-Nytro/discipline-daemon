use rusqlite::Row;
use super::*;
use crate::GenericError;

// pub trait CompoundValueDeserializer {
//   type CompoundValue;

//   fn deserialize(
//     &self, 
//     context: &DeserializeCompoundValueContext,
//   ) -> Result<Self::CompoundValue, GenericError>;
// }

pub struct DeserializeCompoundValueContext<'a>(pub &'a Row<'a>);

impl<'a> DeserializeCompoundValueContext<'a> {
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

  // pub fn read_i64(&self, field_identifier: &String) -> Result<i64, GenericError> {
  //   self.retrieve_column_value(field_identifier)?.as_i64()
  // }
  // pub fn read_string_as_bytes(&self, field_identifier: &String) -> Result<&[u8], GenericError> {
  //   todo!()
  // }
  // pub fn read_string(&self, field_identifier: &String) -> Result<String, GenericError> {
  //   self.retrieve_column_value(field_identifier)?.as_string()
  // }

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

  // pub fn deserialize_compound<Deserializer>(
  //   &self, 
  //   deserializer: &Deserializer,
  // ) -> 
  //   Result<Deserializer::CompoundValue, GenericError>
  // where
  //   Deserializer: CompoundValueDeserializer
  // {
  //   deserializer.deserialize(self)
  // }
}

// pub(super) fn deserialize_compound_value<Deserializer>(
//   row: &Row, 
//   deserializer: &Deserializer,
// ) -> 
//   Result<Deserializer::CompoundValue, GenericError> 
// where 
//   Deserializer: CompoundValueDeserializer
// {
//   DeserializeCompoundValueContext(row).deserialize_compound(deserializer)
// }
