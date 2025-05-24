use std::fmt::Write;
use crate::{db::SerializeValue, Uuid};
use super::{Matcher, MediaKind};

pub fn add_one(matcher: &Matcher, session: &mut impl Write) {
  let id = matcher.id.serialize();
  let media_kind = matcher.media_kind.serialize();
  writeln!(session, "INSERT INTO http_message_media_kind_matchers VALUES ({id}, {media_kind});").unwrap();
}

pub fn find_all(session: &mut impl Write) {
  writeln!(session, "SELECT * FROM http_message_media_kind_matchers;").unwrap();
}

pub fn update_one(prev: &Matcher, curr: &Matcher, session: &mut impl Write) {
  if prev.media_kind != curr.media_kind {
    let id = curr.id.serialize();
    let media_kind = curr.media_kind.serialize();
    writeln!(session, "UPDATE http_message_media_kind_matchers SET media_kind = {media_kind} ;").unwrap();
  }
}

pub fn delete_one(id: &Uuid, session: &mut impl Write) {
  let id = id.serialize();
  writeln!(session, "DELETE FROM http_message_media_kind_matchers WHERE id = {id} ;").unwrap();
}

impl SerializeValue for MediaKind {
  fn serialize(&self) -> String {
    match self {
      Self::Image => 0u8.serialize(),
      Self::Video => 1u8.serialize(),
      Self::Font => 2u8.serialize(),
      Self::Archive => 3u8.serialize(),
      Self::WebPage => 4u8.serialize(),
      Self::WebStyle => 5u8.serialize(),
      Self::WebScript => 6u8.serialize(),
      Self::JavaScript => 7u8.serialize(),
      Self::TypeScript => 8u8.serialize(),
      Self::Binary => 9u8.serialize(),
      Self::ShellScript => 10u8.serialize(),
      Self::Text => 11u8.serialize(),
      Self::Document => 12u8.serialize(),
      Self::GifLike => 13u8.serialize(),
      Self::Icon => 14u8.serialize(),

    }
  }
}