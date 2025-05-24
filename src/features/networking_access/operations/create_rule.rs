use serde::{Serialize, Deserialize};
use crate::{App, DateTime, OperatingSystemUsername, IsOperation, ToPublicRepr};
use crate::networking_access::{RulePublicRepr, RuleCreator, database_procedures};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
  RuleCreationLimitReached,
  IdUnavailable,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  username: OperatingSystemUsername,
  rule_creator: RuleCreator
}

impl IsOperation for Operation {
  type Outcome = Result<RulePublicRepr, Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.app_data.networking_access;

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.username == self.username) else 
    {
      return Err(Error::NoSuchEnforcer);
    };
    
    if enforcer.rules.len() >= 30 {
      return Err(Error::RuleCreationLimitReached);
    }

    if let Some(rule_id) = self.rule_creator.id {
      if enforcer.rules.iter().any(|rule| rule.id == rule_id) {
        return Err(Error::IdUnavailable);
      }
    }

    let now = DateTime::now();
    let mut new_rule = self.rule_creator.create(now);
    let username = &enforcer.username;
    let position = enforcer.rules.len();

    if let Err(_) = database_procedures::create_rule(
      &app.database_connection, 
      &new_rule, 
      username, 
      position,
    ) {
      return Err(Error::InternalError);
    }

    let rule_public_repr = new_rule.to_public_repr();
    enforcer.rules.push(new_rule);
    Ok(rule_public_repr)
  }
}
