use super::{
  Serialize, Deserialize, IsOperation, Uuid
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
  NoSuchEnforcer,
  NoSuchRule,
  RuleIsProtected,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  enforcer_id: Uuid,
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut crate::Daemon) -> Self::Outcome {
    let feature = &mut app.state.user_access;
    let adapter = &app.schema.user_screen_access_regulation_common_info;
    let synchronize_context = app.synchronize_source.create_context_for_now();

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.id == self.enforcer_id) else 
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
    if rule.is_protected(&synchronize_context) {
      return Err(Error::RuleIsProtected);
    }

    if let Err(_) = adapter.delete_rule(
      &app.database_connection, 
      &self.rule_id
    ) {
      return Err(Error::InternalError);
    }

    enforcer.rules.remove(index);
    Ok(())
  }
}