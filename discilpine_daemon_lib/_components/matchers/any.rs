use serde::{Deserialize, Serialize};
use crate::{Uuid, matchers};
use matchers::{Value, Matches};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Matcher {
  Or(Box<super::Or>),
  And(Box<super::And>),
  All(Box<super::All>),
  Not(Box<super::Not>),
  None(Box<super::None>),
  Number(Box<super::Number>),
  String(Box<super::String>),
  WithinNumberRange(Box<super::WithinNumberRange>),
  FacebookBlacklist(Box<super::FacebookBlacklist>),
}

impl Matcher {
  pub fn id(&self) -> &Uuid {
    match self {
      Matcher::Or(matcher) => &matcher.id,
      Matcher::And(matcher) => &matcher.id,
      Matcher::All(matcher) => &matcher.id,
      Matcher::Not(matcher) => &matcher.id,
      Matcher::None(matcher) => &matcher.id,
      Matcher::Number(matcher) => &matcher.id,
      Matcher::String(matcher) => &matcher.id,
      Matcher::WithinNumberRange(matcher) => &matcher.id,
      Matcher::FacebookBlacklist(matcher) => &matcher.id,
    }
  }
  pub fn kind(&self) -> Kind {
    match self {
      Matcher::Or(_) => Kind::Or,
      Matcher::And(_) => Kind::And,
      Matcher::All(_) => Kind::All,
      Matcher::Not(_) => Kind::Not,
      Matcher::None(_) => Kind::None,
      Matcher::Number(_) => Kind::Number,
      Matcher::String(_) => Kind::String,
      Matcher::WithinNumberRange(_) => Kind::WithinNumberRange,
      Matcher::FacebookBlacklist(_) => Kind::FacebookBlacklist,
    }
  }
}

impl Matches for Matcher {
  fn matches(&self, value: &Value) -> bool {
    match self {
      Matcher::Or(matcher) => matcher.matches(value),
      Matcher::And(matcher) => matcher.matches(value),
      Matcher::All(matcher) => matcher.matches(value),
      Matcher::Not(matcher) => matcher.matches(value),
      Matcher::None(matcher) => matcher.matches(value),
      Matcher::Number(matcher) => matcher.matches(value),
      Matcher::String(matcher) => matcher.matches(value),
      Matcher::WithinNumberRange(matcher) => matcher.matches(value),
      Matcher::FacebookBlacklist(matcher) => matcher.matches(value),
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Kind {
  Or,
  And,
  All,
  Not,
  None,
  String,
  Number,
  WithinNumberRange,
  FacebookBlacklist,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Creator {
  Or(Box<matchers::or::Creator>),
  And(Box<matchers::and::Creator>),
  All(Box<matchers::all::Creator>),
  Not(Box<matchers::not::Creator>),
  None(Box<matchers::none::Creator>),
  String(Box<matchers::string::Creator>),
  Number(Box<matchers::number::Creator>),
  WithinNumberRange(Box<matchers::within_number_range::Creator>),
  FacebookBlacklist(Box<matchers::facebook_blacklist::Creator>),
}

impl Creator {
  pub fn create(&self) -> Matcher {
    match self {
      Self::Or(creator) => Matcher::Or(Box::new(creator.create())),
      Self::And(creator) => Matcher::And(Box::new(creator.create())),
      Self::All(creator) => Matcher::All(Box::new(creator.create())),
      Self::Not(creator) => Matcher::Not(Box::new(creator.create())),
      Self::None(creator) => Matcher::None(Box::new(creator.create())),
      Self::Number(creator) => Matcher::Number(Box::new(creator.create())),
      Self::String(creator) => Matcher::String(Box::new(creator.create())),
      Self::WithinNumberRange(creator) => Matcher::WithinNumberRange(Box::new(creator.create())),
      Self::FacebookBlacklist(creator) => Matcher::FacebookBlacklist(Box::new(creator.create())),
    }
  }
}