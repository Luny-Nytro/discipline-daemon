use crate::{matchers, Uuid};
use matchers::{ValueRef::Scheme, Value, Matches};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matcher {
  pub id: Uuid,
  pub scheme: Scheme
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    if let ValueRef::HttpReq(req) = value {
      req.uri.scheme == self.scheme
    } else {
      false
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub scheme: Scheme,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: match &self.id {
        Some(id) => id.clone(),
        None => Uuid::new_v4(),
      },
      scheme: self.scheme.clone(),
    }
  }
}