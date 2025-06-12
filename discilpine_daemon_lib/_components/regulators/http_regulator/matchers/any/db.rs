use std::fmt::Write;
use crate::{db::SerializeValue, Uuid};
use super::{Kind, Matcher};
use super::super::{
  message_header_value,
  message_media_kind,
  message_mime_type,
  req_method,
  req_uri,
  req_uri_filename,
  req_uri_hash,
  req_uri_hostname,
  req_uri_path,
  req_uri_port,
  req_uri_scheme,
};

pub fn add_one(matcher: &Matcher, session: &mut impl Write) {
  match matcher {
    Matcher::ReqUri(matcher) => { req_uri::db::add_one(matcher, session); }
    Matcher::ReqMethod(matcher) => { req_method::db::add_one(matcher, session); }
    Matcher::ReqUriHash(matcher) => { req_uri_hash::db::add_one(matcher, session); }
    Matcher::ReqUriPath(matcher) => { req_uri_path::db::add_one(matcher, session); }
    Matcher::ReqUriPort(matcher) => { req_uri_port::db::add_one(matcher, session); }
    Matcher::ReqUriScheme(matcher) => { req_uri_scheme::db::add_one(matcher, session); }
    Matcher::ReqUriHostname(matcher) => { req_uri_hostname::db::add_one(matcher, session); }
    Matcher::ReqUriFilename(matcher) => { req_uri_filename::db::add_one(matcher, session); }
    Matcher::MessageMimeType(matcher) => { message_mime_type::db::add_one(matcher, session); }
    Matcher::MessageMedaiKind(matcher) => { message_media_kind::db::add_one(matcher, session); }
    Matcher::MessageHeaderValue(matcher) => { message_header_ValueRef::db::add_one(matcher, session); }
  }
}

pub fn update_one(prev: &Matcher, curr: &Matcher, session: &mut impl Write) {
  match (prev, curr) {
    (Matcher::ReqUri(prev), Matcher::ReqUri(curr)) => { 
      req_uri::db::update_one(prev, curr, session); 
    }
    (Matcher::ReqMethod(prev), Matcher::ReqMethod(curr)) => { 
      req_method::db::update_one(prev, curr, session); 
    }
    (Matcher::ReqUriHash(prev), Matcher::ReqUriHash(curr)) => { 
      req_uri_hash::db::update_one(prev, curr, session); 
    }
    (Matcher::ReqUriPath(prev), Matcher::ReqUriPath(curr)) => { 
      req_uri_path::db::update_one(prev, curr, session); 
    }
    (Matcher::ReqUriPort(prev), Matcher::ReqUriPort(curr)) => { 
      req_uri_port::db::update_one(prev, curr, session); 
    }
    (Matcher::ReqUriScheme(prev), Matcher::ReqUriScheme(curr)) => { 
      req_uri_scheme::db::update_one(prev, curr, session); 
    }
    (Matcher::ReqUriHostname(prev), Matcher::ReqUriHostname(curr)) => { 
      req_uri_hostname::db::update_one(prev, curr, session); 
    }
    (Matcher::ReqUriFilename(prev), Matcher::ReqUriFilename(curr)) => { 
      req_uri_filename::db::update_one(prev, curr, session); 
    }
    (Matcher::MessageMimeType(prev), Matcher::MessageMimeType(curr)) => { 
      message_mime_type::db::update_one(prev, curr, session); 
    }
    (Matcher::MessageMedaiKind(prev), Matcher::MessageMedaiKind(curr)) => { 
      message_media_kind::db::update_one(prev, curr, session); 
    }
    (Matcher::MessageHeaderValue(prev), Matcher::MessageHeaderValue(curr)) => { 
      message_header_ValueRef::db::update_one(prev, curr, session); 
    }
    (prev, curr) => {
      delete_one(prev.id(), &prev.kind(), session);
      add_one(curr, session);
    }
  }
}

pub fn delete_one(id: &Uuid, kind: &Kind, session: &mut impl Write) {
  match kind {
    Kind::ReqUri => { req_uri::db::delete_one(id, session); }
    Kind::ReqMethod => { req_method::db::delete_one(id, session); }
    Kind::ReqUriHash => { req_uri_hash::db::delete_one(id, session); }
    Kind::ReqUriPort => { req_uri_port::db::delete_one(id, session); }
    Kind::ReqUriPath => { req_uri_path::db::delete_one(id, session); }
    Kind::ReqUriScheme => { req_uri_scheme::db::delete_one(id, session); }
    Kind::ReqUriFilename => { req_uri_filename::db::delete_one(id, session); }
    Kind::ReqUriHostname => { req_uri_hostname::db::delete_one(id, session); }
    Kind::MessageMimeType => { message_mime_type::db::delete_one(id, session); }
    Kind::MessageMedaiKind => { message_media_kind::db::delete_one(id, session); }
    Kind::MessageHeaderValue => { message_header_ValueRef::db::delete_one(id, session); }
  }
}

impl SerializeValue for Kind {
  fn serialize(&self) -> String {
    match self {
      Self::ReqUri => 0u8.serialize(),
      Self::ReqUriFilename => 1u8.serialize(),
      Self::ReqUriHash => 2u8.serialize(),
      Self::ReqUriPort => 3u8.serialize(),
      Self::ReqUriScheme => 4u8.serialize(),
      Self::ReqUriPath => 5u8.serialize(),
      Self::ReqUriHostname => 6u8.serialize(),
      Self::ReqMethod => 7u8.serialize(),
      Self::MessageMimeType => 8u8.serialize(),
      Self::MessageMedaiKind => 9u8.serialize(),
      Self::MessageHeaderValue => 10u8.serialize(),    
    }
  }
}