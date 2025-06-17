use super::escape_string_for_sqilte_into;

// pub struct SerializeScalarValueContext<'a> {
//   into: &'a mut String,
//   did_write_value: bool,
// }

// impl<'a> SerializeScalarValueContext<'a> {
//   pub(super) fn new(into: &'a mut String) -> Self {
//     Self { 
//       into,  
//       did_write_value: false,
//     }
//   }

//   pub fn write_null(&mut self) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a null value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//       );
//     }

//     self.into.push_str("NULL");
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_boolean(&mut self, boolean: bool) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a boolean value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", boolean.to_string())
//       );
//     }

//     self.into.push_str(if boolean { "TRUE" } else { "FALSE" });
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_i8(&mut self, number: i8) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing an i8 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_u8(&mut self, number: u8) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a u8 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_i16(&mut self, number: i16) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing an i16 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_u16(&mut self, number: u16) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a u16 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_i32(&mut self, number: i32) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing an i32 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_u32(&mut self, number: u32) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a u32 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_i64(&mut self, number: i64) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing an i64 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_u64(&mut self, number: u64) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a u64 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_f32(&mut self, number: f32) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing an f32 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_f64(&mut self, number: f64) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing an f64 value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_isize(&mut self, number: isize) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing an isize value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_usize(&mut self, number: usize) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a usize value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", number.to_string())
//       );
//     }

//     self.into.push_str(&number.to_string());
//     self.did_write_value = true;
//     Ok(())
//   }

//   pub fn write_string(&mut self, string: &String) -> Result<(), GenericError> {
//     if self.did_write_value {
//       return Err(
//         GenericError::new("writing a string value to SerializeScalarValueContext")
//           .add_error("only a single value is allowed in SerializeScalarValueContext, but one has already been written")
//           .add_attachment("value", string.clone())
//       );
//     }

//     escape_string_for_sqilte_into(string, self.into);
//     self.did_write_value = true;
//     Ok(())
//   }  
// }

trait Serializable {
  fn write_into(&self, into: &mut String);
}

impl Serializable for bool {
  fn write_into(&self, into: &mut String) {
    into.push_str(if self { "TRUE" } else { "FALSE" });
  }
}

impl Serializable for i8 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for u8 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for i16 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for u16 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for i32 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for u32 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for i64 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for u64 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for f32 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for f64 {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for isize {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for usize {
  fn write_into(&self, into: &mut String) {
    into.push_str(&self.to_string());
  }
}

impl Serializable for String {
  fn write_into(&self, into: &mut String) {
    escape_string_for_sqilte_into(self, into);
  }
}

impl<'a, T> Serializable for &'a T 
where 
  T: Serializable
{
  fn write_into(&self, into: &mut String) {
    self.write_into(into);
  }
}

impl<T> Serializable for Option<T> 
where 
  T: Serializable
{
  fn write_into(&self, into: &mut String) {
    match self {
      None => {
        into.push_str("NULL");
      }
      Some(inner) => {
        into.write_into(into);
      }
    }
  }
}

pub trait IsScalarValue: Serializable {

}

impl<T> IsScalarValue for T
where 
  T: Serializable
{
  
}

pub trait IntoScalarValue {
  fn into_scalar_value(&self) -> impl IsScalarValue;
}

impl<T> IntoScalarValue for T 
where 
  T: IsScalarValue
{
  fn into_scalar_value(&self) -> impl IsScalarValue {
    self
  }
}

// pub trait FromScalarValue: Sized {
//   type Input: IsScalarValue;

//   fn from_scalar_value(scalar_value: Self::Input) -> Result<Self, GenericError>;
// }


pub(super) fn serialize_scalar_value_into(
  scalar_value: &impl IntoScalarValue,
  into: &mut String,
) {
  scalar_value.into_scalar_value().write_into(into);
  // let length_before_serialization = into.len();

  // let mut context = SerializeScalarValueContext::new(into);
  // if let Err(error) = scalar_value.write_into(&mut context) {
  //   return Err(
  //     error
  //       .change_context("serializing a scalar value to its sqlite representation")
  //       .add_error("the 'serialize_into' method of the value's IntoScalarValue implementation failed")
  //   )
  // }

  // if !context.did_write_value {
  //   return Err(
  //     GenericError::new("serializing a scalar value to its sqlite representation")
  //       .add_error("the 'write_into' method of the value's IntoScalarValue implementation didn't write itself into the provided SerializeScalarValueContext")
  //   );
  // }

  // Ok(())
}

