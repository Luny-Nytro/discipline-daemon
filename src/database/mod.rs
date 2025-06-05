mod serializing_utiliites;
use serializing_utiliites::escape_string_for_sqilte_into;

mod compound_value_serialization;
pub use compound_value_serialization::*;

mod specifications;
pub use specifications::*;

mod scalar_value_deserialization;
pub use scalar_value_deserialization::*;

mod scalar_value_serialization;
pub use scalar_value_serialization::*;

mod compound_value_deserialization;
pub use compound_value_deserialization::{CompoundValueDeserializerContext, deserialize_compound_value as deserialize_sqlite_row_using};

mod actions;
pub use actions::*;

use crate::GenericError;

/// TODO: Rename to SqliteConnection.
pub struct Connection {
  conn: rusqlite::Connection,
  namespace: Namespace,
}

impl Connection {
  pub fn new(path: &str) -> Result<Self, GenericError> {
    rusqlite::Connection::open(path)
    .map(|connection| Connection {
      conn: connection,
      namespace: Namespace {
        identifier: "main".into(),
        fully_qualified_identifier: "main".into(),
      }
    })
    .map_err(|error|
      GenericError::new("Failed to open a connection to a sqlite database file")
        .add_attachment("database file path", path)
        .add_attachment("sqilte error", error.to_string())
    )
  }
  
  pub fn namespace(&self) -> &Namespace {
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
    table: &CollectionSpecfication,
    row_deserializer: &Deserializer
  ) ->
    Result<Vec<Deserializer::Output>, GenericError>
  where 
    Deserializer: CompoundValueDeserializer
  {
    let mut code = String::new();
    generate_code_find_all_collection_items(&mut code, table).map_err(|error|
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

      let rule = deserialize_compound_value(row, row_deserializer).map_err(|error|
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
    table: &CollectionSpecfication,
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

      return deserialize_compound_value(row, row_deserializer).map_err(|error|
        error.change_context("Sqlie error: Failed to find first row: Failed to deserialize a row")
          .add_attachment("sqlite row", format!("{row:?}"))
          .add_attachment("table name", table.fully_qualified_name.clone())
      );
    }
  }

  pub fn apply_collection_item_modifications(
    &self,
    collection_specification: &CollectionSpecfication,
    collection_item_matcher: &CollectionItemMatcher,
    collection_item_modifications: &CollectionItemModifications,
  ) -> Result<(), GenericError> {
    todo!()
  }
}