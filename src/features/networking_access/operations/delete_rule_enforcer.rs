use serde::{Serialize, Deserialize};
use crate::{App, DateTime, IsOperation, OperatingSystemUsername};
use crate::networking_access::database_procedures;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
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

    let Some(position) = feature
      .enforcers
      .iter()
      .position(|enforcer| enforcer.username == self.username) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    let now = DateTime::now();
    let enforcer = &mut feature.enforcers[position];
    if enforcer.are_some_rules_enabled(now) {
      return Err(Error::SomeRulesAreEnabled);
    }

    if let Err(_) = database_procedures::delete_enforcer(
      &app.database_connection, 
      enforcer,
    ) {
      return Err(Error::InternalError);
    }
    
    feature.enforcers.remove(position);
    Ok(())  
  }
}
