use serde::{Serialize, Deserialize};
use crate::{Rule, Uuid};
use super::{Value, Matches};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub children: Vec<super::Any>
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    self.children.iter().all(|item| item.matches(value))
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub children: Vec<super::any::Creator>,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher { 
      id: self.id.clone().unwrap_or_else(Uuid::new_v4),
      children: self.children.iter().map(|create| create.create()).collect(),
    }
  }
}