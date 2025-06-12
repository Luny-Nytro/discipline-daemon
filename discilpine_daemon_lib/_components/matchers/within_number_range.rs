use serde::{Deserialize, Serialize};
use crate::{matchers::{Matches, Value}, Rule, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub min: i64,
  pub max: i64,
}

impl Matches<i64> for Matcher {
  fn matches(&self, value: &i64) -> bool {
    self.min <= *value && *value <= self.max 
  }
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    if let ValueRef::Number(value) = value {
      self.matches(value)
    } else {
      false
    }
  }
}

impl Matcher {
  pub fn check_permission_modify(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn guarded_change_min(&mut self, new_min: i64, rule: &Rule) -> Result<(), ()> {
    self.check_permission_modify(rule)?;
    self.min = new_min;
    Ok(())
  }

  pub fn guarded_change_max(&mut self, new_max: i64, rule: &Rule) -> Result<(), ()> {
    self.check_permission_modify(rule)?;
    self.min = new_max;
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub min: i64,
  pub max: i64,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: self.id.clone().unwrap_or_else(Uuid::new_v4),
      max: self.max,
      min: self.min,
    }
  }
}