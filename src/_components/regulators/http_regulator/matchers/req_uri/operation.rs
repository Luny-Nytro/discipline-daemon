use serde::{Deserialize, Serialize};
use crate::{matchers, Rule, Uuid};
use super::Matcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
  Id,
  Child(matchers::any::Operation),
  ChangeChild(matchers::Any),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  Id(Uuid),
  Child(matchers::any::operation::Outcome),
  ChangeChild(Result<(), ()>),
}

impl Operation {
  pub fn execute(&self, matcher: &mut Matcher, rule: &Rule) -> Outcome {
    match self {
      Self::Id => {
        Outcome::Id(matcher.id.clone())
      }
      Self::Child(operation) => {
        Outcome::Child(operation.execute(&mut matcher.child, rule))
      }
      Self::ChangeChild(new_child) => {
        Outcome::ChangeChild(matcher.guarded_change_child(new_child.clone(), rule))
      }
    }
  }
}