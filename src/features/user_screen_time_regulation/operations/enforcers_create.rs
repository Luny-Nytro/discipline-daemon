use super::{
  Serialize, Deserialize, RegulatorCreator, App, 
  OperatingSystemUserId, IsOperation, ToPublicRepr,
  CommonInfo, RegulatorPublicRepr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
  IdUnavailable,
  EnforcerCreationLimitReached,
  EnforcerAlreadyCreatedForUser,
  InternalErrorOrNoSuchUser,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  creator: RegulatorCreator
}

impl IsOperation for Operation {
  type Outcome = Result<RegulatorPublicRepr, Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.state.user_access;
    let adapter = &app.state_database_adapter.user_access;

    if feature
      .enforcers
      .len() >= CommonInfo::MAX_ENFORCERS
    {
      return Err(Error::EnforcerCreationLimitReached);
    }

    if feature
      .enforcers
      .iter()
      .any(|enforcer| enforcer.username == self.creator.username) 
    {
      return Err(Error::EnforcerAlreadyCreatedForUser);
    }

    let user_id = match OperatingSystemUserId::from_username(&self.creator.username) {
      Ok(value) => {
        value
      }
      Err(_) => {
        return Err(Error::InternalErrorOrNoSuchUser);
      }
    };

    let mut new_enforcer = self.creator.create(user_id);

    if feature
      .enforcers
      .iter()
      .any(|enforcer| enforcer.id == new_enforcer.id)
    {
      return Err(Error::IdUnavailable);
    }
  
    if let Err(_) = adapter.create_enforcer(
      &app.database_connection, 
      &new_enforcer,
    ) {
      return Err(Error::InternalError);
    }

    let enforcer_public_repr = new_enforcer.to_public_repr();
    feature.enforcers.push(new_enforcer);
    Ok(enforcer_public_repr)
  }
}
