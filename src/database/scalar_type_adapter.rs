use crate::GenericError;
use super::ColumnValue;
use super::SerializeScalarValueContext;

// TODO: Delete this.
pub trait ScalarTypeAdapter {
  type Type;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext);
  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError>;
}