use serde::{Deserialize, Serialize};
use crate::{matchers::{ValueRef::{HttpReq, Method}, Matches, Value}, Rule, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Matcher {
  pub id: Uuid,
  pub method: Method,
}

impl Matches<HttpReq> for Matcher {
  fn matches(&self, value: &HttpReq) -> bool {
    value.get_method() == self.method
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
  pub fn check_permission_change_method(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }
  
  pub fn guarded_change_method(&mut self, new_method: Method, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_method(rule)?;
    self.method = new_method;
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub method: Method,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: match &self.id {
        Some(id) => id.clone(),
        None => Uuid::new_v4(),
      },
      method: self.method.clone(),
    }
  }
}