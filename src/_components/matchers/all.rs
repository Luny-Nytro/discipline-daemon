use serde::{Deserialize, Serialize};
use crate::Uuid;
use super::Matches;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid
}

impl<T> Matches<T> for Matcher {
  fn matches(&self, _value: &T) -> bool {
    true
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher { id: self.id.clone().unwrap_or_else(Uuid::new_v4) }
  }
}