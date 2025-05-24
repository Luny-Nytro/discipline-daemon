// TODO: Delete these adapters.

use super::{ScalarTypeAdapter, SerializeScalarValueContext, ColumnValue};
use crate::GenericError;


pub struct BooleanAdapter {}

impl BooleanAdapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for BooleanAdapter {
  type Type = bool;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_boolean(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_boolean()
  }
}

pub struct StringAdapter {}

impl StringAdapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for StringAdapter {
  type Type = String;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_string(value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_string()
  }
}

// pub struct OptionAdapter<Inner: ScalarTypeAdapter>(Inner);

// impl<Inner: ScalarTypeAdapter> OptionAdapter<Inner> {
//   pub fn new(inner: Inner) -> Self {
//     Self(inner)
//   }
// }

// impl<Inner: ScalarTypeAdapter> ScalarTypeAdapter for OptionAdapter<Inner> {
//   type Type = Option<Inner::Type>;

//   fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
//     match value {
//       None => context.as_null(),
//       Some(inner) => Inner::serialize(&self, value, context), inner.serialize(inner, context),
//     }
//   }

//   fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
//     value.as_optional_deserializable()
//   }
// }

pub struct I8Adapter {}

impl I8Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for I8Adapter {
  type Type = i8;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_i8(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_i8()
  }
}

pub struct I16Adapter {}

impl I16Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for I16Adapter {
  type Type = i16;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_i16(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_i16()
  }
}

pub struct I32Adapter {}

impl I32Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for I32Adapter {
  type Type = i32;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_i32(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_i32()
  }
}

pub struct I64Adapter {}

impl I64Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for I64Adapter {
  type Type = i64;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_i64(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_i64()
  }
}

pub struct U16Adapter {}

impl U16Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for U16Adapter {
  type Type = u16;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_u16(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_u16()
  }
}

pub struct U32Adapter {}

impl U32Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for U32Adapter {
  type Type = u32;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_u32(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_u32()
  }
}

pub struct U64Adapter {}

impl U64Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for U64Adapter {
  type Type = u64;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_u64(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_u64()
  }
}

pub struct F32Adapter {}

impl F32Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for F32Adapter {
  type Type = f32;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_f32(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_f32()
  }
}

pub struct F64Adapter {}

impl F64Adapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl ScalarTypeAdapter for F64Adapter {
  type Type = f64;

  fn serialize(&self, value: &Self::Type, context: SerializeScalarValueContext) {
    context.as_f64(*value);
  }

  fn deserialize(&self, value: ColumnValue) -> Result<Self::Type, GenericError> {
    value.as_f64()
  }
}
