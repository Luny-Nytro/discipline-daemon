use super::*;
use crate::*;
use rusqlite::Connection;
use std::{
  path::PathBuf,
  sync::{Arc, Mutex},
};

pub struct Database {
  pub connection: Arc<Mutex<Connection>>,
  pub operating_system_integration_linux_data: implementation
    ::operating_system_integration_linux_data
    ::DataCollection,
  pub operating_system_integration_linux_user: implementation
    ::operating_system_integration_linux_user
    ::UserCollection,
  pub screen_access_regulation_rule: implementation
    ::screen_access_regulation_rule
    ::RuleCollection,
  pub screen_access_regulation_policy: implementation
    ::screen_access_regulation_policy
    ::PolicyCollection,
  pub internet_access_regulation_rule: implementation
    ::internet_access_regulation_rule
    ::RuleCollection,
  pub internet_access_regulation_policy: implementation
    ::internet_access_regulation_policy
    ::PolicyCollection,
}

impl Database {
  pub fn open(database_directory_path: &PathBuf) -> Result<Self, GenericError> {
    if database_directory_path.is_relative() {
      return Err(
        GenericError::new("open a database connection")
          .add_error("database directory path must be absolute")
          .add_attachment(
            "database directory path",
            database_directory_path.to_string_lossy(),
          ),
      );
    }

    let connection =
      rusqlite::Connection::open(database_directory_path.join("data.db")).map_err(|error| {
        GenericError::new("open a database connection")
          .add_error(
            "the sqlite bindings crate, rusqlite, failed to open a sqlite database connection",
          )
          .add_attachment(
            "database directory path",
            database_directory_path.to_string_lossy(),
          )
          .add_attachment("error", error.to_string())
      })?;

    let database = Database {
      connection: Arc::new(Mutex::new(connection)),
      operating_system_integration_linux_data:
        implementation::operating_system_integration_linux_data::DataCollection::new(
          "OperatingSystemIntegrationLinuxData".into(),
        ),

      operating_system_integration_linux_user:
        implementation::operating_system_integration_linux_user::UserCollection::new(
          "OperatingSystemIntegrationLinuxUsers".into(),
        ),

      screen_access_regulation_policy:
        implementation::screen_access_regulation_policy::PolicyCollection::new(
          "ScreenAccessRegulationPolicies".into(),
        ),

      screen_access_regulation_rule:
        implementation::screen_access_regulation_rule::RuleCollection::new(
          "ScreenAccessRegulationRules".into(),
        ),
      internet_access_regulation_policy: implementation
        ::internet_access_regulation_policy
        ::PolicyCollection
        ::new("InternetAccessRegulationPolicies".into()),

      internet_access_regulation_rule: implementation
        ::internet_access_regulation_rule
        ::RuleCollection
        ::new("InternetAccessRegulationRules".into()),
    };

    let mut definitions = DatabaseCode::new();
    implementation
      ::operating_system_integration_linux_data
      ::write_define(&database, &mut definitions);

    implementation
      ::operating_system_integration_linux_user
      ::write_define(&database, &mut definitions);

    implementation
      ::screen_access_regulation_policy
      ::write_define(&database, &mut definitions);

    implementation
      ::screen_access_regulation_rule
      ::write_define(&database, &mut definitions);

    implementation
      ::internet_access_regulation_policy
      ::write_define(&database, &mut definitions);

    implementation
      ::internet_access_regulation_rule
      ::write_define(&database, &mut definitions);

    database.execute(definitions.as_str())?;

    Ok(database)
  }

  pub(super) fn execute(&self, code: &str) -> Result<(), GenericError> {
    if code.is_empty() {
      return Ok(());
    }

    self
      .connection
      .lock()
      .unwrap()
      .execute_batch(code)
      .map_err(|error| {
        GenericError::new("execute sql code")
          .add_attachment("error", error.to_string())
          .add_attachment("code", code)
      })
  }
}

pub struct DatabaseCode {
  pub(super) code: String,
}

impl DatabaseCode {
  pub fn new() -> Self {
    Self {
      code: String::new(),
    }
  }

  pub fn write(&mut self, str: &str) {
    self.code.push_str(str);
  }

  pub fn as_ref(&self) -> &String {
    &self.code
  }

  pub fn as_str(&self) -> &str {
    &self.code
  }

  pub fn as_mut(&mut self) -> &mut String {
    &mut self.code
  }
}

// pub struct DatabaseCode {
//   code: String
// }

// impl DatabaseCode {
//   pub fn new() -> Self {
//     Self {
//       code: String::new(),
//     }
//   }

//   pub(super) fn write(&self, string: &str) {

//   }

//   pub(super) fn as_mut(&mut self) -> &mut String {
//     &mut self.code
//   }

//   pub(super) fn as_ref(&self) -> &str {
//     &self.code
//   }
// }
