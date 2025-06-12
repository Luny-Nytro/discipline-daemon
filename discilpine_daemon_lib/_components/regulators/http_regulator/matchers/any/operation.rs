use serde::{Serialize, Deserialize};
use crate::Rule;
use super::Matcher;
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
pub enum Operation {
  ReqUri(Vec<req_uri::Operation>),
  ReqUriFilename(Vec<req_uri_filename::Operation>),
  ReqUriHash(Vec<req_uri_hash::Operation>),
  ReqUriPort(Vec<req_uri_port::Operation>),
  ReqUriScheme(Vec<req_uri_scheme::Operation>),
  ReqUriPath(Vec<req_uri_path::Operation>),
  ReqUriHostname(Vec<req_uri_hostname::Operation>),
  ReqMethod(Vec<req_method::Operation>),
  MessageMimeType(Vec<message_mime_type::Operation>),
  MessageMedaiKind(Vec<message_media_kind::Operation>),
  MessageHeaderValue(Vec<message_header_ValueRef::Operation>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  ReqUri(Result<Vec<req_uri::operation::Outcome>, ()>),
  ReqUriFilename(Result<Vec<req_uri_filename::operation::Outcome>, ()>),
  ReqUriHash(Result<Vec<req_uri_hash::operation::Outcome>, ()>),
  ReqUriPort(Result<Vec<req_uri_port::operation::Outcome>, ()>),
  ReqUriScheme(Result<Vec<req_uri_scheme::operation::Outcome>, ()>),
  ReqUriPath(Result<Vec<req_uri_path::operation::Outcome>, ()>),
  ReqUriHostname(Result<Vec<req_uri_hostname::operation::Outcome>, ()>),
  ReqMethod(Result<Vec<req_method::operation::Outcome>, ()>),
  MessageMimeType(Result<Vec<message_mime_type::operation::Outcome>, ()>),
  MessageMedaiKind(Result<Vec<message_media_kind::operation::Outcome>, ()>),
  MessageHeaderValue(Result<Vec<message_header_ValueRef::operation::Outcome>, ()>),
}

impl Operation {
  pub fn execute(&self, matcher: &mut Matcher, rule: &Rule) -> Outcome {
    match self {
      Self::ReqUri(operations) => Outcome::ReqUri(match matcher {
        Matcher::ReqUri(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::ReqUriFilename(operations) => Outcome::ReqUriFilename(match matcher {
        Matcher::ReqUriFilename(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::ReqUriHash(operations) => Outcome::ReqUriHash(match matcher {
        Matcher::ReqUriHash(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::ReqUriPort(operations) => Outcome::ReqUriPort(match matcher {
        Matcher::ReqUriPort(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::ReqUriScheme(operations) => Outcome::ReqUriScheme(match matcher {
        Matcher::ReqUriScheme(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::ReqUriPath(operations) => Outcome::ReqUriPath(match matcher {
        Matcher::ReqUriPath(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::ReqUriHostname(operations) => Outcome::ReqUriHostname(match matcher {
        Matcher::ReqUriHostname(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::ReqMethod(operations) => Outcome::ReqMethod(match matcher {
        Matcher::ReqMethod(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::MessageMimeType(operations) => Outcome::MessageMimeType(match matcher {
        Matcher::MessageMimeType(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::MessageMedaiKind(operations) => Outcome::MessageMedaiKind(match matcher {
        Matcher::MessageMedaiKind(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),
      Self::MessageHeaderValue(operations) => Outcome::MessageHeaderValue(match matcher {
        Matcher::MessageHeaderValue(matcher) => Ok(operations.iter().map(|operation| operation.execute(matcher, rule)).collect()),
        _ => Err(())
      }),           
    }
  }
}