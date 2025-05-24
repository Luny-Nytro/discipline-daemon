use serde::{Deserialize, Serialize};
use crate::{Rule, Uuid};
use super::Matcher;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
  Id,
  Scheme,
  ChangeScheme(Scheme),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
  Id(Uuid),
  Scheme(Scheme),
  ChangeScheme(Result<(), ()>),
}

impl Operation {
  pub fn execute(&self, matcher: &Matcher, rule: &Rule) -> Outcome {
    match self {
      Self::Id => {
        Outcome::Id(matcher.id.clone())
      }
      Self::Scheme => {
        Outcome::Scheme(matcher.scheme.clone())
      }
      Self::ChangeScheme(new_scheme) => {
        Outcome::ChangeScheme(matcher.guarded_change_scheme(new_scheme.clone(), rule))
      }
    }
  }
}