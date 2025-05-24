use super::SerializeScalarValueContext;

pub trait SerializableScalarValue {
  fn serialize_into(&self, ctx: SerializeScalarValueContext);
}

pub trait ToSerializableScalarValue {
  fn to_serializable_scalar_value(&self) -> impl SerializableScalarValue;
}

impl<T> SerializableScalarValue for T
where 
  T: ToSerializableScalarValue
{
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    self.to_serializable_scalar_value().serialize_into(ctx);
  }
}