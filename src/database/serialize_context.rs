use crate::GenericError;
use super::{Column, ColumnNamesapce, CompoundValueSerializer, SerializableScalarValue};

pub struct SerializeContext {
  column_names: String,
  column_values: String,
}

impl SerializeContext {
  pub fn new() -> Self {
    Self {
      column_names: String::new(),
      column_values: String::new(),
    }
  }

  fn written_any_columns(&mut self) -> bool {
    self.column_names.len() > 0
  }

  fn write_separating_comma(&mut self) {
    if self.written_any_columns() {
      self.column_names.push_str(", ");
      self.column_values.push_str(", ");
    }
  }

  fn write_column(&mut self, column_info: &Column, column_value: &str) {
    self.write_separating_comma();
    self.column_names.push_str(&column_info.fully_qualified_name);
    self.column_values.push_str(column_value);
  }

  pub fn push_null(&mut self, column_info: &Column) {
    self.write_column(column_info, "NULL");
  }
  
  pub fn push_boolean(&mut self, column_info: &Column, boolean: bool) {
    self.write_column(column_info, if boolean {
      "TRUE"
    } else {
      "FALSE"
    });
  }

  fn push_i8(&mut self, column_info: &Column, number: i8) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_u8(&mut self, column_info: &Column, number: u8) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_i16(&mut self, column_info: &Column, number: i16) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_u16(&mut self, column_info: &Column, number: u16) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_i32(&mut self, column_info: &Column, number: i32) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_u32(&mut self, column_info: &Column, number: u32) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_i64(&mut self, column_info: &Column, number: i64) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_u64(&mut self, column_info: &Column, number: u64) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_f32(&mut self, column_info: &Column, number: f32) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_f64(&mut self, column_info: &Column, number: f64) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_isize(&mut self, column_info: &Column, number: isize) {
    self.write_column(column_info, &number.to_string())
  }

  fn push_usize(&mut self, column_info: &Column, number: usize) {
    self.write_column(column_info, &number.to_string())
  }
  
  pub fn push_string(&mut self, column_info: &Column, string: &String) {
    self.write_separating_comma();
    self.column_names.push_str(&column_info.fully_qualified_name);
    escape_string_for_sqilte_into(string, &mut self.column_values);
  }

  pub fn serializable_scalar<Value: SerializableScalarValue>(
    &mut self, 
    column_info: &Column, 
    scalar_value: &Value,
  ) {
    self.write_separating_comma();
    self.column_names.push_str(&column_info.fully_qualified_name);
    scalar_value.serialize_into(SerializeScalarValueContext { into: &mut self.column_values });
  }

  pub fn serializable_compound<Value>(
    &mut self, 
    serializer: &impl CompoundValueSerializer<Input = Value>,
    value: &Value,
  ) {
    serializer.serialize_into(value, self);
  }

  pub fn finish(self) -> Option<String> {
    if self.column_names.len() == 0 {
      None
    } else {
      Some(format!("({}) VALUES ({})", self.column_names, self.column_values))
    }
  }
}


fn escape_string_for_sqilte_into(string: &String, into: &mut String) {
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

pub struct SerializeScalarValueContext<'a> {
  into: &'a mut String
}

impl<'a> SerializeScalarValueContext<'a> {
  pub fn new(into: &'a mut String) -> Self {
    Self { into }
  }
  pub fn as_null(self) {
    self.into.push_str("NULL");
  }
  
  pub fn as_boolean(self, boolean: bool) {
    self.into.push_str(if boolean {
      "TRUE"
    } else {
      "FALSE"
    });
  }

  pub fn as_i8(self, number: i8) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_u8(self, number: u8) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_i16(self, number: i16) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_u16(self, number: u16) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_i32(self, number: i32) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_u32(self, number: u32) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_i64(self, number: i64) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_u64(self, number: u64) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_f32(self, number: f32) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_f64(self, number: f64) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_isize(self, number: isize) {
    self.into.push_str(&number.to_string())
  }

  pub fn as_usize(self, number: usize) {
    self.into.push_str(&number.to_string())
  }
  
  pub fn as_string(self, string: &String) {
    escape_string_for_sqilte_into(string, self.into);
  }
}

pub struct UpdateStatement {

}

impl UpdateStatement {
  pub fn new_given_three_where_columns(
    column_1: &Column,
    column_1_value: &impl SerializableScalarValue,
    column_2: &Column,
    column_2_value: &impl SerializableScalarValue,
    column_3: &Column,
    column_3_value: &impl SerializableScalarValue,
  ) -> Self {
    todo!()
  }

  pub fn set(&mut self, column: &Column, value: &impl SerializableScalarValue) {

  }
}

pub struct UpdateStatementSetClause {
  code: String,
}

impl UpdateStatementSetClause {
  pub fn new() -> Self {
    Self {
      code: String::new(),
    }
  }

