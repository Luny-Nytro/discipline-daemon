use serde::{Deserialize, Serialize};
use crate::{matchers, Rule, Uuid};
use matchers::{ValueRef::{HttpReq, HttpRes}, Matches, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub mime_type: String,
}

impl Matches<HttpReq> for Matcher {
  fn matches(&self, value: &HttpReq) -> bool {
    if let Some(mime_type) = value.get_header_value_as_str("content-type") {
      mime_type == self.mime_type
    } else {
      false
    }
  }
}

impl Matches<HttpRes> for Matcher {
  fn matches(&self, value: &HttpRes) -> bool {
    if let Some(mime_type) = value.get_header_value_as_str("content-type") {
      mime_type == self.mime_type
    } else {
      false
    }
  }
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    match value {
      ValueRef::HttpReq(value) => self.matches(value),
      ValueRef::HttpRes(value) => self.matches(value),
      _ => false,
    }    
  }
}

impl Matcher {
  pub fn check_permission_change_mime_type(&self, rule: &Rule) -> Result<(), ()> {
    todo!()
  }
  pub fn guarded_change_mime_type(&mut self, new_mime_type: String, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_mime_type(rule)?;
    self.mime_type = new_mime_type;
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub mime_type: String,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: match &self.id {
        Some(id) => id.clone(),
        None => Uuid::new_v4(),
      },
      mime_type: self.mime_type.clone(),
    }
  }
}