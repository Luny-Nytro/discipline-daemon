use rusqlite::Row;
use crate::GenericError;
use super::{Column, ColumnValue, CompoundValueDeserializer, DeserializableScalarValue};

pub fn deserialize_sqlite_row_using<Deserializer>(
  row: &Row, 
  deserializer: &Deserializer,
) -> 
  Result<Deserializer::Output, GenericError> 
where 
  Deserializer: CompoundValueDeserializer
{
  DeserializeContext(row).deserialize_compound(deserializer)
}

pub struct DeserializeContext<'a>(&'a Row<'a>);

impl<'a> DeserializeContext<'a> {
  pub fn get_column_value(&self, column: &Column) -> Result<ColumnValue, GenericError> {
    self.0.get_ref(column.fully_qualified_name.as_str())
      .map_err(|error| {
        GenericError::new("Get column value failed: SQlite wrapper returned error")
          .add_attachment("column name", column.fully_qualified_name.clone())
          .add_attachment("sqlite error", error.to_string())
      })
      .map(
        ColumnValue::new
      )
  }

  pub fn deserializable_scalar<Value>(
    &self, 
    column: &Column,
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
  ) -> Result<Deserializer::Output, GenericError>
  where
    Deserializer: CompoundValueDeserializer
  {
    deserializer.deserialize(self)
  }

  // pub fn deserialize<Adapter, Value>(adapter: &Adapter, row: Row<'a>) -> Result<Value, DatabaseError> 
  // where 
  //   Adapter: CompoundTypeAdapter<Value>
  // {
  //   adapter.deserialize(&Self(row))
  // }
}