  pub fn update_column(
    &mut self, 
    column: &Column, 
    new_column_value: &impl SerializableScalarValue,
  ) ->
    Result<(), GenericError>
  {
    if self.code.len() > 0 {
      self.code.push_str(", ");
    }

    self.code.push_str(&column.fully_qualified_name);
    self.code.push_str(" = ");
    new_column_value.serialize_into(SerializeScalarValueContext { 
      into: &mut self.code 
    }); 
    Ok(())
  }

  pub fn finish(self) -> Result<String, GenericError> {
    // TODO: Return an error if the clause is empty.
    Ok(self.code)
  }
}

pub struct UpdateByIdStatement {
  
}

pub fn serialize_string(into: &mut String, string: &String) {
  escape_string_for_sqilte_into(string, into);
}

// pub struct CompoundTypeInfo {
//   id: Column,
//   name: Column,
//   protector: Column,
// }

// impl CompoundTypeInfo {
//   pub fn new() -> Self {
//     Self {
//       id: CompoundTypeFieldInfo::builder("id"),
//       name: CompoundTypeFieldInfo::builder("name"),
//       protector: protector::CompoundTypeInfo::new("protector"),
//     }
//   }
// }

pub struct Database {
  namespace: DatabaseNamespace
}

impl Database {
  pub fn new() -> Self {
    Self {
      namespace: DatabaseNamespace {
        name: "Main".into(),
        path: "Main".into(),
      }
    }
  }

  pub fn namespace(&self) -> &DatabaseNamespace {
    &self.namespace
  }
}

pub struct DatabaseNamespace {
  pub(super) name: String,
  pub(super) path: String,
}

impl DatabaseNamespace {
  pub fn create_namespace(&self, name: &str) -> Result<DatabaseNamespace, GenericError> {
    Ok(DatabaseNamespace { 
      name: name.into(), 
      path: format!("{}_{}", self.path, name),
    })
  }

  pub fn create_table(&self, name: &str) -> Result<Table, GenericError> {
    // TODO: Verify 'name' is a valid table name
    Ok(Table {
      name: name.into(),
      fully_qualified_name: format!("{}_{}", self.path, name),
      column_namespace: ColumnNamesapce::new(),
    })
  }
}

pub struct Table {
  name: String,
  pub(super) fully_qualified_name: String,
  column_namespace: ColumnNamesapce,
}

impl Table {
  pub fn column_namespace(&self) -> &ColumnNamesapce {
    &self.column_namespace
  }
}

pub struct TableInitializer<'a> {
  into: &'a String,
}

impl<'a> TableInitializer<'a> {
  pub fn new(into: &'a String) -> Self {
    Self { into }
  }

  pub fn add_column(&mut self, column: &Column) {

  }

  pub fn finalize(self) {
    
  }
}

pub fn generate_sql_initialize_table(
  into: &mut String,
  table: &Table,
  columns: &[&Column],
) -> 
  Result<(), GenericError> 
{
  // TODO: Return an error if 'columns' is empty.
  // TODO: Return an error if a column is both primary and optional
  // TODO: Return an error if a column is both primary and unique

  into.push_str("CREATE TABLE IF NOT EXISTS ");
  serialize_string(into, &table.fully_qualified_name);
  into.push_str(" (");

  let mut wrote_some_columns = false;
  for column in columns {
    if wrote_some_columns {
      into.push_str(", ");
    }

    into.push_str(&column.fully_qualified_name);
    
    if !column.optional {
      into.push_str(" NOT NULL");
    }

    if column.unique {
      into.push_str(" UNIQUE");
    }

    wrote_some_columns = true;
  }

  let mut initialized_primary_key_constraint = false;
  let mut wrote_some_primary_keys = false;
  for column in columns {
    if !column.primary {
      continue;
    }

    if !initialized_primary_key_constraint {
      into.push_str(", (");
      initialized_primary_key_constraint = true;
    }

    if wrote_some_primary_keys {
      into.push_str(", ");
    }

    into.push_str(&column.fully_qualified_name);
    wrote_some_primary_keys = true;
  }

  if initialized_primary_key_constraint {
    into.push_str(")");
  }

  into.push_str(");");
  Ok(())
}

pub fn generate_create_row_statement<Serializer>(
  into: &mut String,
  table: &Table,
  serializer: &Serializer,
  row: &Serializer::Input,
) ->
  Result<(), GenericError>
