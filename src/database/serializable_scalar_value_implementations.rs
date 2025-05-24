use crate::GenericError;
use super::{ColumnValue, DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext};

impl SerializableScalarValue for bool {
  fn serialize_into(&self, ctx: super::SerializeScalarValueContext) {
    ctx.as_boolean(*self);
  }
}

impl DeserializableScalarValue for bool {
  fn deserialize(value: super::ColumnValue) -> Result<Self, crate::GenericError> {
    value.as_boolean()
  }
}

impl SerializableScalarValue for String {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_string(self);
  }
}

impl DeserializableScalarValue for String {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_string()
  }
}

impl<T> SerializableScalarValue for Option<T>
where 
  T: SerializableScalarValue
{
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    match self {
      Self::None => {
        ctx.as_null();
      }
      Some(inner) => {
        inner.serialize_into(ctx);
      }
    }
  }
}

impl<T> DeserializableScalarValue for Option<T> 
where 
  T: DeserializableScalarValue
{
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_optional_deserializable()
  }
}

impl SerializableScalarValue for i8 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_i8(*self);
  }
}

impl DeserializableScalarValue for i8 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_i8()
  }
}

impl SerializableScalarValue for i16 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_i16(*self);
  }
}

impl DeserializableScalarValue for i16 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_i16()
  }
}

impl SerializableScalarValue for i32 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_i32(*self);
  }
}

impl DeserializableScalarValue for i32 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_i32()
  }
}

impl SerializableScalarValue for i64 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_i64(*self);
  }
}

impl DeserializableScalarValue for i64 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_i64()
  }
}

impl SerializableScalarValue for u8 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_u8(*self);
  }
}

impl DeserializableScalarValue for u8 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_u8()
  }
}

impl SerializableScalarValue for u16 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_u16(*self);
  }
}

impl DeserializableScalarValue for u16 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_u16()
  }
}

impl SerializableScalarValue for u32 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_u32(*self);
  }
}

impl DeserializableScalarValue for u32 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_u32()
  }
}

impl SerializableScalarValue for u64 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_u64(*self);
  }
}

impl DeserializableScalarValue for u64 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_u64()
  }
}

impl SerializableScalarValue for f32 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_f32(*self);
  }
}

impl DeserializableScalarValue for f32 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_f32()
  }
}

impl SerializableScalarValue for f64 {
  fn serialize_into(&self, ctx: SerializeScalarValueContext) {
    ctx.as_f64(*self);
  }
}

impl DeserializableScalarValue for f64 {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    value.as_f64()
  }
}