use crate::{
  password_enabler, CountdownTimer, Duration, OperatingSystemUserId, OperatingSystemUsername, PasswordEnabler, TimeRange, Uuid, WeekdayRange
};

use crate::database::{
  Deserialize, DeserializeContext, ScalarTypeError, 
  ScalarTypeSerde, SerializationContext, Serialize, 
  ValueRef,
};

use super::{
  Activator, OperatingSystemCalls, Enabler, Rule, Enforcer 
};

use crate::{
  weekday_range, countdown_timer, 
  time_range,
};

// SECTION: Enabler.
pub enum EnablerType {
  ForDuration,
  ByPassword,
}

impl ScalarTypeSerde for EnablerType {
  fn serialize(&self) -> String {
    match self {
      Self::ForDuration => 0.serialize(),
      Self::ByPassword => 1.serialize(),        
    }
  }

  fn deserialize(value: ValueRef) -> Option<Self> {
    match value {
      ValueRef::Integer(0) => Some(Self::ForDuration), 
      ValueRef::Integer(1) => Some(Self::ByPassword), 
      _ => None,
    }
  }
}

impl Serialize for Enabler {
  fn serialize_into(&self, ctx: &mut SerializationContext) {
    match self {
      Self::ForDuration(inner) => {
        ctx.scalar(&EnablerType::ForDuration);
        ctx.compound(inner);
      } 
      Self::ByPassword(inner) => {
        ctx.scalar(&EnablerType::ByPassword);
        ctx.compound(inner);
        ctx.null();
      }
    }
  }
}

#[derive(Debug)]
pub enum EnablerDeserializeError {
  Type(ScalarTypeError),
  ForDuration(countdown_timer::database_serde::DeserializeError),
  ByPassword(password_enabler::database_serde::DeserializeError),  
}

impl Deserialize for Enabler {
  type Error = EnablerDeserializeError;

  fn columns_number() -> usize {
    // This this for the Type column.
    1
    + 
    CountdownTimer::columns_number()
  }

  fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
    let r#type = match ctx.scalar_type() {
      Ok(value) => value,
      Err(error) => return Err(EnablerDeserializeError::Type(error))
    };

    match r#type {
      EnablerType::ForDuration => {
        match ctx.compund_type() {
          Ok(value) => Ok(Enabler::ForDuration(value)),
          Err(error) => Err(EnablerDeserializeError::ForDuration(error)),
        }
      }
      EnablerType::ByPassword => {
        match ctx.compund_type() {
          Ok(value) => Ok(Enabler::ByPassword(value)),
          Err(error) => Err(EnablerDeserializeError::ByPassword(error)),
        }
      }
    }
  }
}

// SECTION: Activator.
#[derive(Debug)]
pub enum ActivatorType {
  AtWeekday,
  NotAtWeekday,
  InTimeRange,
  NotInTimeRange,
  AtHour,
  NotAtHour,
  ForDuration,
  InWeekdayRange,
  NotInWeekdayRange,
}

impl ScalarTypeSerde for ActivatorType {
  fn serialize(&self) -> String {
    match self {
      Self::AtWeekday => 0.serialize(),
      Self::NotAtWeekday => 1.serialize(),
      Self::InTimeRange => 2.serialize(),
      Self::NotInTimeRange => 3.serialize(),
      Self::AtHour => 4.serialize(),
      Self::NotAtHour => 5.serialize(),
      Self::ForDuration => 6.serialize(),
      Self::InWeekdayRange => 7.serialize(),
      Self::NotInWeekdayRange => 8.serialize(),        
    }
  }

  fn deserialize(value: ValueRef) -> Option<Self> {
    match value {
      ValueRef::Integer(0) => Some(Self::AtWeekday), 
      ValueRef::Integer(1) => Some(Self::NotAtWeekday), 
      ValueRef::Integer(2) => Some(Self::InTimeRange), 
      ValueRef::Integer(3) => Some(Self::NotInTimeRange), 
      ValueRef::Integer(4) => Some(Self::AtHour), 
      ValueRef::Integer(5) => Some(Self::NotAtHour), 
      ValueRef::Integer(6) => Some(Self::ForDuration), 
      ValueRef::Integer(7) => Some(Self::InWeekdayRange), 
      ValueRef::Integer(8) => Some(Self::NotInWeekdayRange),  
      _ => None,
    }
  }
}