where 
  Serializer: CompoundValueSerializer
{
  let mut serialize_context = SerializeContext::new();
  serializer.serialize_into(row, &mut serialize_context);

  let Some(serialized_row) = serialize_context.finish() else {
    return Err(GenericError::new("TODO: Write some descriptive error message here"))
  };

  into.push_str("INSERT INTO ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(" ");
  into.push_str(&serialized_row);
  into.push_str(";");

  Ok(())
}

pub fn generate_sql_where_1_column(
  into: &mut String,
  table: &Table,
  column_1: &Column,
  column_1_value: &impl SerializableScalarValue,
) {
  into.push_str("DELETE FROM ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(" WHERE ");
  into.push_str(&column_1.fully_qualified_name);
  into.push_str(" = ");
  column_1_value.serialize_into(SerializeScalarValueContext { into });
  into.push_str(";");
}

pub fn generate_sql_delete_where_3_columns(
  into: &mut String,
  table: &Table,
  column_1: &Column,
  column_1_value: &impl SerializableScalarValue,
  column_2: &Column,
  column_2_value: &impl SerializableScalarValue,
  column_3: &Column,
  column_3_value: &impl SerializableScalarValue,
) {
  // Append the initial DELETE FROM clause
  into.push_str("DELETE FROM ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(" WHERE ");

  // Append the first column condition
  into.push_str(&column_1.fully_qualified_name);
  into.push_str(" = ");
  column_1_value.serialize_into(SerializeScalarValueContext { into });

  // Append the second column condition
  into.push_str(" AND ");
  into.push_str(&column_2.fully_qualified_name);
  into.push_str(" = ");
  column_2_value.serialize_into(SerializeScalarValueContext { into });

  // Append the third column condition
  into.push_str(" AND ");
  into.push_str(&column_3.fully_qualified_name);
  into.push_str(" = ");
  column_3_value.serialize_into(SerializeScalarValueContext { into });

  // Terminate the SQL statement
  into.push_str(";");
}

pub fn generate_update_column_where_column_statement(
  into: &mut String,
  table: &Table,
  column: &Column,
  new_column_value: &impl SerializableScalarValue,
  where_column: &Column,
  where_column_value: &impl SerializableScalarValue,
) -> 
  Result<(), GenericError>
{
  into.push_str("UPDATE ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(" SET ");
  into.push_str(&column.fully_qualified_name);
  into.push_str(" = ");
  new_column_value.serialize_into(SerializeScalarValueContext { into });
  into.push_str(" WHERE ");
  into.push_str(&where_column.fully_qualified_name);
  into.push_str(" = ");
  where_column_value.serialize_into(SerializeScalarValueContext { into });
  into.push_str(";");
  Ok(())
}

pub fn generate_find_all_rows_statement(
  into: &mut String,
  table: &Table,
) -> 
  Result<(), GenericError>
{
  into.push_str("SELECT * FROM ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(";");
  Ok(())
}

pub fn generate_update_where_column_statement_given_set_clause(
  into: &mut String,
  table: &Table,
  where_column: &Column,
  where_column_value: &impl SerializableScalarValue,
  update_statement_set_clause: UpdateStatementSetClause
) ->
  Result<(), GenericError>
{
  // TODO: use 'GenericError.change_context' to attach more context to the error.
  let updates = update_statement_set_clause.finish()?;
  
  into.push_str("UPDATE ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(" SET ");
  into.push_str(&updates);
  into.push_str(" WHERE ");
  into.push_str(&where_column.fully_qualified_name);
  into.push_str(" = ");
  where_column_value.serialize_into(SerializeScalarValueContext { into });
  into.push_str(";");
  Ok(())
}

pub fn generate_delete_rows_where_column_in_statement<ColumnValue>(
  into: &mut String,
  table: &Table,
  column: &Column,
  column_values: &[ColumnValue]
) ->
  Result<(), GenericError>
where 
  ColumnValue: SerializableScalarValue
{
  // TODO: Return an error if the 'column_values' is empty.

  into.push_str("DELETE FROM ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(" WHERE ");
  into.push_str(&column.fully_qualified_name);
  into.push_str(" IN (");

  let mut wrote_some_column_values = false;
  for column_value in column_values {
    if wrote_some_column_values {
      into.push_str(", ");
    }

    column_value.serialize_into(SerializeScalarValueContext { into });

    wrote_some_column_values = true;
  }

  into.push_str(");");

  Ok(())
}

pub fn generate_ensure_row_create_statement<Serializer>(
  into: &mut String,
  table: &Table,
  serializer: &Serializer,
  row: &Serializer::Input
) ->
  Result<(), GenericError>
where
  Serializer: CompoundValueSerializer
{
  let mut serialize_context = SerializeContext::new();
  serializer.serialize_into(row, &mut serialize_context);

  let Some(serialized_row) = serialize_context.finish() else {
    return Err(GenericError::new("TODO: Write some descriptive error message here"))
  };

  into.push_str("INSERT OR IGNORE INTO ");
  into.push_str(&table.fully_qualified_name);
  into.push_str(" ");
  into.push_str(&serialized_row);
  into.push_str(";");

  Ok(())
}