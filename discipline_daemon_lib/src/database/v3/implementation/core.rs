use super::*;

impl SerializableScalarValue for i8 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_i8(*self);
  }
}

impl SerializableScalarValue for u8 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_u8(*self);
  }
}

impl SerializableScalarValue for i16 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_i16(*self);
  }
}

impl SerializableScalarValue for u16 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_u16(*self);
  }
}

impl SerializableScalarValue for i32 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_i32(*self);
  }
}

impl SerializableScalarValue for u32 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_u32(*self);
  }
}

impl SerializableScalarValue for i64 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_i64(*self);
  }
}

impl SerializableScalarValue for u64 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_u64(*self);
  }
}

impl SerializableScalarValue for f32 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_f32(*self);
  }
}

impl SerializableScalarValue for f64 {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_f64(*self);
  }
}

impl SerializableScalarValue for isize {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_isize(*self);
  }
}

impl SerializableScalarValue for usize {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_usize(*self);
  }
}

impl SerializableScalarValue for bool {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_boolean(*self);
  }
}

impl SerializableScalarValue for String {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    context.write_string(self);
  }
}

impl<T> SerializableScalarValue for Option<T> 
where 
  T: SerializableScalarValue
{
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    match self {
      None => {
        context.write_null();
      }
      Some(value) => {
        value.serialize(context);
      }
    }
  }
}

impl<'a, T> SerializableScalarValue for &'a T 
where 
  T: SerializableScalarValue
{
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    (*self).serialize(context);
  }
}


impl DeserializableScalarValue for bool {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_boolean()
  }
}

impl DeserializableScalarValue for String {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_string()
  }
}

impl<T> DeserializableScalarValue for Option<T> 
where 
  T: DeserializableScalarValue
{
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_optional_deserializable()
  }
}

impl DeserializableScalarValue for i8 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_i8()
  }
}

impl DeserializableScalarValue for i16 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_i16()
  }
}

impl DeserializableScalarValue for i32 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_i32()
  }
}

impl DeserializableScalarValue for i64 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_i64()
  }
}

impl DeserializableScalarValue for u8 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_u8()
  }
}

impl DeserializableScalarValue for u16 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_u16()
  }
}

impl DeserializableScalarValue for u32 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_u32()
  }
}

impl DeserializableScalarValue for u64 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_u64()
  }
}

impl DeserializableScalarValue for f32 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_f32()
  }
}

impl DeserializableScalarValue for f64 {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_f64()
  }
}

impl DeserializableScalarValue for usize {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    value.as_usize()
  }
}