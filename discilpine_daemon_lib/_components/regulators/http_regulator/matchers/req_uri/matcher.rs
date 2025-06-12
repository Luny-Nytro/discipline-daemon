use serde::{Deserialize, Serialize};
use crate::{Uuid, matchers, Rule};
use matchers::{ValueRef::HttpReq, Matches, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub child: Option<matchers::Any>,
}

impl Matches<HttpReq> for Matcher {
  fn matches(&self, value: &HttpReq) -> bool {
    if let Some(child) = &self.child {
      child.matches(&ValueRef::String(value.uri.href.clone()))  
    } else {
      false
    }
  }
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    if let ValueRef::HttpReq(value) = value {
      self.matches(value)  
    } else {
      false
    }
  }
}

impl Matcher {
  pub fn check_permission_change_child(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn guarded_change_child(&mut self, new_child: Matcher, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_child(rule)?;
    self.child = new_child;
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub child: Option<matchers::any::Creator>,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: match &self.id {
        Some(id) => id.clone(),
        None => Uuid::new_v4(),
      },
      child: match &self.child {
        Some(creator) => Some(creator.create()),
        None => None,
      },
    }
  }
}