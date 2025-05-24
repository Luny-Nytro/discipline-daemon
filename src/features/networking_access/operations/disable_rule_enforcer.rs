use serde::{Serialize, Deserialize};
use crate::{App, DateTime, IsOperation, OperatingSystemUsername};
use crate::networking_access::database_procedures;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
  EnforcerAlreadyDisabled,
  SomeRulesAreEnabled,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  username: OperatingSystemUsername
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

    if !enforcer.is_enabled {
      return Err(Error::EnforcerAlreadyDisabled);
    }

    let now = DateTime::now();
    if enforcer.are_some_rules_enabled(now) {
      return Err(Error::SomeRulesAreEnabled);
    }

    if let Err(_) = database_procedures::disable_enforcer(
      &app.database_connection, 
      &self.username,
    ) {
      return Err(Error::InternalError);
    }

    enforcer.is_enabled = false;
    Ok(())
  }
}

