use std::fmt::Write;
use crate::{db::{SerializeValue, Updates}, matchers, Uuid};
use super::Matcher;

pub fn add_one(matcher: &Matcher, session: &mut impl Write) {
  if let Some(child) = &matcher.child {
    matchers::any::db::add_one(child, session);
  }

  let id = matcher.id.serialize();

  let (child_id, child_kind) = match &matcher.child {
    Some(child) => (
      child.id().serialize(),
      child.kind().serialize(),
    ),
    None => (
      "NULL".into(),
      "NULL".into(),
    )
  };

  writeln!(session, "INSERT INTO http_uri_hostname_matchers VALUES ({id}, {child_id}, {child_kind});").unwrap();
}

pub fn find_all(session: &mut impl Write) {
  writeln!(session, "SELECT * FROM http_uri_hostname_matchers;").unwrap();
}

pub fn update_one(prev: &Matcher, curr: &Matcher, session: &mut impl Write) {
  let mut updates = Updates::new();
  
  match (&prev.child, &curr.child) {
    (None, None) => {
      // noop
    }
    (Some(prev), Some(curr)) => {
      if prev.id() != curr.id() {
        updates.add("child_id", &curr.id().serialize());
      }
      if prev.kind() != curr.kind() {
        updates.add("child_kind", &curr.kind().serialize());
      }
      matchers::any::db::update_one(prev, curr, session);
    }
    (None, Some(curr)) => {
      updates.add("child_id", &curr.id().serialize());
      updates.add("child_kind", &curr.kind().serialize());
      matchers::any::db::add_one(curr, session);
    }
    (Some(prev), None) => {
      updates.add("child_id", "NULL");
      updates.add("child_kind", "NULL");
      matchers::any::db::delete_one(prev.id(), prev.kind(), session);
    }
  }

  if updates.is_not_empty() {
    let id = curr.id.serialize();
    writeln!(session, "UPDATE http_uri_filename_matchers SET {updates} WHERE id = {id} ;").unwrap();
  }
}

pub fn delete_one(id: &Uuid, session: &mut impl Write) {
  let id = id.serialize();
  writeln!(session, "DELETE FROM http_uri_filename_matchers WHERE id = {id} ;").unwrap();
}