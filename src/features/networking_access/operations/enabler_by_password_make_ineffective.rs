use serde::{Serialize, Deserialize};
use crate::{App, OperatingSystemUsername, IsOperation, Password, Uuid};
use crate::networking_access::{database_procedures, Enabler};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
  NoSuchRule,
  WrongEnablerType,
  AlreadyIneffective,
  WrongPassword,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  username: OperatingSystemUsername,
  password: Password,
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.app_data.networking_access;

    let Some(enforcer) = &mut feature
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

    let Enabler::ByPassword(enabler) = &mut rule.enabler else {
      return Err(Error::WrongEnablerType);
    };

    if !enabler.is_effective() {
      return Err(Error::AlreadyIneffective);
    }

    if !enabler.is_right_password(&self.password) {
      return Err(Error::WrongPassword);
    }

    if let Err(_) = database_procedures::enabler_by_password_make_ineffective(
      &app.database_connection, 
      &self.rule_id,
    ) {
      return Err(Error::InternalError);
    }

    enabler.make_ineffective();
    Ok(())
  }
}
