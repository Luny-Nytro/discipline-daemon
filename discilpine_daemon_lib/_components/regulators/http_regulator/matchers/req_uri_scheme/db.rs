use std::fmt::Write;
use super::{Matcher};
use crate::{Uuid, db::SerializeValue};

pub fn add_one(matcher: &Matcher, session: &mut impl Write) {
  let id = matcher.id.serialize();
  let scheme = matcher.scheme.serialize();
  writeln!(session, "INSERT INTO http_req_uri_scheme_matchers VALUES ({id}, {value});").unwrap();
}

pub fn find_all(session: &mut impl Write) {
  writeln!(session, "SELECT * FROM http_req_uri_scheme_matchers;").unwrap();
}

pub fn update_one(prev: &Matcher, curr: &Matcher, session: &mut impl Write) {
  if prev.scheme != curr.scheme {
    let id = curr.id.serialize();
    let scheme = curr.scheme.serialize();
    writeln!(session, "UPDATE http_req_uri_scheme_matchers SET scheme = {scheme} WHERE id = {id} ;").unwrap();
  }
}

pub fn delete_one(id: &Uuid, session: &mut impl Write) {
  let id = id.serialize();
  writeln!(session, "DELETE FROM http_req_uri_scheme_matchers WHERE id = {id} ;").unwrap();
}