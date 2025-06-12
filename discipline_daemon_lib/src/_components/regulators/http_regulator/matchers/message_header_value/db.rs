use std::fmt::Write;
use super::Matcher;
use crate::{matchers, Uuid, db::{Updates, SerializeValue}};

pub fn add_one(matcher: &Matcher, session: &mut impl Write) {
  if let Some(matcher) = &matcher.header_value {
    matchers::any::db::add_one(matcher, session);
  }

  let id = matcher.id.serialize();
  let header_name = matcher.header_name.serialize();
  
  let (header_value_matcher_id, header_value_matcher_kind) = match &matcher.header_value {
    Some(matcher) => (
      matcher.id().serialize(),
      matcher.kind().serialize(),
    ),
    None => (
      "NULL".into(),
      "NULL".into(),
    )
  };

  writeln!(session, "INSERT INTO http_message_header_value_matchers VALUES ({\
    id\
  }, {\
    header_name\
  }, {\
    header_value_matcher_id\
  }, {\
    header_value_matcher_kind\
  });").unwrap();
}

pub fn find_all(session: &mut impl Write) {
  writeln!(session, "SELECT * FROM http_message_header_value_matchers;").unwrap();
}

pub fn update_one(prev: &Matcher, curr: &Matcher, session: &mut impl Write) {
  let mut updates = Updates::new();
  
  if prev.header_name != curr.header_name {
    updates.add("header_name", &curr.header_name.serialize());
  }
  
  match (&prev.header_value, &curr.header_value) {
    (None, None) => {
      // noop
    }
    (Some(prev), None) => {
      updates.add("header_value_matcher_id", "NULL");
      updates.add("header_value_matcher_kind", "NULL");
      matchers::any::db::delete_one(prev.id(), prev.kind(), session);
    }
    (None, Some(curr)) => {
      updates.add("header_value_matcher_id", &curr.id().serialize());
      updates.add("header_value_matcher_kind", &curr.kind().serialize());
      matchers::any::db::add_one(curr, session);
    }
    (Some(prev), Some(curr)) => {
      if prev.id() != curr.id() {
        updates.add("header_value_matcher_id", &curr.id().serialize());
      }
      if prev.kind() != curr.kind() {
        updates.add("header_value_matcher_kind", &curr.kind().serialize());
      }
      matchers::any::db::update_one(prev, curr, session);
    }
  }

  if updates.is_not_empty() {
    let id = curr.id.serialize();
    writeln!(session, "UPDATE http_message_header_value_matchers SET {updates} WHERE id = {id} ;").unwrap();
  }
}

pub fn delete_one(id: &Uuid, session: &mut impl Write) {
  let id = id.serialize();
  writeln!(session, "DELETE FROM http_message_header_value_matchers WHERE id = {id} ;").unwrap();
}