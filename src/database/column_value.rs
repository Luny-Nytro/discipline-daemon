use rusqlite::types::ValueRef;
use crate::GenericError;
use super::DeserializableScalarValue;

pub struct ColumnValue<'a> {
  value: ValueRef<'a>
}

impl<'a> ColumnValue<'a> {
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
        Err(GenericError::new(format!(
          "Cast value as boolean failed: Expected value to be ValueRef::Integer(0) or ValueRef::Integer(1), but found {:?}", self.value
        )))
      }
    } 
  }

  pub fn as_string(self) -> Result<String, GenericError> {
    let ValueRef::Text(string) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as string failed: Expected a ValueRef::Text but found {:?}", self.value
      )));
    };

    String::from_utf8(string.into()).map_err(|error|
      GenericError::new(format!(
        "Cast value as string failed: Failed to create string from utf8 byte array. Error: {error}"
      ))
    )
  }

  pub fn as_i8(self) -> Result<i8, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as i8 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as i8: Failed to convert integer {integer} to i8. Error: {error}"
      ))
    )
  }
  
  pub fn as_u8(self) -> Result<u8, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as u8 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as u8: Failed to convert integer {integer} to u8. Error: {error}"
      ))
    )
  }
  
  pub fn as_i16(self) -> Result<i16, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as i16 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as i16: Failed to convert integer {integer} to i16. Error: {error}"
      ))
    )
  }
  
  pub fn as_u16(self) -> Result<u16, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as u16 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as u16: Failed to convert integer {integer} to u16. Error: {error}"
      ))
    )
  }
  
  pub fn as_i32(self) -> Result<i32, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as i32 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as i32: Failed to convert integer {integer} to i32. Error: {error}"
      ))
    )
  }
  
  pub fn as_u32(self) -> Result<u32, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as u32 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };

    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as u32: Failed to convert integer {integer} to u32. Error: {error}"
      ))
    )
  }
  
  pub fn as_i64(self) -> Result<i64, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as i64 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    Ok(integer) // i64 is the native SQLite integer type
  }
  
  pub fn as_u64(self) -> Result<u64, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as u64 failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as u64: Failed to convert integer {integer} to u64. Error: {error}"
      ))
    )
  }
  
  pub fn as_f32(self) -> Result<f32, GenericError> {
    match self.value {
      ValueRef::Integer(integer) => Ok(integer as f32),
      ValueRef::Real(float) => Ok(float as f32),
      _ => Err(GenericError::new(format!(
        "Cast value as f32 failed: Expected a ValueRef::Integer or ValueRef::Real but found {:?}", 
        self.value
      )))
    }
  }
  
  pub fn as_f64(self) -> Result<f64, GenericError> {
    match self.value {
      ValueRef::Integer(integer) => Ok(integer as f64),
      ValueRef::Real(float) => Ok(float),
      _ => Err(GenericError::new(format!(
        "Cast value as f64 failed: Expected a ValueRef::Integer or ValueRef::Real but found {:?}", 
        self.value
      )))
    }
  }
  
  pub fn as_isize(self) -> Result<isize, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as isize failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as isize: Failed to convert integer {integer} to isize. Error: {error}"
      ))
    )
  }
  
  pub fn as_usize(self) -> Result<usize, GenericError> {
    let ValueRef::Integer(integer) = self.value else {
      return Err(GenericError::new(format!(
        "Cast value as usize failed: Expected a ValueRef::Integer but found {:?}", 
        self.value
      )));
    };
    integer.try_into().map_err(|error| 
      GenericError::new(format!(
        "Cast value as usize: Failed to convert integer {integer} to usize. Error: {error}"
      ))
    )
  }

  pub fn as_optional_boolean(self) -> Result<Option<bool>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    Ok(Some(self.as_boolean().map_err(|error| {
      error.change_context("Cast value as optional boolean failed: Value is not null and casting to boolean failed")
    })?))
  }
  
  pub fn as_optional_string(self) -> Result<Option<String>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    Ok(Some(self.as_string().map_err(|error| {
      error.change_context("Cast value as optional string failed: Value is not null and casting to string failed")
    })?))
  }
  
  pub fn as_optional_i8(self) -> Result<Option<i8>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_i8().map_err(|error| {
      error.change_context("Cast value as optional i8 failed: Value is not null and casting to i8 failed")
    })?))
  }
  
  pub fn as_optional_u8(self) -> Result<Option<u8>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }

    Ok(Some(self.as_u8().map_err(|error| {
      error.change_context("Cast value as optional u8 failed: Value is not null and casting to u8 failed")
    })?))
  }
  
  pub fn as_optional_i16(self) -> Result<Option<i16>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_i16().map_err(|error| {
      error.change_context("Cast value as optional i16 failed: Value is not null and casting to i16 failed")
    })?))
  }
  
  pub fn as_optional_u16(self) -> Result<Option<u16>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_u16().map_err(|error| {
      error.change_context("Cast value as optional u16 failed: Value is not null and casting to u16 failed")
    })?))
  }
  
  pub fn as_optional_i32(self) -> Result<Option<i32>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_i32().map_err(|error| {
      error.change_context("Cast value as optional i32 failed: Value is not null and casting to i32 failed")
    })?))
  }
  
  pub fn as_optional_u32(self) -> Result<Option<u32>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_u32().map_err(|error| {
      error.change_context("Cast value as optional u32 failed: Value is not null and casting to u32 failed")
    })?))
  }
  
  pub fn as_optional_i64(self) -> Result<Option<i64>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_i64().map_err(|error| {
      error.change_context("Cast value as optional i64 failed: Value is not null and casting to i64 failed")
    })?))
  }
  
  pub fn as_optional_u64(self) -> Result<Option<u64>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_u64().map_err(|error| {
      error.change_context("Cast value as optional u64 failed: Value is not null and casting to u64 failed")
    })?))
  }
  
  pub fn as_optional_f32(self) -> Result<Option<f32>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_f32().map_err(|error| {
      error.change_context("Cast value as optional f32 failed: Value is not null and casting to f32 failed")
    })?))
  }
  
  pub fn as_optional_f64(self) -> Result<Option<f64>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_f64().map_err(|error| {
      error.change_context("Cast value as optional f64 failed: Value is not null and casting to f64 failed")
    })?))
  }
  
  pub fn as_optional_isize(self) -> Result<Option<isize>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_isize().map_err(|error| {
      error.change_context("Cast value as optional isize failed: Value is not null and casting to isize failed")
    })?))
  }
  
  pub fn as_optional_usize(self) -> Result<Option<usize>, GenericError> {
    if let ValueRef::Null = self.value {
      return Ok(None);
    }
    
    Ok(Some(self.as_usize().map_err(|error| {
      error.change_context("Cast value as optional usize failed: Value is not null and casting to usize failed")
    })?))
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
