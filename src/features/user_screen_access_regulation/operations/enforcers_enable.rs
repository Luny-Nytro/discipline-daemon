use super::{
  Serialize, Deserialize, Daemon, IsOperation, Uuid,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
  EnforcerAlreadyEnabled,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  id: Uuid
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut Daemon) -> Self::Outcome {
    let feature = &mut app.state.user_access;
    let adapter = &app.schema.user_screen_access_regulation_common_info;

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.id == self.id) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    if enforcer.is_enforcing_enabled {
      return Err(Error::EnforcerAlreadyEnabled);
    }

    if let Err(_) = adapter.update_enforcer_is_enforcing_enabled(
      &app.database_connection, 
      &self.id,
      true,
    ) {
      return Err(Error::InternalError);
    }

    enforcer.is_enforcing_enabled = true;
    Ok(())
  }
}
