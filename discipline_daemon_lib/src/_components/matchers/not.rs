use serde::{Deserialize, Serialize};
use crate::Uuid;
use super::{Value, Matches};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Matcher {
  pub id: Uuid,
  pub item: Option<super::Any>,
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    match &self.item {
      Some(child) => !child.matches(value),
      None => false,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creator {
  pub child: Option<super::any::Creator>,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: Uuid::new_v4(),
      item: match &self.child {
        Some(creator) => Some(creator.create()),
        None => None,
      },
    }
  }
}