use serde::{Deserialize, Serialize};
use crate::{matchers::ValueRef::Method, Rule, Uuid};
use super::Matcher;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
  Id,
  Method,
  ChangeMethod(Method),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
  Id(Uuid),
  Method(Method),
  ChangeMethod(Result<(), ()>),
}

impl Operation {
  pub fn execute(&self, matcher: &mut Matcher, rule: &Rule) -> Outcome {
    match self {
      Self::Id => {
        Outcome::Id(matcher.id.clone())
      }
      Self::Method => {
        Outcome::Method(matcher.method.clone())
      }
      Self::ChangeMethod(new_method) => {
        Outcome::ChangeMethod(matcher.guarded_change_method(new_method.clone(), rule))
      }
    }
  }
}