use serde::{Serialize, Deserialize};
use crate::{App, Duration, IsOperation, OperatingSystemUsername, Uuid};
use crate::networking_access::{database_procedures, Activator};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
  NoSuchRule,
  NoSuchEnforcer,
  WrongEnablerType,
  WouldBeEffectiveForTooLong,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  username: OperatingSystemUsername,
  increment: Duration,
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    // Get feature.
    let feature = &mut app.app_data.networking_access;

    // Get rule enforcer.
    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.username == self.username) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    // Get rule.
    let Some(rule) = enforcer
      .rules
      .iter_mut()
      .find(|rule| rule.id == self.rule_id) else 
    {
      return Err(Error::NoSuchRule);
    };
    
    // Get activator.
    let Activator::ForDuration(countdown_timer) = &mut rule.activator else {
      return Err(Error::WrongEnablerType);
    };

    let Some(new_remaining_duration) = countdown_timer
      .remaining_duration()
      .checked_add(&self.increment) else 
    {
      return Err(Error::WouldBeEffectiveForTooLong);
    };

    if new_remaining_duration.total_weeks() > 3 {
      return Err(Error::WouldBeEffectiveForTooLong);
    }

    if let Err(_) = database_procedures::enabler_for_duration_change_remaining_duration(
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
