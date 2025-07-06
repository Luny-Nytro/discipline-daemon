use super::*;

pub struct CollectionItemUpdateDraft {
  code: String,
}

impl CollectionItemUpdateDraft {
  pub fn new() -> Self {
    Self {
      code: String::new(),
    }
  }

  fn did_write_an_update(&self) -> bool {
    self.code.len() > 0
  }

  pub fn write_update(
    &mut self, 
    field: &String, 
    new_value: &impl SerializableScalarValue,
  ) {
    if self.did_write_an_update() {
      self.code.push_str(", ");
    } else {
      self.code.push_str("SET ");
    }

    self.code.push_str(field);
    self.code.push_str(" = ");
    serialize_scalar_value_into(new_value, &mut self.code);
  }

  pub fn write_null(
    &mut self, 
    field: &String, 
  ) {
    if self.did_write_an_update() {
      self.code.push_str(", ");
    } else {
      self.code.push_str("SET ");
    }

    self.code.push_str(field);
    self.code.push_str(" = ");
    self.code.push_str("NULL");
  }

  pub(super) fn finish(&self) -> Option<&String> {
    Some(&self.code)
  }
}
