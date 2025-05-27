mod column_info;

pub use column_info::{
  Column,
  ColumnBuilder,
  ColumnNamespace,
};

mod serializable_scalar_value;
pub use serializable_scalar_value::{SerializableScalarValue, ToSerializableScalarValue};

mod deserializable_scalar_value;
pub use deserializable_scalar_value::DeserializableScalarValue;

mod serialize_context;
pub use serialize_context::{
  SerializeContext, 
  SerializeScalarValueContext,
  Database,
  DatabaseNamespace,
  Table,
  UpdateByIdStatement,
  UpdateStatement,
  generate_sql_initialize_table,
  generate_sql_insert_row,
  generate_sql_delete_where_1_column,
  generate_update_column_where_column_statement,
  generate_find_all_rows_statement,
  generate_update_where_column_statement_given_set_clause,
  generate_delete_rows_where_column_in_statement,
  generate_ensure_row_create_statement,
  generate_sql_delete_where_3_columns,
  UpdateStatementSetClause,
  InitializeTableStatement,
  WriteColumns,
  WriteColumnsContext,
  generate_sql_initialize_table_given_columns_writer,
  generate_sql_delete_where_2_columns,
};

mod column_value;
pub use column_value::ColumnValue;

mod compound_value_serializer;
pub use compound_value_serializer::CompoundValueSerializer;

mod deserialize_context;
pub use deserialize_context::{DeserializeContext, deserialize_sqlite_row_using as deserialize_sqlite_row_using};

pub mod serializable_scalar_value_implementations;

mod compound_value_deserialize;
pub use compound_value_deserialize::CompoundValueDeserializer;

mod scalar_type_adapter;
pub use scalar_type_adapter::ScalarTypeAdapter;

pub mod scalar_type_adapters;
pub use scalar_type_adapters::*;

use crate::GenericError;


pub type DatabaseConnection = rusqlite::Connection;

/// TODO: Rename to SqliteConnection.
pub struct Connection {
  conn: rusqlite::Connection,
  namespace: DatabaseNamespace,
}

impl Connection {
  pub fn new(path: &str) -> Result<Self, GenericError> {
    rusqlite::Connection::open(path)
    .map(|connection| Connection {
      conn: connection,
      namespace: DatabaseNamespace {
        name: "main".into(),
        path: "main".into(),
      }
    })
    .map_err(|error|
      GenericError::new("Failed to open a connection to a sqlite database file")
        .add_attachment("database file path", path)
        .add_attachment("sqilte error", error.to_string())
    )
  }
  
  pub fn namespace(&self) -> &DatabaseNamespace {
    &self.namespace
  }

  pub fn execute(&self, code: &str) -> 
    Result<(), GenericError>
  {
    self.conn.execute_batch(code).map_err(|error|
      GenericError::new("Sqlite wrapper: Failed to execute SQL code")
        .add_attachment("error", error.to_string())
        .add_attachment("SQL code", code)
    )
  }

  pub fn find_all_rows<Deserializer>(
    &self,
    table: &Table,
    row_deserializer: &Deserializer
  ) ->
    Result<Vec<Deserializer::Output>, GenericError>
  where 
    Deserializer: CompoundValueDeserializer
  {
    let mut code = String::new();
    generate_find_all_rows_statement(&mut code, table).map_err(|error|
      error.change_context("Sqlite error: Failed to find all rows: Failed to generate statement")
        .add_attachment("table name", table.fully_qualified_name.clone())
    )?;

    let mut statement = self.conn.prepare(&code).map_err(|error|
      GenericError::new("Sqlite error: Failed to find all rows: Failed to create sqlite query statement")
        .add_attachment("sqlite error", error.to_string())
        .add_attachment("sqlite code", code.clone())
        .add_attachment("table name", table.fully_qualified_name.clone())
    )?;
    
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("Sqlite error: Failed to find all rows: Failed to run sqlite query statement")
        .add_attachment("sqlite error", error.to_string())
        .add_attachment("sqlite code", code.clone())
        .add_attachment("table name", table.fully_qualified_name.clone())
    )?;

    let mut rules = Vec::new();
    loop {
      let row = iterator.next().map_err(|error|
        GenericError::new("Sqlite error: Failed to find all rows: Failed to get next row")
          .add_attachment("sqlite iterator error", error.to_string())
          .add_attachment("sqlite code", code.clone())
          .add_attachment("table name", table.fully_qualified_name.clone())
      )?;

      let Some(row) = row else {
        break;
      };

      let rule = deserialize_sqlite_row_using(row, row_deserializer).map_err(|error|
        error.change_context("Sqlie error: Failed to find all rows: Failed to deserialize a row")
          .add_attachment("sqlite row", format!("{row:?}"))
          .add_attachment("table name", table.fully_qualified_name.clone())
      )?;

      rules.push(rule);
    }

    Ok(rules)
  }

  pub fn find_some_row<Deserializer>(
    &self,
    table: &Table,
    row_deserializer: &Deserializer
  ) ->
    Result<Deserializer::Output, GenericError>
  where 
    Deserializer: CompoundValueDeserializer
  {
    let mut code = String::new();
    code.push_str("SELECT * FROM ");
    code.push_str(&table.fully_qualified_name);
    code.push_str(";");

    let mut statement = self.conn.prepare(&code).map_err(|error|
      GenericError::new("Sqlite error: Failed to find first row: Failed to create sqlite query statement")
        .add_attachment("sqlite error", error.to_string())
        .add_attachment("sqlite code", code.clone())
        .add_attachment("table name", table.fully_qualified_name.clone())
    )?;
    
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("Sqlite error: Failed to find first row: Failed to run sqlite query statement")
        .add_attachment("sqlite error", error.to_string())
        .add_attachment("sqlite code", code.clone())
        .add_attachment("table name", table.fully_qualified_name.clone())
    )?;

    loop {
      let row = iterator.next().map_err(|error|
        GenericError::new("Sqlite error: Failed to find first row: Failed to get next row")
          .add_attachment("sqlite iterator error", error.to_string())
          .add_attachment("sqlite code", code.clone())
          .add_attachment("table name", table.fully_qualified_name.clone())
      )?;

      let Some(row) = row else {
        return Err(
          GenericError::new("Sqlite error: Failed to find some row: No matching row")
            .add_attachment("sqlite code", code.clone())
            .add_attachment("table name", table.fully_qualified_name.clone())
        )
      };

      return deserialize_sqlite_row_using(row, row_deserializer).map_err(|error|
        error.change_context("Sqlie error: Failed to find first row: Failed to deserialize a row")
          .add_attachment("sqlite row", format!("{row:?}"))
          .add_attachment("table name", table.fully_qualified_name.clone())
      );
    }
  }
}