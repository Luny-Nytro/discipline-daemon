use std::{collections::HashSet, fmt::Write, path::Path};
use rusqlite::Connection;
use crate::GenericError;

fn generate_code_initialize_database(into: &mut String) {
  writeln!(into, "
    CREATE TABLE IF NOT EXISTS Blocklist (
      domain TEXT PRIMARY KEY
    );
  ").unwrap();
}

pub struct Database {
  connection: Connection
}

impl Database {
  pub fn open(path: impl AsRef<Path>) -> Result<Self, GenericError> {
    let connection = Connection::open(path).map_err(|error|
      GenericError::new("openning database connection")
        .add_error("sqlite bindings provider returned an error")
        .add_attachment("error", error.to_string())
    )?;

    let mut initialize_statement = String::new();
    generate_code_initialize_database(&mut initialize_statement);
    connection.execute_batch(&initialize_statement).map_err(|error|
      GenericError::new("openning database connection")
        .add_error("failed to execute sql code that initializes the database")
        .add_attachment("code", initialize_statement)
        .add_attachment("error", error.to_string())
    )?;

    Ok(Self {
      connection: connection
    })
  }

  pub fn retrieve_blocklist(&self) -> Result<HashSet<String>, GenericError> {
    let code = "SELECT * FROM Blocklist";
    let mut statement = self.connection.prepare(code).map_err(|error|
      GenericError::new("retrieving blocklist from the database")
        .add_error("failed to create a sqlite statement that selects the blocklist")
        .add_attachment("error", error.to_string())
    )?;
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("retrieving blocklist from the database")
        .add_error("failed to create a sqlite iterator")
        .add_attachment("error", error.to_string())
    )?;
    let mut domains = HashSet::new();
    loop {
      let row = iterator.next().map_err(|error|
        GenericError::new("retrieving blocklist from the database")
          .add_error("failed to retrieve the next blocklist item")
          .add_attachment("error", error.to_string())
      )?;
      let Some(row) = row else {
        return Ok(domains);
      };
      let domain: String = row.get("domain").map_err(|error| 
        GenericError::new("retrieving blocklist from the database")
          .add_error("failed to get the 'domain' column from a blocklist row")
          .add_attachment("error", error.to_string())
      )?;
      domains.insert(domain);
    }
  }

  pub fn add_domain_to_blocklist(&self, domain: &str) -> Result<(), GenericError> {
    let mut code = String::new();
    code.push_str("INSERT INTO Blocklist (domain) VALUES (");
    escape_string_for_sqilte_into(domain, &mut code);
    code.push_str(");");
    self.connection.execute_batch(&code).map_err(|error| 
      GenericError::new("adding a new blocklist item to the database")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code)
    )
  }
}

pub fn escape_string_for_sqilte_into(string: &str, into: &mut String) {
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

// fn escape_string_for_sqilte(string: &str) -> String {
//   let mut output = String::new();
//   escape_string_for_sqilte_into(string, &mut output);
//   output
// }