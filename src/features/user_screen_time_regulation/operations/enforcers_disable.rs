use super::{
  Serialize, Deserialize, App, IsOperation, Uuid
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
  EnforcerAlreadyDisabled,
  SomeRulesAreEnabled,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  id: Uuid
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.state.user_access;
    let adapter = &app.state_database_adapter.user_access;
    let synchronize_context = app.synchronize_source.create_context_for_now();

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.id == self.id) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    if !enforcer.is_enforcing_enabled {
      return Err(Error::EnforcerAlreadyDisabled);
    }

    if enforcer.may_be_deleted(&synchronize_context) {
      return Err(Error::SomeRulesAreEnabled);
    }

    if let Err(_) = adapter.update_enforcer_is_enforcing_enabled(
      &app.database_connection, 
      &self.id,
      false,
    ) {
      return Err(Error::InternalError);
    }

    enforcer.is_enforcing_enabled = false;
    Ok(())
  }
}
