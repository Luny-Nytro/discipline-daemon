use super::*;

pub trait CompoundValueSerializer {
  type CompoundValue;

  fn serialize(
    &self, 
    value: &Self::CompoundValue,
    context: &mut SerializeCompoundValueContext, 
  );
}

pub struct SerializeCompoundValueContext {
  pub column_names: String,
  pub column_values: String,
}

impl SerializeCompoundValueContext {
  pub fn new() -> Self {
    Self {
      column_names: String::new(),
      column_values: String::new(),
    }
  }

  fn did_write_some_columns(&mut self) -> bool {
    self.column_names.len() > 0
  }

  pub fn write_separating_commas(&mut self) {
    if self.did_write_some_columns() {
      self.column_names.push_str(", ");
      self.column_values.push_str(", ");
    }
  }

  pub fn write_null(&mut self, field: &String) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str("NULL");
  }
  
  pub fn write_boolean(&mut self, field: &String, boolean: bool) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(if boolean { "TRUE" } else { "FALSE" });
  }

  pub fn write_i8(&mut self, field: &String, number: i8) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_u8(&mut self, field: &String, number: u8) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_i16(&mut self, field: &String, number: i16) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_u16(&mut self, field: &String, number: u16) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_i32(&mut self, field: &String, number: i32) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_u32(&mut self, field: &String, number: u32) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_i64(&mut self, field: &String, number: i64) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_u64(&mut self, field: &String, number: u64) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_f32(&mut self, field: &String, number: f32) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_f64(&mut self, field: &String, number: f64) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_isize(&mut self, field: &String, number: isize) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_usize(&mut self, field: &String, number: usize) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    self.column_values.push_str(&number.to_string());
  }

  pub fn write_string(&mut self, field: &String, string: &String) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    escape_string_into(string, &mut self.column_values);
  }

  pub fn write_scalar<Value: SerializableScalarValue>(
    &mut self, 
    field: &String, 
    value: &Value,
  ) {
    self.write_separating_commas();
    self.column_names.push_str(field);
    serialize_scalar_value_into(value, &mut self.column_values);
  }

  pub fn write_serializable_compound_value<Value>(
    &mut self, 
    serializer: &impl CompoundValueSerializer<CompoundValue = Value>,
    value: &Value,
  ) {
    serializer
      // TODO: Maybe map the error and change the context and add a proper error message
      .serialize(value, self);
  }
}

pub(super) fn serialize_compound_value_into<Value>(
  serializer: &impl CompoundValueSerializer<CompoundValue = Value>,
  value: &Value,
  into: &mut String,
) {
  let mut context = SerializeCompoundValueContext::new();
  serializer.serialize(value, &mut context);
    // .map_err(|error| 
    //   error
    //     .change_context("serializing a compound value into its sqlite representation")
    //     .add_error("the 'write_into' method of the value's CompoundValueSerializer implementation failed")
    // )?;

  // if !context.did_write_some_columns() {
  //   return Err(
  //     GenericError::new("serializing a compound value into its sqlite representation")
  //       .add_error("the 'write_into' of the value's CompoundValueSerializer implementation did not write itself into the provided CompoundValueSerializerContext")
  //   );
  // }

  into.push_str("(");
  into.push_str(&context.column_names);
  into.push_str(") VALUES (");
  into.push_str(&context.column_values);
  into.push_str(")");
}