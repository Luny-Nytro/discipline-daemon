use super::*;

pub trait CompoundTypeSerializer {
  type CompoundType;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
  ) -> Result<(), GenericError>;
}

pub struct CompoundTypeSerializerContext {
  column_names: String,
  column_values: String,
}

impl CompoundTypeSerializerContext {
  fn new() -> Self {
    Self {
      column_names: String::new(),
      column_values: String::new(),
    }
  }

  fn did_write_some_columns(&mut self) -> bool {
    self.column_names.len() > 0
  }

  fn write_separating_comma(&mut self) {
    if self.did_write_some_columns() {
      self.column_names.push_str(", ");
      self.column_values.push_str(", ");
    }
  }

  fn write_column(
    &mut self, 
    field: &Field, 
    value: &str,
  ) {
    self.write_separating_comma();
    self.column_names.push_str(field.path().to_sql_identifier_string());
    self.column_values.push_str(value);
  }

  pub fn write_null(&mut self, field: &Field) -> Result<(), GenericError> {
    self.write_column(field, "NULL");
    Ok(())
  }
  
  pub fn write_boolean(&mut self, field: &Field, boolean: bool) -> Result<(), GenericError> {
    self.write_column(field, if boolean { "TRUE" } else { "FALSE" });
    Ok(())
  }

  // fn push_i8(&mut self, column_info: &Column, number: i8) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_u8(&mut self, column_info: &Column, number: u8) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_i16(&mut self, column_info: &Column, number: i16) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_u16(&mut self, column_info: &Column, number: u16) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_i32(&mut self, column_info: &Column, number: i32) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_u32(&mut self, column_info: &Column, number: u32) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_i64(&mut self, column_info: &Column, number: i64) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_u64(&mut self, column_info: &Column, number: u64) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_f32(&mut self, column_info: &Column, number: f32) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_f64(&mut self, column_info: &Column, number: f64) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_isize(&mut self, column_info: &Column, number: isize) {
  //   self.write_column(column_info, &number.to_string())
  // }

  // fn push_usize(&mut self, column_info: &Column, number: usize) {
  //   self.write_column(column_info, &number.to_string())
  // }
  
  pub fn write_string(&mut self, field: &Field, string: &String) -> Result<(), GenericError> {
    self.write_separating_comma();
    self.column_names.push_str(field.path().to_sql_identifier_string());
    // TODO
    escape_string_for_sqilte_into(string, &mut self.column_values);
    Ok(())
  }

  pub fn serializable_scalar<Value: IntoScalarValue>(
    &mut self, 
    field: &Field, 
    value: &Value,
  ) -> 
    Result<(), GenericError> 
  {
    let mut temp = String::new();
    
    serialize_scalar_value_into(value, &mut temp);
      // .map_err(|error| 
      //   error
      //     .change_context("writing a IntoScalarValue into a CompoundValueSerializerContext")
      //     .add_error("failed to serialize the scalar value")
      // )?;
      
    self.write_separating_comma();
    self.column_names.push_str(field.path().to_sql_identifier_string());
    self.column_values.push_str(&temp);
    Ok(())
  }

  pub fn serializable_compound<Value>(
    &mut self, 
    serializer: &impl CompoundTypeSerializer<CompoundType = Value>,
    value: &Value,
  ) -> Result<(), GenericError> {
    serializer
      // TODO: Maybe map the error and change the context and add a proper error message
      .serialize_into(value, self)
  }
}

pub(super) fn serialize_compound_value_into<Value>(
  serializer: &impl CompoundTypeSerializer<CompoundType = Value>,
  value: &Value,
  into: &mut String,
) -> Result<(), GenericError> {
  let mut context = CompoundTypeSerializerContext::new();
  serializer
    .serialize_into(value, &mut context)
    .map_err(|error| 
      error
        .change_context("serializing a compound value into its sqlite representation")
        .add_error("the 'write_into' method of the value's CompoundValueSerializer implementation failed")
    )?;

  if !context.did_write_some_columns() {
    return Err(
      GenericError::new("serializing a compound value into its sqlite representation")
        .add_error("the 'write_into' of the value's CompoundValueSerializer implementation did not write itself into the provided CompoundValueSerializerContext")
    );
  }

  into.push_str("(");
  into.push_str(&context.column_names);
  into.push_str(") VALUES (");
  into.push_str(&context.column_values);
  into.push_str(")");
  Ok(())
}