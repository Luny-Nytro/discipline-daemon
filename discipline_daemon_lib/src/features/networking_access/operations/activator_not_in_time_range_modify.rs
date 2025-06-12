use serde::{Serialize, Deserialize};
use crate::{App, IsOperation, TimeRange, Uuid, OperatingSystemUsername};
use crate::networking_access::{database_procedures, Activator};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
  NoSuchRule,
  NoSuchEnforcer,
  WouldMakeRuleLessRestrictive,
  WrongActivatorType,
  // WouldBlockForTooLong,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  username: OperatingSystemUsername,
  new_time_range: TimeRange,
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  // TODO: Refuse to execute this operation if it would result in making 
  // the user blocked for too long or for most of the time.
  //
  // This is crucial for safety to prevent the app user from accidently 
  // blocking himself outside of his account forever or most of the time.

  fn execute(self, app: &mut App) -> Self::Outcome {
    // Get the enforcers feature.
    let feature = &mut app.app_data.networking_access;

    // Get the enforcer.
    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.username == self.username) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    // Get the rule.
    let Some(rule) = enforcer
      .rules
      .iter_mut()
      .find(|rule| rule.id == self.rule_id) else 
    {
      return Err(Error::NoSuchRule);
    };

    let Activator::NotInTimeRange(time_range) = &mut rule.activator else {
      return Err(Error::WrongActivatorType);
    };

    if self.new_time_range.is_wider_than_or_equal_to(time_range) {
      return Err(Error::WouldMakeRuleLessRestrictive);
    }

    if let Err(_) = database_procedures::activator_not_in_time_range_replace(
      &app.database_connection,
      &self.rule_id,
      &self.new_time_range
    ) {
      return Err(Error::InternalError);
    }

    *time_range = self.new_time_range;
    Ok(())
  }
}
