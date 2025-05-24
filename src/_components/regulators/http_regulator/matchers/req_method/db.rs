use std::fmt::Write;
use super::Matcher;
use crate::{Uuid, db::SerializeValue};

pub fn add_one(matcher: &Matcher, session: &mut impl Write) {
  let id = matcher.id.serialize();
  let method = matcher.method.serialize();
  writeln!(session, "INSERT INTO http_req_method_matchers VALUES ({id}, {method});").unwrap();
}

pub fn find_all(session: &mut impl Write) {
  writeln!(session, "SELECT * FROM http_req_method_matchers;").unwrap();
}

pub fn update_one(prev: &Matcher, curr: &Matcher, session: &mut impl Write) {
  if prev.method != curr.method {
    let id = curr.id.serialize();
    let method = curr.method.serialize();
    writeln!(session, "UPDATE http_req_method_matchers SET method = {method} WHERE id = {id} ;").unwrap();
  }
}

pub fn delete_one(id: &Uuid, session: &mut impl Write) {
  let id = id.serialize();
  writeln!(session, "DELETE FROM http_req_method_matchers WHERE id = {id} ;").unwrap();
}