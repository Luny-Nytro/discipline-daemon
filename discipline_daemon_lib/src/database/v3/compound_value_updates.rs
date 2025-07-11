use super::*;

macro_rules! write_number {
  ($method:ident: $type:ty) => {
    pub fn $method(&mut self, field: &str, new_value: $type) {
      if !self.is_empty() {
        self.code.push_str(", ");
      }

      self.code.push_str(field);
      self.code.push_str(" = ");
      self.code.push_str(&new_value.to_string());
    }
  };
}


pub struct CollectionItemUpdateDraft {
  code: String,
}

impl CollectionItemUpdateDraft {
  pub fn new() -> Self {
    Self {
      code: String::new(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.code.is_empty()
  }

  write_number!(write_i8: i8);
  write_number!(write_i16: i16);
  write_number!(write_i32: i32);
  write_number!(write_i64: i64);
  write_number!(write_i128: i128);
  write_number!(write_isize: isize);
  write_number!(write_u8: u8);
  write_number!(write_u16: u16);
  write_number!(write_u32: u32);
  write_number!(write_u64: u64);
  write_number!(write_u128: u128);
  write_number!(write_usize: usize);
  write_number!(write_f32: f32);
  write_number!(write_f64: f64);
  
  pub fn write_scalar(
    &mut self, 
    field: &String, 
    new_value: &impl SerializableScalarValue,
  ) {
    if !self.is_empty() {
      self.code.push_str(", ");
    }

    self.code.push_str(field);
    self.code.push_str(" = ");
    serialize_scalar_value_into(new_value, &mut self.code);
  }

  pub fn write_null(
    &mut self, 
    field: &String, 
  ) {
    if self.is_empty() {
      self.code.push_str(", ");
    }

    self.code.push_str(field);
    self.code.push_str(" = ");
    self.code.push_str("NULL");
  }

  pub(super) fn updates(&self) -> Option<&String> {
    Some(&self.code)
  }
}
