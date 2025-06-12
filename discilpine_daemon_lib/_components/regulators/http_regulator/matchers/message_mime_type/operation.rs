use serde::{Deserialize, Serialize};
use crate::{Rule, Uuid};
use super::Matcher;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
  Id,
  MimeType,
  ChangeMimeType(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
  Id(Uuid),
  MimeType(String),
  ChangeMimeType(Result<(), ()>)
}

impl Operation {
  pub fn execute(&self, matcher: &mut Matcher, rule: &Rule) -> Outcome {
    match self {
      Self::Id => {
        Outcome::Id(matcher.id.clone())
      }
      Self::MimeType => {
        Outcome::MimeType(matcher.mime_type.clone())
      }
      Self::ChangeMimeType(new_mime_type) => {
        Outcome::ChangeMimeType(matcher.guarded_change_mime_type(new_mime_type.clone(), rule))
      }
    }
  }
}