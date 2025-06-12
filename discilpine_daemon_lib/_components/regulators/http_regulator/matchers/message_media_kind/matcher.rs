use serde::{Deserialize, Serialize};
use crate::{matchers::{self, ValueRef::{HttpReq, HttpRes}}, Rule, Uuid};
use matchers::{Value, Matches};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum MediaKind {
  /// Matches any image.
  Image,
  /// Matches any video.
  Video,
  /// Matches any font.
  Font,
  /// Matches any archive.
  Archive,
  /// Matches any HTML file or similar files.
  WebPage,
  /// Matches any CSS file or similar files.
  WebStyle,
  /// Matches any JavaScript file or similar files.
  WebScript,
  /// Matches any JavaScript file.
  JavaScript,
  /// Matches any TypeScript file.
  TypeScript,
  /// Matches any binary file or stream.
  Binary,
  /// Matches any shell script file.
  ShellScript,
  /// Any text file.
  Text,
  /// Any document (including text files).
  Document,
  /// Gif or similar images.
  GifLike,
  /// Matches any icon.
  Icon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matcher {
  pub id: Uuid,
  pub media_kind: MediaKind,
}

impl Matches<HttpReq> for Matcher {
  fn matches(&self, value: &HttpReq) -> bool {
    todo!()
  }
}

impl Matches<HttpRes> for Matcher {
  fn matches(&self, value: &HttpRes) -> bool {
    todo!()
  }
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    match value {
      ValueRef::HttpReq(http_req) => self.matches(http_req),
      ValueRef::HttpRes(http_res) => self.matches(http_res),
      _ => false,
    }
  }
}

impl Matcher {
  pub fn check_permission_change_media_kind(&self, rule: &Rule) -> Result<(), ()> {
    todo!()
  }

  pub fn guarded_change_media_kind(&mut self, new_media_kind: MediaKind, rule: &Rule) -> Result<(), ()> {
    self.check_permission_change_media_kind(rule)?;
    self.media_kind = new_media_kind;
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
  pub id: Option<Uuid>,
  pub media_kind: MediaKind,
}

impl Creator {
  pub fn create(&self) -> Matcher {
    Matcher {
      id: match &self.id {
        Some(id) => id.clone(),
        None => Uuid::new_v4(),
      },
      media_kind: self.media_kind.clone(),
    }
  }
}