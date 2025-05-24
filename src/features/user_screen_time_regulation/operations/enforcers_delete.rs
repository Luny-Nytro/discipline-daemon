use super::{
  Serialize, Deserialize, Uuid, IsOperation, App
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  NoSuchEnforcer,
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

    let Some(position) = feature
      .enforcers
      .iter()
      .position(|enforcer| enforcer.id == self.id) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    let enforcer = &mut feature.enforcers[position];
    if enforcer.may_be_deleted(&synchronize_context) {
      return Err(Error::SomeRulesAreEnabled);
    }

    if let Err(_) = adapter.delete_enforcer(
      &app.database_connection, 
      enforcer,
    ) {
      return Err(Error::InternalError);
    }
    
    feature.enforcers.remove(position);
    Ok(())
  }
}

