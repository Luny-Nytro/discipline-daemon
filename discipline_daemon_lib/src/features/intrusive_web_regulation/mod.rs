/// Allow the user to specify conditions for regulating web access.
/// 
/// Regulations include:
/// - blocking websites matching specified donain, url, .etc
/// 
/// Conditions include:
/// - Webpage url, or specific url component
/// - Whether an image classifier classifies an image as containing nudity
/// - Media type (image, video, pdf, binary, android app package, .etc)
/// - Time of day
/// - Day of week
/// - Webpage view time allowance
/// - Webpage vist count allowance
/// - YouTube video id, title, description or category
/// - YouTube channel id, title, description or category
/// - YouTube video sensistive status
/// 
/// Create a simple language to allow the user to match components and
/// media types within webpages and specify what to do with them:
/// - remove them if they are components
/// - blur or replace them if they are images
/// - blur them or remove the sound if they are videos

mod media_types;

use serde::{Deserialize, Serialize};
use crate::{DateTime, Hour, Uuid, Weekday};
use super::{CountdownTimer, TimeRange, PasswordAuthenticationStatus, WeekdayRange};

// SECTION: Protector.
#[derive(Debug, Clone)]
pub enum Protector {
  AtWeekday(Weekday),
  NotAtWeekday(Weekday),
  InTimeRange(TimeRange),
  NotInTimeRange(TimeRange),
  AtHour(Hour),
  NotAtHour(Hour),
  CountdownTimer(CountdownTimer),
  InWeekdayRange(WeekdayRange),
  NotInWeekdayRange(WeekdayRange),
  PasswordAuthenticationStatus(PasswordAuthenticationStatus),
}

impl Protector {
  pub fn is_protecting(&mut self, now: DateTime) -> bool {
    match self {
      Protector::AtWeekday(weekday) => {
        now.weekday() == *weekday
      }
      Protector::NotAtWeekday(weekday) => {
        now.weekday() != *weekday
      }
      Protector::AtHour(hour) => {
        now.hour() == *hour
      }
      Protector::NotAtHour(hour) => {
        now.hour() != *hour
      }
      Protector::InTimeRange(time_range) => {
        time_range.contains(now.time())
      }
      Protector::NotInTimeRange(time_range) => {
        !time_range.contains(now.time())
      }
      Protector::CountdownTimer(countdown) => {
        !countdown.is_finished()
      }
      Protector::InWeekdayRange(weekday_range) => {
        weekday_range.contains(now.weekday())
      }
      Protector::NotInWeekdayRange(weekday_range) => {
        !weekday_range.contains(now.weekday())
      }
      Protector::PasswordAuthenticationStatus(password_authentication_status) => {
        password_authentication_status.is_locked()
      }
    }
  }  
}

// SECTION: Activator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Activator {
  AtWeekday(Weekday),
  NotAtWeekday(Weekday),
  InTimeRange(TimeRange),
  NotInTimeRange(TimeRange),
  AtHour(Hour),
  NotAtHour(Hour),
  CountdownTimer(CountdownTimer),
  InWeekdayRange(WeekdayRange),
  NotInWeekdayRange(WeekdayRange),
}

impl Activator {
  pub fn is_activated(&mut self, now: DateTime) -> bool {
    match self {
      Activator::AtWeekday(weekday) => {
        now.weekday() == *weekday
      }
      Activator::NotAtWeekday(weekday) => {
        now.weekday() != *weekday
      }
      Activator::AtHour(hour) => {
        now.hour() == *hour
      }
      Activator::NotAtHour(hour) => {
        now.hour() != *hour
      }
      Activator::InTimeRange(time_range) => {
        time_range.contains(now.time())
      }
      Activator::NotInTimeRange(time_range) => {
        !time_range.contains(now.time())
      }
      Activator::CountdownTimer(countdown) => {
        countdown.synchronize(now);
        countdown.is_running()
      }
      Activator::InWeekdayRange(weekday_range) => {
        weekday_range.contains(now.weekday())
      }
      Activator::NotInWeekdayRange(weekday_range) => {
        !weekday_range.contains(now.weekday())
      }
    }
  }
}

// SECTION: Action.
pub enum Action {
  Allow,
  Block,
}

// SECTION: MediaTypeRule.
pub enum MediaType {
  AllImageTypes,
  AllVideoTypes,
  AllAudioTypes,

}

pub struct MediaTypeRule {
  id: Uuid,
  protector: Protector, 
  activator: Activator, 
  media_type: MediaType,
  action: Action,
  message_head_matcher: (),
  actions: (),
}
// 
pub enum RuleAction {
  BlockMediaTypes,
  AllowMediaTypes,

}
// [block/allow] [message/media types] when [always/never/countdown timer/hour/uri/]
// []
pub struct GoogleSafeSearchRule {
  id: Uuid,
  activator: (),
  protector: (),
  action: GoogleSafeSearchAction,
}

pub enum GoogleSafeSearchAction {
  Blur,
  Filter,
  Disable,
}

pub struct Enforcer {
  rules: Vec<Rule>,
}


// GoogleSafeSearch
// YahooSafeSearch
// BingSafeSearch
// Block youtube videos
// Allow youtube videos
// Allow youtube videos
// Allow Urls
// Block Urls
// Block media types
// allow media types


pub struct Rule {
  id: Uuid,
  
  action: Action,
}

pub enum Matcher {
  
  String(String),
  Boolean(bool),
  Number(i64),
  DateTime(DateTime),
  BeforeDateTime(DateTime),
  AfterDateTime(DateTime),
}

pub struct StringMatcher {
  string: String,
  is_case_sensitive: bool,
}

pub struct StringStartMatcher {
  string: String,
  is_case_sensitive: bool
}

pub struct StringEndMatcher {
  string: String,
  is_case_sesnitive: bool,
}

