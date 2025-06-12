use serde::{Deserialize, Serialize};
use crate::{Rule, Uuid};
use super::{Matcher, MediaKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
  Id,
  MediaKind,
  ChangeMediaKind(MediaKind)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
  Id(Uuid),
  MediaKind(MediaKind),
  ChangeMediaKind(Result<(), ()>),
}

impl Operation {
  pub fn execute(&self, matcher: &mut Matcher, rule: &Rule) -> Outcome {
    match self {
      Self::Id => {
        Outcome::Id(matcher.id.clone())
      }
      Self::MediaKind => {
        Outcome::MediaKind(matcher.media_kind.clone())
      }
      Self::ChangeMediaKind(new_media_kind) => {
        Outcome::ChangeMediaKind(matcher.guarded_change_media_kind(new_media_kind.clone(), rule))
      }
    }
  }
}