impl Serialize for Activator {
  fn serialize_into(&self, ctx: &mut SerializationContext) {
    match self {
      Self::AtWeekday(weekday) => {
        ctx.scalar(&ActivatorType::AtWeekday);
        ctx.scalar(weekday);
        ctx.null();
        ctx.null();
      } 
      Self::NotAtWeekday(weekday) => {
        ctx.scalar(&ActivatorType::NotAtWeekday);
        ctx.scalar(weekday);
        ctx.null();
        ctx.null();
      } 
      Self::InTimeRange(time_range) => {
        ctx.scalar(&ActivatorType::InTimeRange);
        ctx.compound(time_range);
        ctx.null();
      } 
      Self::NotInTimeRange(time_range) => {
        ctx.scalar(&ActivatorType::NotInTimeRange);
        ctx.compound(time_range);
        ctx.null();
      } 
      Self::AtHour(hour) => {
        ctx.scalar(&ActivatorType::AtHour);
        ctx.scalar(hour);
        ctx.null();
        ctx.null();
      } 
      Self::NotAtHour(hour) => {
        ctx.scalar(&ActivatorType::NotAtHour);
        ctx.scalar(hour);
        ctx.null();
        ctx.null();
      } 
      Self::ForDuration(countdown_timer) => {
        ctx.scalar(&ActivatorType::ForDuration);
        ctx.compound(countdown_timer);
      } 
      Self::InWeekdayRange(weekday_range) => {
        ctx.scalar(&ActivatorType::InWeekdayRange);
        ctx.compound(weekday_range);
        ctx.null();
      } 
      Self::NotInWeekdayRange(weekday_range) => {
        ctx.scalar(&ActivatorType::NotInWeekdayRange);
        ctx.compound(weekday_range);
        ctx.null();
      }
    }
  }
}

#[derive(Debug)]
pub enum ActivatorDeserializeError {
  Type(ScalarTypeError),
  AtWeekday(ScalarTypeError),
  NotAtWeekday(ScalarTypeError),
  InTimeRange(time_range::database_serde::DeserializeError),
  NotInTimeRange(time_range::database_serde::DeserializeError),
  AtHour(ScalarTypeError),
  NotAtHour(ScalarTypeError),
  ForDuration(countdown_timer::database_serde::DeserializeError),
  InWeekdayRange(weekday_range::database_serde::DeserializeError),
  NotInWeekdayRange(weekday_range::database_serde::DeserializeError),
}

impl Deserialize for Activator {
  type Error = ActivatorDeserializeError;

  fn columns_number() -> usize {
    // This is for the Type column.
    1

    + PasswordEnabler::columns_number()
      .max(TimeRange::columns_number())
      .max(WeekdayRange::columns_number())
      .max(CountdownTimer::columns_number())
  }

  fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
    let r#type = match ctx.scalar_type() {
      Ok(value) => value,
      Err(error) => return Err(ActivatorDeserializeError::Type(error))
    };

    match r#type {
      ActivatorType::AtHour => {
        match ctx.scalar_type() {
          Ok(hour) => Ok(Activator::AtHour(hour)),
          Err(error) => Err(ActivatorDeserializeError::AtHour(error)),
        }
      }
      ActivatorType::AtWeekday => {
        match ctx.scalar_type() {
          Ok(weekday) => Ok(Activator::AtWeekday(weekday)),
          Err(error) => Err(ActivatorDeserializeError::AtWeekday(error)),
        }
      }
      ActivatorType::ForDuration => {
        match ctx.compund_type() {
          Ok(countdown_timer) => Ok(Activator::ForDuration(countdown_timer)),
          Err(error) => Err(ActivatorDeserializeError::ForDuration(error)),
        }
      }
      ActivatorType::InTimeRange => {
        match ctx.compund_type() {
          Ok(time_range) => Ok(Activator::InTimeRange(time_range)),
          Err(error) => Err(ActivatorDeserializeError::InTimeRange(error)),
        }
      }
      ActivatorType::InWeekdayRange => {
        match ctx.compund_type() {
          Ok(weekday_range) => Ok(Activator::InWeekdayRange(weekday_range)),
          Err(error) => Err(ActivatorDeserializeError::InWeekdayRange(error)),
        }
      }
      ActivatorType::NotAtHour => {
        match ctx.scalar_type() {
          Ok(hour) => Ok(Activator::NotAtHour(hour)),
          Err(error) => Err(ActivatorDeserializeError::NotAtHour(error)),
        }
      }
      ActivatorType::NotAtWeekday => {
        match ctx.scalar_type() {
          Ok(weekday) => Ok(Activator::NotAtWeekday(weekday)),
          Err(error) => Err(ActivatorDeserializeError::NotAtWeekday(error)),
        }
      }
      ActivatorType::NotInTimeRange => {
        match ctx.compund_type() {
          Ok(time_range) => Ok(Activator::NotInTimeRange(time_range)),
          Err(error) => Err(ActivatorDeserializeError::NotInTimeRange(error)),
        }
      }
      ActivatorType::NotInWeekdayRange => {
        match ctx.compund_type() {
          Ok(weekday_range) => Ok(Activator::NotInWeekdayRange(weekday_range)),
          Err(error) => Err(ActivatorDeserializeError::NotInWeekdayRange(error)),
        }
      }
    }
  }
}

