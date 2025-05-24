use super::SerializeScalarValueContext;

pub trait SerializableScalarValue {
  fn serialize_into(&self, ctx: SerializeScalarValueContext);
}