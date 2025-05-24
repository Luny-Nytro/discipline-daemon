use serde::{Deserialize, Serialize};
use crate::{matchers::{self, ValueRef::HttpReq}, Uuid};
use matchers::{Matches, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub child: Option<matchers::Any>,
}

impl Matches<HttpReq> for Matcher {
  fn matches(&self, value: &HttpReq) -> bool {
    if let Some(child) = &self.child {
      child.matches(&ValueRef::String(value.get_uri_filename().into()))
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub child: Option<matchers::any::Creator>
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