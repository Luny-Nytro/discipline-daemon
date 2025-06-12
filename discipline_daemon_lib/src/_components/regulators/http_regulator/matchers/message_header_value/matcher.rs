use serde::{Deserialize, Serialize};
use crate::{matchers, Rule, Uuid};
use matchers::{ValueRef::HttpReq, Matches, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub header_name: String,
  pub header_value: Option<matchers::Any>,
}

impl Matches<HttpReq> for Matcher {
  fn matches(&self, value: &HttpReq) -> bool {
    let Some(header_value) = value.get_header_value_as_str(&self.header_name) else {
      return false;
    };

    let Some(header_value_matcher) = &self.header_value else {
      return false;
    };

    header_value_matcher.matches(&ValueRef::String(header_value.into()))
  }
}

impl Matches<Value> for Matcher {
  fn matches(&self, value: &Value) -> bool {
    if let ValueRef::HttpReq(value) = value {
      self.matches(value)
    } else {
      false
    }
  }
}

impl Matcher {
  pub fn check_permission_change_header_name(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn guarded_change_header_name(&mut self, new_header_name: String, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_header_name(rule)?;
    self.header_name = new_header_name;
    Ok(())
  }

  pub fn check_permission_change_header_value_matcher(&self, rule: &Rule) -> Result<(), ()> {
    if rule.is_guarded() {
      Err(())
    } else {
      Ok(())
    }
  }

  pub fn guarded_change_header_value_matcher(&mut self, new_matcher: matchers::Any, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_header_value_matcher(rule)?;
    self.header_value = Some(new_matcher);
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub header_name: String,
  pub header_value: Option<matchers::any::Creator>,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: match &self.id {
        Some(id) => id.clone(),
        None => Uuid::new_v4(),
      },
      header_name: self.header_name.clone(),
      header_value: match &self.header_value {
        Some(creator) => Some(creator.create()),
        None => None,
      }
    }
  }
}