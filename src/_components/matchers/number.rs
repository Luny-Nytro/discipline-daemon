use crate::{Rule, Uuid};
use serde::{Deserialize, Serialize};
use super::{Matches, Value};

/// This matcher matches isize numbers against the isize number in its `number`
/// field.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Matcher {
  pub id: Uuid,
  pub value: i64,
}

impl Matches<i64> for Matcher {
  fn matches(&self, value: &i64) -> bool {
    *value == self.value
  }
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    match value {
      ValueRef::Number(number) => self.matches(number),
      _ => false,
    }
  }
}

impl Matcher {
  pub fn check_permission_change_value(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn guarded_change_value(&mut self, new_value: i64, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_value(rule)?;
    self.value = new_value;
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub value: i64
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: self.id.clone().unwrap_or_else(Uuid::new_v4),
      value: self.value,
    }
  }
}
