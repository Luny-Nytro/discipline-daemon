use serde::{Deserialize, Serialize};
use crate::{matchers::{Matches, Value}, Uuid};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Matcher {
  ReqUri(req_uri::Matcher),
  ReqUriFilename(req_uri_filename::Matcher),
  ReqUriHash(req_uri_hash::Matcher),
  ReqUriPort(req_uri_port::Matcher),
  ReqUriScheme(req_uri_scheme::Matcher),
  ReqUriPath(req_uri_path::Matcher),
  ReqUriHostname(req_uri_hostname::Matcher),
  ReqMethod(req_method::Matcher),
  MessageMimeType(message_mime_type::Matcher),
  MessageMedaiKind(message_media_kind::Matcher),
  MessageHeaderValue(message_header_ValueRef::Matcher),
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    match self {
      Self::ReqUri(matcher) => matcher.matches(value),
      Self::ReqUriFilename(matcher) => matcher.matches(value),
      Self::ReqUriHash(matcher) => matcher.matches(value),
      Self::ReqUriPort(matcher) => matcher.matches(value),
      Self::ReqUriScheme(matcher) => matcher.matches(value),
      Self::ReqUriPath(matcher) => matcher.matches(value),
      Self::ReqUriHostname(matcher) => matcher.matches(value),
      Self::ReqMethod(matcher) => matcher.matches(value),
      Self::MessageMimeType(matcher) => matcher.matches(value),
      Self::MessageMedaiKind(matcher) => matcher.matches(value),
      Self::MessageHeaderValue(matcher) => matcher.matches(value),
    }
  }
}

impl Matcher {
  pub const fn kind(&self) -> Kind {
    match self {
      Self::ReqUri(_) => Kind::ReqUri,
      Self::ReqUriFilename(_) => Kind::ReqUriFilename,
      Self::ReqUriHash(_) => Kind::ReqUriHash,
      Self::ReqUriPort(_) => Kind::ReqUriPort,
      Self::ReqUriScheme(_) => Kind::ReqUriScheme,
      Self::ReqUriPath(_) => Kind::ReqUriPath,
      Self::ReqUriHostname(_) => Kind::ReqUriHostname,
      Self::ReqMethod(_) => Kind::ReqMethod,
      Self::MessageMimeType(_) => Kind::MessageMimeType,
      Self::MessageMedaiKind(_) => Kind::MessageMedaiKind,
      Self::MessageHeaderValue(_) => Kind::MessageHeaderValue,
    }
  }

  pub const fn id(&self) -> &Uuid {
    match self {
      Self::ReqUri(matcher) => &matcher.id,
      Self::ReqUriFilename(matcher) => &matcher.id,
      Self::ReqUriHash(matcher) => &matcher.id,
      Self::ReqUriPort(matcher) => &matcher.id,
      Self::ReqUriScheme(matcher) => &matcher.id,
      Self::ReqUriPath(matcher) => &matcher.id,
      Self::ReqUriHostname(matcher) => &matcher.id,
      Self::ReqMethod(matcher) => &matcher.id,
      Self::MessageMimeType(matcher) => &matcher.id,
      Self::MessageMedaiKind(matcher) => &matcher.id,
      Self::MessageHeaderValue(matcher) => &matcher.id,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Kind {
  ReqUri,
  ReqUriFilename,
  ReqUriHash,
  ReqUriPort,
  ReqUriScheme,
  ReqUriPath,
  ReqUriHostname,
  ReqMethod,
  MessageMimeType,
  MessageMedaiKind,
  MessageHeaderValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Creator {
  ReqUri(req_uri::Creator),
  ReqUriFilename(req_uri_filename::Creator),
  ReqUriHash(req_uri_hash::Creator),
  ReqUriPort(req_uri_port::Creator),
  ReqUriScheme(req_uri_scheme::Creator),
  ReqUriPath(req_uri_path::Creator),
  ReqUriHostname(req_uri_hostname::Creator),
  ReqMethod(req_method::Creator),
  MessageMimeType(message_mime_type::Creator),
  MessageMedaiKind(message_media_kind::Creator),
  MessageHeaderValue(message_header_ValueRef::Creator),   
}

impl Creator {
  pub fn create(&self) -> Matcher {
    match self {
      Self::ReqUri(creator) => Matcher::ReqUri(creator.create()),
      Self::ReqUriFilename(creator) => Matcher::ReqUriFilename(creator.create()),
      Self::ReqUriHash(creator) => Matcher::ReqUriHash(creator.create()),
      Self::ReqUriPort(creator) => Matcher::ReqUriPort(creator.create()),
      Self::ReqUriScheme(creator) => Matcher::ReqUriScheme(creator.create()),
      Self::ReqUriPath(creator) => Matcher::ReqUriPath(creator.create()),
      Self::ReqUriHostname(creator) => Matcher::ReqUriHostname(creator.create()),
      Self::ReqMethod(creator) => Matcher::ReqMethod(creator.create()),
      Self::MessageMimeType(creator) => Matcher::MessageMimeType(creator.create()),
      Self::MessageMedaiKind(creator) => Matcher::MessageMedaiKind(creator.create()),
      Self::MessageHeaderValue(creator) => Matcher::MessageHeaderValue(creator.create()),
    }
  }
}