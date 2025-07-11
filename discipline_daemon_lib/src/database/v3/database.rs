use std::path::PathBuf;
use rusqlite::Connection;
use super::implementation::*;
use crate::{database::v3::implementation, *};

pub struct Database {
  pub connection: Connection,
  pub app: AppCollection,
  pub user: UserCollection,
  pub user_screen_access_regulation_rule: UserScreenAccessRuleCollection,
  pub user_screen_access_regulation_policy: UserScreenAccessPolicyCollection,
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
      app: AppCollection::new(
        "App".into(), 
        "Id".into(), 
        "UserScreenAccessRegulationPrivatePassword".into(), 
        "UserScreenAccessRegulationApplingInterval".into(),
      ),
      user: UserCollection::new(
        "Users".into(), 
        "Id".into(), 
        "UserName".into(), 
        "OperatingSystemUserId".into(), 
        "OperatingSystemUserName".into(), 
        "OperatingSystemUserPassword".into(), 
        "UserScreenAccessRegulationIsApplyingEnabled".into(), 
        "UserScreenAccessRegulationIsUserScreenAccessBlocked".into(),
      ),
      user_screen_access_regulation_policy: UserScreenAccessPolicyCollection::new(
        "UserScreenAccessRegulationPolicies".into(), 
        "Id".into(), 
        "Name".into(), 
        "UserId".into(), 
        "EnablerDuration".into(), 
        "EnablerRemainingDuration".into(), 
        "EnablerPreviousSynchronizationTime".into(),
      ),
      user_screen_access_regulation_rule: UserScreenAccessRuleCollection::new(
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

    let mut initialization_code = String::new();
    database.app.write_definition_into(&mut initialization_code);
    database.user.write_definition_into(&mut initialization_code);
    database.user_screen_access_regulation_rule.write_definition_into(&mut initialization_code);
    database.user_screen_access_regulation_policy.write_definition_into(&mut initialization_code);

    database.execute(&initialization_code)?;

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

  pub fn create_user_update_draft(&self) -> user_collection::UserUpdateDraft {
    self.user.create_user_update_draft(self)
  }
  pub fn create_user_collection_update_draft(&self) -> user_collection::UserCollectionUpdateDraft {
    self.user.create_collection_update_draft(self)
  }
  pub fn create_user_screen_access_regulation_rule_update_draft(&self) -> screen_access_regulation_rule_collection::RuleUpdateDraft {
    self.user_screen_access_regulation_rule.create_rule_update_draft(self)
  }
  pub fn create_user_screen_access_regulation_rule_collection_update_draft(&self) -> screen_access_regulation_rule_collection::RuleCollectionUpdateDraft {
    self.user_screen_access_regulation_rule.create_collection_update_draft(self)
  }
  pub fn create_user_screen_access_regulation_policy_update_draft(&self) -> screen_access_regulation_policy_collection::PolicyUpdateDraft {
    self.user_screen_access_regulation_policy.create_policy_update_draft(self)
  }
  pub fn create_user_screen_access_regulation_policy_collection_update_draft(&self) -> screen_access_regulation_policy_collection::PolicyCollectionUpdateDraft {
    self.user_screen_access_regulation_policy.create_collection_update_draft(self)
  }

  pub fn load_app_state(&self) {
    
  }
}