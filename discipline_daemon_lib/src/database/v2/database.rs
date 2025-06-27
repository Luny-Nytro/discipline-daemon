use std::path::PathBuf;
use super::*;

pub struct Database {
  pub(super) connection: rusqlite::Connection,
}

impl Database {
  pub fn open(database_directory_path: PathBuf) -> Result<Self, GenericError> {
    if database_directory_path.is_relative() {
      return Err(
        GenericError::new("open a database connection")
          .add_error("database directory path must be absolute")
          .add_attachment("database directory path", database_directory_path.to_string_lossy())
      )
    }

    match rusqlite::Connection::open(database_directory_path.join("data.db")) {
      Ok(connection) => {
        Ok(Database {
          connection
        })
      }

      Err(error) => {
        Err(
          GenericError::new("open a database connection")
            .add_error("the sqlite bindings crate, rusqlite, failed to open a sqlite database connection")
            .add_attachment("database directory path", database_directory_path.to_string_lossy())
            .add_attachment("error", error.to_string())
        )
      }
    }
  }

  pub(super) fn execute(&self, code: &str) -> 
    Result<(), GenericError>
  {
    self.connection.execute_batch(code).map_err(|error|
      GenericError::new("execute sql code")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code)
    )
  }

  pub fn create_modifications_draft(&self) -> DatabaseModificationsDraft {
    todo!()
  }
}