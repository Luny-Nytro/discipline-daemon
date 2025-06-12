use serde::{Deserialize, Serialize};
use crate::{Rule, Uuid};
use super::{Matches, Value};

/// This matcher matches string `Value`s against the string it has in its `string`
/// field.
/// 
/// Matching can be case sensitive or insensitive.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub value: String,
  pub is_case_sensitive: bool,
}

impl Matches<String> for Matcher {
  fn matches(&self, value: &String) -> bool {
    if self.is_case_sensitive {
      self.value == *value
    } else {
      self.value.to_lowercase() == value.to_lowercase()
    }
  }
}

impl Matches<Value> for Matcher {
  fn matches(&self, value: &Value) -> bool {
    if let ValueRef::String(value) = value {
      self.matches(value)
    } else {
      false
    }
  }
}

impl Matcher {
  pub fn check_permission_change_string(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn check_permission_change_is_case_sensitive(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn guarded_change_string(&mut self, new_string: String, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_string(rule)?;
    self.value = new_string;
    Ok(())
  }

  pub fn guarded_change_is_case_sensitive(&mut self, new_value: bool, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_string(rule)?;
    self.is_case_sensitive = new_value;
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub value: String,
  pub is_case_sensitive: Option<bool>,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: match &self.id {
        Some(id) => id.clone(),
        None => Uuid::new_v4(),
      },
      value: self.value.clone(),
      is_case_sensitive: match &self.is_case_sensitive {
        Some(value) => *value,
        None => true,
      }
    }
  }
}