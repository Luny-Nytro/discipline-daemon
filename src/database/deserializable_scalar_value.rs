use crate::GenericError;
use super::ColumnValue;

pub trait DeserializableScalarValue: Sized {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError>;
}