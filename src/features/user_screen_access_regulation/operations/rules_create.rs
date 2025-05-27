use super::{
  Serialize, Deserialize, Daemon, IsOperation, Uuid,
  Rule, RuleCreator, Regulator
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
  RuleCreationLimitReached,
  IdUnavailable,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  enforcer_id: Uuid,
  rule_creator: RuleCreator
}

impl IsOperation for Operation {
  type Outcome = Result<Rule, Error>;

  fn execute(self, app: &mut Daemon) -> Self::Outcome {
    let feature = &mut app.state.user_access;
    let adapter = &app.schema.user_screen_access_regulation_common_info;

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.id == self.enforcer_id) else 
    {
      return Err(Error::NoSuchEnforcer);
    };
    
    if enforcer.rules.len() >= Regulator::MAX_POLICIES {
      return Err(Error::RuleCreationLimitReached);
    }

    if let Some(rule_id) = self.rule_creator.id {
      if enforcer.rules.iter().any(|rule| rule.id == rule_id) {
        return Err(Error::IdUnavailable);
      }
    }

    let rule = self.rule_creator.create();
    if let Err(_) = adapter.create_rule(
      &app.database_connection, 
      &rule, 
      enforcer.policies_number(),
      &enforcer.id, 
    ) {
      return Err(Error::InternalError);
    }

    enforcer.rules.push(rule.clone());
    Ok(rule)
  }
}
