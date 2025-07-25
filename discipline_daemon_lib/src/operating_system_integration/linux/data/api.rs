use serde::{Deserialize, Serialize};
use crate::api::Context;
use super::*;

pub struct AddToManagedUsers {
  user_identification_method: UserIdentificationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddToManagedUsersReturn {
  NoSuchUser,
  AlreadyAdded,
  InternalError,
  Success,
  // Success(UserPublicRepr),
}

fn manage_user(
  context: &Context,
  operation: AddToManagedUsers,
  integration: &mut Data,
) 
  -> AddToManagedUsersReturn
{
  if integration.is_user_managed(&operation.user_identification_method) {
    return AddToManagedUsersReturn::AlreadyAdded;
  }

  let user_info = retrieve_user_info(operation.user_identification_method);
  let (user_id, user_name, user_password) = match user_info {
    RetrieveUserInfoReturn::Error => {
      return AddToManagedUsersReturn::InternalError;
    }
    RetrieveUserInfoReturn::NoSuchUser => {
      return AddToManagedUsersReturn::InternalError;
    }
    RetrieveUserInfoReturn::Success { user_id, user_name, user_password } => {
      
    }
  };

  let operating_system_user_id = match OperatingSystemUserId::from_username(&self.operating_system_user_name) {
    Ok(value) => {
      value
    }
    Err(error) => {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }
  };


  if daemon.state.users.iter()
    .any(|user| user.operating_system_user_id == operating_system_user_id)
  {
    return Outcome::OperatingSystemUserWithGivenIdIsAlreadyManaged;
  }

  let user = User {
    id: self.user_id.unwrap_or_else(Uuid::new_v4),
    name: self.user_name,
    operating_system_user_id: operating_system_user_id,
    operating_system_user_name: self.operating_system_user_name,
    operating_system_user_password: self.operating_system_user_password,
    screen_access_regulation: user_screen_access_regulation::Regulation::new(Vec::new()),
  };

  if let Err(error) = db::add_user(&daemon.database, &user) {
    daemon.log_internal_error(error);
    return Outcome::InternalError;
  }

  daemon.state.users.push(user.clone());
  Outcome::Success(user.into_public())
}

pub enum Operation {
  AddUserGivenId { id: OperatingSystemUserId },
  AddUserGivenName { name: OperatingSystemUserName },
  DeleteUserGivenId { id: OperatingSystemUserId },
  DeleteUserGivenName { id: OperatingSystemUserName },
}

pub(super) fn api_task(
  integration: &mut Data,
  operations: &mut Vec<ApiOperation>
) {

    // let operations = take(&mut *operations.lock().unwrap());
    // for operation in operations {
    //   match operation {
    //     ApiOperation::AddUser(id, name, password, interval) => {
    //       integration.operating_system_users.push(OperatingSystemUser {
    //         screen_access_status: UserScreenAccessStatus::LoginAllowed,
    //         operating_system_user_id: id,
    //         operating_system_user_name: name,
    //         operating_system_user_password: password,
    //         screen_access_regulation_enforcing_interval: interval,
    //       });
    //     }
    //     ApiOperation::DeleteUser(id) => {

    //     }
    //   }
}



