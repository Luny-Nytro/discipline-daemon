pub struct SerializeScalarValueContext<'a> {
  into: &'a mut String,
}

impl<'a> SerializeScalarValueContext<'a> {
  pub(super) fn new(into: &'a mut String) -> Self {
    Self { 
      into,  
    }
  }

  pub fn write_null(&mut self) {
    self.into.push_str("NULL");
  }

  pub fn write_boolean(&mut self, boolean: bool) {
    self.into.push_str(if boolean { "TRUE" } else { "FALSE" });
  }

  pub fn write_i8(&mut self, number: i8) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_u8(&mut self, number: u8) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_i16(&mut self, number: i16) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_u16(&mut self, number: u16) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_i32(&mut self, number: i32) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_u32(&mut self, number: u32) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_i64(&mut self, number: i64) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_u64(&mut self, number: u64) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_f32(&mut self, number: f32) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_f64(&mut self, number: f64) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_isize(&mut self, number: isize) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_usize(&mut self, number: usize) {
    self.into.push_str(&number.to_string());
  }

  pub fn write_string(&mut self, string: &String) {
    escape_string_into(string, self.into);
  }  
}

pub fn escape_string_into(string: &String, into: &mut String) {
  into.push('\'');

  for char in string.chars() {
    if char == '\'' {
      into.push_str("''");
    } else {
      into.push(char);
    }
  }

  into.push('\'');
}

pub trait SerializableScalarValue {
  fn serialize(&self, context: &mut SerializeScalarValueContext);
}

pub fn serialize_scalar_value_into(
  scalar_value: &impl SerializableScalarValue,
  into: &mut String,
) {
  scalar_value.serialize(&mut SerializeScalarValueContext { into });
}