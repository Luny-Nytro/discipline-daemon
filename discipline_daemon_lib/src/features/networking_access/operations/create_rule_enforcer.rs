use serde::{Serialize, Deserialize};
use crate::{App, OperatingSystemUsername, IsOperation, ToPublicRepr};
use crate::networking_access::{OperatingSystemCalls, EnforcerPublicRepr, Enforcer, database_procedures};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
  EnforcerCreationLimitReached,
  EnforcerAlreadyCreatedForUser,
  InternalErrorOrNoSuchUser,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  username: OperatingSystemUsername
}

impl IsOperation for Operation {
  type Outcome = Result<EnforcerPublicRepr, Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.app_data.networking_access;

    if feature.len() >= 10 {
      return Err(Error::EnforcerCreationLimitReached);
    }

    if feature.is_enforcer_created_for_user(&self.username) {
      return Err(Error::EnforcerAlreadyCreatedForUser);
    }

    let user_id = match OperatingSystemCalls::get_user_id(&self.username) {
      Ok(value) => {
        value
      }
      Err(_) => {
        return Err(Error::InternalErrorOrNoSuchUser);
      }
    };

    let mut new_enforcer = Enforcer::new(user_id, self.username);

    if let Err(_) = database_procedures::create_enforcer(
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