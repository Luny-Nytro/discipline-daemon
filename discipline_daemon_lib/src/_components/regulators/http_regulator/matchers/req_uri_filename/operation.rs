use serde::{Deserialize, Serialize};
use crate::{Rule, Uuid};
use super::Matcher;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
  Id,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
  Id(Uuid),
}

impl Operation {
  pub fn execute(&self, matcher: &Matcher, rule: &Rule) -> Outcome {
    match self {
      Self::Id => {
        Outcome::Id(matcher.id.clone())
      }
    }
  }
}