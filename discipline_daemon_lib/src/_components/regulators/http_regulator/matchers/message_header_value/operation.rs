use serde::{Deserialize, Serialize};
use crate::{Rule, Uuid, matchers};
use super::Matcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
  Id,
  HeaderName,
  ChangeHeaderName(String),
  HeaderValueMatcher(matchers::any::Operation),
  ChangeHeaderValueMatcher(matchers::Any),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  Id(Uuid),
  HeaderName(String),
  ChangeHeaderName,
  HeaderValueMatcher(matchers::any::operation::Outcome),
  ChangeHeaderValueMatcher,
}

impl Operation {
  pub fn execute(&self, matcher: &mut Matcher, rule: &Rule) -> Result<Outcome, impl Into<ApiOperationError>> {
    match self {
      Self::Id => {
        Ok(Outcome::Id(matcher.id.clone()))
      }
      
      Self::HeaderName => {
        Ok(Outcome::HeaderName(matcher.header_name.clone()))
      }
      
      
      Self::HeaderValueMatcher(operation) => {
        match &matcher.header_value {
          Some(ref mut header_value) => {
            Ok(Outcome::HeaderValueMatcher(operation.execute(header_value, rule)))
          }
          None => {
            Err(Error::HeaderValueMatcherIsNull)
          }
        }
      }
      
      Self::ChangeHeaderName(new_header_name) => {
        Ok(Outcome::ChangeHeaderName(matcher.guarded_change_header_name(new_header_name.clone(), rule)))
      }
      
      Self::ChangeHeaderValueMatcher(new_matcher) => {
        if let Err(()) = matcher.guarded_change_header_value_matcher(new_matcher.clone(), rule) {
          Err(Error::MayNotChangeHeaderValueMatcherNow)
        } else {
          Ok(Outcome::ChangeHeaderValueMatcher)
        }
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  HeaderValueMatcherIsNull,
  MayNotChangeHeaderValueMatcherNow,
}

impl Into<ApiOperationError> for Error {
  fn into(self) -> ApiOperationError {
    todo!()
  }
}

pub struct ApiOperationExecutionContext {
  
}

impl ApiOperationExecutionContext {
  pub fn err(&mut self, error: impl Into<ApiOperationError>) {

  }
}

pub enum ApiOperationError {

}