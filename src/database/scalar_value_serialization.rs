use super::escape_string_for_sqilte_into;
use crate::GenericError;

pub struct SerializeScalarValueContext<'a> {
  into: &'a mut String,
  did_write_value: bool,
}

impl<'a> SerializeScalarValueContext<'a> {
  pub(super) fn new(into: &'a mut String) -> Self {
    Self { 
      into,  
      did_write_value: false,
    }
  }

  pub fn write_null(&mut self) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a null value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
      );
    }

    self.into.push_str("NULL");
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_boolean(&mut self, boolean: bool) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a boolean value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", boolean.to_string())
      );
    }

    self.into.push_str(if boolean { "TRUE" } else { "FALSE" });
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_i8(&mut self, number: i8) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing an i8 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_u8(&mut self, number: u8) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a u8 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_i16(&mut self, number: i16) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing an i16 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_u16(&mut self, number: u16) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a u16 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_i32(&mut self, number: i32) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing an i32 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_u32(&mut self, number: u32) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a u32 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_i64(&mut self, number: i64) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing an i64 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_u64(&mut self, number: u64) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a u64 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_f32(&mut self, number: f32) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing an f32 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_f64(&mut self, number: f64) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing an f64 value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_isize(&mut self, number: isize) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing an isize value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_usize(&mut self, number: usize) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a usize value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", number.to_string())
      );
    }

    self.into.push_str(&number.to_string());
    self.did_write_value = true;
    Ok(())
  }

  pub fn write_string(&mut self, string: &String) -> Result<(), GenericError> {
    if self.did_write_value {
      return Err(
        GenericError::new("writing a string value to SerializeScalarValueContext")
          .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
          .add_attachment("value", string.clone())
      );
    }

    escape_string_for_sqilte_into(string, self.into);
    self.did_write_value = true;
    Ok(())
  }  
}

pub trait SerializableScalarValue {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError>;
}

impl SerializableScalarValue for bool {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_boolean(*self)
  }
}

impl SerializableScalarValue for String {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_string(self)
  }
}

impl<'a> SerializableScalarValue for &'a String {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_string(*self)
  }
}

impl<T> SerializableScalarValue for Option<T>
where 
  T: SerializableScalarValue
{
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    match self {
      Self::None => {
        context.as_null()
      }
      Some(inner) => {
        inner.serialize_into(context)
      }
    }
  }
}

impl SerializableScalarValue for i8 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_i8(*self)
  }
}

impl SerializableScalarValue for i16 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_i16(*self)
  }
}

impl SerializableScalarValue for i32 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_i32(*self)
  }
}

impl SerializableScalarValue for i64 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_i64(*self)
  }
}

impl SerializableScalarValue for u8 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_u8(*self)
  }
}

impl SerializableScalarValue for u16 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_u16(*self)
  }
}

impl SerializableScalarValue for u32 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_u32(*self)
  }
}

impl SerializableScalarValue for u64 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_u64(*self)
  }
}

impl SerializableScalarValue for f32 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_f32(*self)
  }
}

impl SerializableScalarValue for f64 {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_f64(*self)
  }
}

impl SerializableScalarValue for usize {
  fn serialize_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    context.as_usize(*self)
  }
}

pub(super) fn serialize_scalar_value_into(
  scalar_value: &impl SerializableScalarValue,
  into: &mut String,
) -> Result<(), GenericError> {
  let length_before_serialization = into.len();

  let mut context = SerializeScalarValueContext::new(into);
  if let Err(error) = scalar_value.serialize_into(&mut context) {
    return Err(
      error
        .change_context("serializing a scalar value to its sqlite representation")
        .add_error("the 'serialize_into' method of the value's SerializableScalarValue implementation failed")
    )
  }

  if !context.did_write_value {
    return Err(
      GenericError::new("serializing a scalar value to its sqlite representation")
        .add_error("the 'serialize_into' method of the value's SerializableScalarValue implementation didn't write itself into the provided SerializeScalarValueContext")
    );
  }

  Ok(())
}

