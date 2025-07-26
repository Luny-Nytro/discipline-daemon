use std::path::PathBuf;
use rusqlite::Connection;
use super::implementation::*;
use super::*;
use crate::*;

pub struct Database {
  pub connection: Connection,
  pub common: AppCollection,
  pub operating_system_integration_linux_user: implementation::operating_system_integration_linux_user::UserCollection,
  pub screen_access_regulation_rule: implementation::screen_access_regulation_policy_integration::RuleCollection,
  pub screen_access_regulation_policy: implementation::screen_access_regulation_rule_integration::PolicyCollection,
}

impl Database {
  pub fn open(
    database_directory_path: PathBuf,
  ) -> 
    Result<Self, GenericError> 
  {

    if database_directory_path.is_relative() {
      return Err(
        GenericError::new("open a database connection")
          .add_error("database directory path must be absolute")
          .add_attachment("database directory path", database_directory_path.to_string_lossy())
      )
    }

    let connection = rusqlite::Connection::open(database_directory_path.join("data.db")).map_err(|error| {
      GenericError::new("open a database connection")
        .add_error("the sqlite bindings crate, rusqlite, failed to open a sqlite database connection")
        .add_attachment("database directory path", database_directory_path.to_string_lossy())
        .add_attachment("error", error.to_string())
    })?;

    let database = Database {
      connection,
      common: AppCollection::new(
        "App".into(), 
        "Id".into(), 
        "UserScreenAccessRegulationPrivatePassword".into(), 
        "UserScreenAccessRegulationApplingInterval".into(),
      ),
      operating_system_integration_linux_user: UserCollection::new(
        "Users".into(), 
        "Id".into(), 
        "UserName".into(), 
        "OperatingSystemUserId".into(), 
        "OperatingSystemUserName".into(), 
        "OperatingSystemUserPassword".into(), 
        "UserScreenAccessRegulationIsApplyingEnabled".into(), 
        "UserScreenAccessRegulationIsUserScreenAccessBlocked".into(),
      ),
      screen_access_regulation_policy: UserScreenAccessPolicyCollection::new(
        "UserScreenAccessRegulationPolicies".into()
      ),
      screen_access_regulation_rule: UserScreenAccessRuleCollection::new(
        "UserScreenAccessRegulationRules".into(), 
        "Id".into(), 
        "UserId".into(), 
        "PolicyId".into(), 
        "ActivatorEnumType".into(), 
        "ActivatorEnumData1".into(), 
        "ActivatorEnumData2".into(), 
        "Position".into(),
      )
    };

    let mut definitions = DatabaseCode::new();
    app_collection::write_define(&database, &mut definitions);
    // user_collection::write_define(&database, &mut definitions);
    screen_access_regulation_rule_integration::write_define(&database, &mut definitions);
    screen_access_regulation_policy_integration::write_define(&database, &mut definitions);

    database.execute(definitions.as_str())?;

    Ok(database)
  }

  pub(super) fn execute(&self, code: &str) -> 
    Result<(), GenericError>
  {
    if code.is_empty() {
      return Ok(());
    }

    self.connection.execute_batch(code).map_err(|error|
      GenericError::new("execute sql code")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code)
    )
  }
}

pub struct DatabaseCode {
  pub(super) code: String
}

impl DatabaseCode {
  pub fn new() -> Self {
    Self {
      code: String::new()
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