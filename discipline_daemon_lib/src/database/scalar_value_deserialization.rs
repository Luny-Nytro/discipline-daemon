use std::num::TryFromIntError;

use crate::GenericError;
use rusqlite::types::ValueRef;

fn serialize_value_ref(value_ref: ValueRef) -> String {
  format!("{value_ref:?}")
}

pub struct ScalarValue<'a> {
  value: ValueRef<'a>
}

impl<'a> ScalarValue<'a> {
  pub fn new(value: ValueRef<'a>) -> Self {
    Self {
      value
    }
  }

  pub fn as_boolean(self) -> Result<bool, GenericError> {
    match self.value {
      ValueRef::Integer(0) => {
        Ok(false)
      }
      ValueRef::Integer(1) => {
        Ok(true)
      }
      _ => {
        Err(
          GenericError::new("casting scalar value as boolean")
            .add_error("scalar value is not of the boolean type")
            .add_attachment("scalar value", serialize_value_ref(self.value))
        )
      }
    } 
  }

  pub fn as_string(self) -> Result<String, GenericError> {
    let ValueRef::Text(string) = self.value else {
      return Err(
        GenericError::new("casting scalar value as string")
          .add_error("scalar value is not of the string type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    String::from_utf8(string.into()).map_err(|error|
      GenericError::new("casting scalar value as boolean")
        .add_error("scalar value is not valid utf8")
        .add_attachment("raw string", format!("{string:?}"))
        .add_attachment("utf8 parse error", error.to_string())
    )
  }

  pub fn as_i8(self) -> Result<i8, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as i8")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as i8")
        .add_error("scalar value is too large and doesn't fit in the i8 type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_u8(self) -> Result<u8, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as u8")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as u8")
        .add_error("scalar value is too large and doesn't fit in the u8 type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_i16(self) -> Result<i16, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as i16")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as i16")
        .add_error("scalar value is too large and doesn't fit in the i16 type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_u16(self) -> Result<u16, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as u16")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as u16")
        .add_error("scalar value is too large and doesn't fit in the u16 type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_i32(self) -> Result<i32, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as i32")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as i32")
        .add_error("scalar value is too large and doesn't fit in the i32 type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_u32(self) -> Result<u32, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as u32")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as u32")
        .add_error("scalar value is too large and doesn't fit in the u32 type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_i64(self) -> Result<i64, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as i64")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    Ok(integer)
  }

  pub fn as_u64(self) -> Result<u64, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as u64")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as u64")
        .add_error("scalar value is too large and doesn't fit in the u64 type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_isize(self) -> Result<isize, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as isize")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as isize")
        .add_error("scalar value is too large and doesn't fit in the isize type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_usize(self) -> Result<usize, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(
        GenericError::new("casting scalar value as usize")
          .add_error("scalar value is not of the integer type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      );
    };

    integer.try_into().map_err(|error: TryFromIntError| 
      GenericError::new("casting scalar value as usize")
        .add_error("scalar value is too large and doesn't fit in the usize type")
        .add_attachment("scalar value", integer.to_string())
        .add_attachment("error", error.to_string())
    )
  }

  pub fn as_f32(self) -> Result<f32, GenericError> {
    match self.value {
      ValueRef::Real(error) => Ok(error as f32),
      _ => Err(
        GenericError::new("casting scalar value as f32")
          .add_error("scalar value is not of the real type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      )
    }
  }

  pub fn as_f64(self) -> Result<f64, GenericError> {
    match self.value {
      ValueRef::Real(real) => Ok(real),
      _ => Err(
        GenericError::new("casting scalar value as f64")
          .add_error("scalar value is not of the real type")
          .add_attachment("scalar value", serialize_value_ref(self.value))
      )
    }
  }

  pub fn as_optional_boolean(self) -> Result<Option<bool>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_boolean() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional boolean")
          .add_error("scalar value is neither null nor a boolean"))
    }
  }

  pub fn as_optional_string(self) -> Result<Option<String>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_string() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional string")
          .add_error("scalar value is neither null nor a string"))
    }
  }

  pub fn as_optional_i8(self) -> Result<Option<i8>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_i8() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional i8")
          .add_error("scalar value is neither null nor a i8"))
    }
  }

  pub fn as_optional_u8(self) -> Result<Option<u8>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_u8() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional u8")
          .add_error("scalar value is neither null nor a u8"))
    }
  }

  pub fn as_optional_i16(self) -> Result<Option<i16>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_i16() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional i16")
          .add_error("scalar value is neither null nor a i16"))
    }
  }

  pub fn as_optional_u16(self) -> Result<Option<u16>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_u16() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional u16")
          .add_error("scalar value is neither null nor a u16"))
    }
  }

  pub fn as_optional_i32(self) -> Result<Option<i32>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_i32() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional i32")
          .add_error("scalar value is neither null nor a i32"))
    }
  }

  pub fn as_optional_u32(self) -> Result<Option<u32>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_u32() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional u32")
          .add_error("scalar value is neither null nor a u32"))
    }
  }

  pub fn as_optional_i64(self) -> Result<Option<i64>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_i64() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional i64")
          .add_error("scalar value is neither null nor a i64"))
    }
  }

  pub fn as_optional_u64(self) -> Result<Option<u64>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_u64() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional u64")
          .add_error("scalar value is neither null nor a u64"))
    }
  }

  pub fn as_optional_f32(self) -> Result<Option<f32>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_f32() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional f32")
          .add_error("scalar value is neither null nor a f32"))
    }
  }

  pub fn as_optional_f64(self) -> Result<Option<f64>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_f64() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional f64")
          .add_error("scalar value is neither null nor a f64"))
    }
  }

  pub fn as_optional_isize(self) -> Result<Option<isize>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_isize() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional isize")
          .add_error("scalar value is neither null nor a isize"))
    }
  }

  pub fn as_optional_usize(self) -> Result<Option<usize>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    match self.as_usize() {
      Ok(v) => Ok(Some(v)),
      Err(error) => Err(
        error
          .change_context("casting scalar value as optional usize")
          .add_error("scalar value is neither null nor a usize"))
    }
  }

  pub fn as_optional_deserializable<T>(self) -> Result<Option<T>, GenericError> 
  where 
    T: DeserializableScalarValue
  {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    Ok(Some(T::deserialize(self).map_err(|error| 
      error.change_context("Cast value as optional deserializable failed: value is not null and deserializable errored")
    )?))
  }
}

pub trait DeserializableScalarValue: Sized {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError>;
}

impl DeserializableScalarValue for bool {
  fn deserialize(value: ScalarValue) -> Result<Self, crate::GenericError> {
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