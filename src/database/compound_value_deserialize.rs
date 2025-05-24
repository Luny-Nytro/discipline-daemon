use crate::GenericError;
use super::DeserializeContext;

pub trait CompoundValueDeserializer {
  type Output;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError>;
}