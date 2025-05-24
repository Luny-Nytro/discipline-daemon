use serde::{Serialize, Deserialize};
use crate::{App, DateTime, Duration, IsOperation, Uuid, OperatingSystemUsername};
use crate::networking_access::{Action, Activator, database_procedures};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
  NoSuchEnforcer,
  NoSuchRule,
  WrongActivatorType,
  WouldMakeRuleLessRestrictive,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  username: OperatingSystemUsername,
  decrement: Duration,
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.app_data.networking_access;

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.username == self.username) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    let Some(rule) = enforcer
      .rules
      .iter_mut()
      .find(|rule| rule.id == self.rule_id) else 
    {
      return Err(Error::NoSuchRule);
    };

    let now = DateTime::now();
    if rule.is_enabled(now) && rule.action == Action::Block {
      return Err(Error::WouldMakeRuleLessRestrictive);
    }

    let Activator::ForDuration(countdown_timer) = &mut rule.activator else {
      return Err(Error::WrongActivatorType);
    };

    let new_remaining_duration = countdown_timer
      .remaining_duration()
      .checked_sub(&self.decrement)
      .unwrap_or(Duration::ZERO);

    if let Err(_) = database_procedures::activator_for_duration_change_remaining_duration(
      &app.database_connection,
      &self.rule_id,
      &new_remaining_duration
    ) {
      return Err(Error::InternalError);
    }

    countdown_timer.change_remaining_duration(new_remaining_duration);
    Ok(())
  }
}