// SECTION: Rule.
pub struct RuleSerializer<'a> {
  rule: &'a Rule,
  username: &'a OperatingSystemUsername,
  position: usize,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    rule: &'a Rule, 
    username: &'a OperatingSystemUsername, 
    position: usize,
  ) -> Self {
    Self {
      rule,
      username,
      position,
    }
  }
}

impl<'a> Serialize for RuleSerializer<'a> {
  fn serialize_into(&self, ctx: &mut SerializationContext) {
    ctx.scalar(&self.rule.id);
    ctx.compound(&self.rule.activator);
    ctx.compound(&self.rule.enabler);
    ctx.scalar(self.username);
    ctx.scalar(&self.position);
  }
}

#[derive(Debug, Clone)]
pub struct RuleNormalized {
  pub(super) id: Uuid,
  pub(super) activator: Activator,
  pub(super) enabler: Enabler,
  pub(super) username: OperatingSystemUsername,
  pub(super) position: usize,
}

impl RuleNormalized {
  pub fn finalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
      enabler: self.enabler,
    }
  }
}

#[derive(Debug)]
pub enum RuleNormalizedDeserializeError {
  Id(ScalarTypeError),
  Action(ScalarTypeError),
  Activator(ActivatorDeserializeError),
  Enabler(EnablerDeserializeError),
  Username(ScalarTypeError),
  Position(ScalarTypeError),
}

impl Deserialize for RuleNormalized {
  type Error = RuleNormalizedDeserializeError;

  fn columns_number() -> usize {
    // This is for the Id, Username and Position columns.
    3 

    + Activator::columns_number()
    + CountdownTimer::columns_number()
  }

  fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
    Ok(RuleNormalized {
      id: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(RuleNormalizedDeserializeError::Id(error)),
      },
      activator: match ctx.compund_type() {
        Ok(value) => value,
        Err(error) => return Err(RuleNormalizedDeserializeError::Activator(error)),
      },
      enabler: match ctx.compund_type() {
        Ok(value) => value,
        Err(error) => return Err(RuleNormalizedDeserializeError::Enabler(error)),
      },
      username: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(RuleNormalizedDeserializeError::Username(error)),
      },
      position: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(RuleNormalizedDeserializeError::Position(error)),
      },
    })
  }
}

// SECTION: Enforcer.
impl Serialize for Enforcer {
  fn serialize_into(&self, ctx: &mut SerializationContext) {
    ctx.scalar(&self.user_id);
    ctx.scalar(&self.username);
    ctx.scalar(&self.is_blocked);
    ctx.scalar(&self.is_enabled);
  }
}

pub struct EnforcerNormalized {
  pub(super) user_id: OperatingSystemUserId,
  pub(super) username: OperatingSystemUsername,
  pub(super) is_blocked: bool,
  pub(super) is_enabled: bool,
}

#[derive(Debug)]
pub enum EnforcerDserializeError {
  UserId(ScalarTypeError),
  Username(ScalarTypeError),
  IsBlocked(ScalarTypeError),
  IsEnabled(ScalarTypeError),
}

impl Deserialize for EnforcerNormalized {
  type Error = EnforcerDserializeError;

  fn columns_number() -> usize {
    4
  }

  fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
    Ok(EnforcerNormalized {
      user_id: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(EnforcerDserializeError::UserId(error)),
      },
      username: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(EnforcerDserializeError::Username(error))
      },
      is_blocked:  match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(EnforcerDserializeError::IsBlocked(error))
      },
      is_enabled:  match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(EnforcerDserializeError::IsEnabled(error))
      },
    })
  }
}

impl EnforcerNormalized {
  pub fn finalize(self, rules: Vec<Rule>) -> Enforcer {
    Enforcer { 
      rules, 
      user_id: self.user_id,
      username: self.username, 
      is_blocked: self.is_blocked, 
      is_enabled: self.is_enabled, 
      operating_system_calls: OperatingSystemCalls::new(),
    }
  }
}

// SECTION: Feature.
pub struct FeatureNormalized {
  pub(super) enforcing_interval: Duration
}

impl Default for FeatureNormalized {
  fn default() -> Self {
    Self {
      enforcing_interval: Duration::from_minutes(5).unwrap(),
    }
  }
}

impl Serialize for FeatureNormalized {
  fn serialize_into(&self, ctx: &mut SerializationContext) {
    ctx.scalar(&self.enforcing_interval);
  }
}

#[derive(Debug)]
pub enum FeatureDeserializeError {
  ApplyingInterval(ScalarTypeError),
}

impl Deserialize for FeatureNormalized {
  type Error = FeatureDeserializeError;

  fn columns_number() -> usize {
    1
  }

  fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
    Ok(Self {
      enforcing_interval: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(FeatureDeserializeError::ApplyingInterval(error)),
      }
    })
  }
}