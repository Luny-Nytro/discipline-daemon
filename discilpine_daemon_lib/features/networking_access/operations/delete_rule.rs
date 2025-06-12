use serde::{Deserialize, Serialize};
use crate::{DateTime, IsOperation, OperatingSystemUsername, Uuid};
use crate::networking_access::database_procedures;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
  NoSuchEnforcer,
  NoSuchRule,
  RuleEnabled,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  username: OperatingSystemUsername,
  rule_id: Uuid,
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut crate::App) -> Self::Outcome {
    let feature = &mut app.app_data.networking_access;

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.username == self.username) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    let Some(index) = enforcer
      .rules
      .iter_mut()
      .position(|rule| rule.id == self.rule_id) else
    {
      return Err(Error::NoSuchRule);
    };

    let rule = &mut enforcer.rules[index];

    if rule.is_enabled(DateTime::now()) {
      return Err(Error::RuleEnabled);
    }

    if let Err(_) = database_procedures::delete_rule(
      &app.database_connection, 
      &self.rule_id
    ) {
      return Err(Error::InternalError);
    }

    enforcer.rules.remove(index);
    Ok(())
  }
}