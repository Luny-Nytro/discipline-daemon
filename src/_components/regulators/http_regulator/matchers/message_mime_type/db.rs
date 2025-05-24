use std::fmt::Write;
use super::Matcher;
use crate::{db::SerializeValue, Uuid};

pub fn add_one(matcher: &Matcher, session: &mut impl Write) {
  let id = matcher.id.serialize();
  let mime_type = matcher.mime_type.serialize();
  writeln!(session, "INSERT INTO http_message_mime_type_matchers VALUES ({id}, {mime_type});").unwrap();
}

pub fn find_all(session: &mut impl Write) {
  writeln!(session, "SELECT * FROM http_message_mime_type_matchers;").unwrap();
}

pub fn update_one(prev: &Matcher, curr: &Matcher, session: &mut impl Write) {
  if prev.mime_type != curr.mime_type {
    let id = curr.id.serialize();
    let mime_type = curr.mime_type.serialize();
    writeln!(session, "UPDATE http_message_mime_type_matchers SET mime_type = {mime_type} WHERE id = {id} ;").unwrap();
  }
}

pub fn delete_one(id: &Uuid, session: &mut impl Write) {
  let id = id.serialize();
  writeln!(session, "DELETE FROM http_message_mime_type_matchers WHERE id = {id} ;").unwrap()
}