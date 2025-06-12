use rusqlite::Row;
use crate::GenericError;
use super::{ScalarFieldSpecification, ScalarValue, DeserializableScalarValue};

pub trait CompoundValueDeserializer {
  type Output;

  fn deserialize(
    &self, 
    context: &CompoundValueDeserializerContext,
  ) -> Result<Self::Output, GenericError>;
}

pub struct CompoundValueDeserializerContext<'a>(&'a Row<'a>);

impl<'a> CompoundValueDeserializerContext<'a> {
  fn get_column_value(&self, column: &ScalarFieldSpecification) -> Result<ScalarValue, GenericError> {
    self.0.get_ref(column.fully_qualified_identifier.as_str())
      .map_err(|error| {
        GenericError::new("Get column value failed: SQlite wrapper returned error")
          .add_attachment("column name", column.fully_qualified_identifier.clone())
          .add_attachment("sqlite error", error.to_string())
      })
      .map(
        ScalarValue::new
      )
  }

  // TODO: rename to deserializable_scalar_field
  pub fn deserializable_scalar<Value>(
    &self, 
    column: &ScalarFieldSpecification,
  ) 
    -> Result<Value, GenericError>
  where 
    Value: DeserializableScalarValue
  {
    self
      .get_column_value(column)
      .and_then(Value::deserialize)
  }

  pub fn deserialize_compound<Deserializer>(
    &self, 
    deserializer: &Deserializer,
  ) -> 
    Result<Deserializer::Output, GenericError>
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
  Result<Deserializer::Output, GenericError> 
where 
  Deserializer: CompoundValueDeserializer
{
  CompoundValueDeserializerContext(row).deserialize_compound(deserializer)
}
