use serde::{Deserialize, Serialize};
use crate::api::Context;
use super::*;

pub struct ManageUser {
  user_identification_method: UserIdentificationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManageUserReturn {
  NoSuchUser,
  AlreadyManaged,
  InternalError,
  Success,
  // Success(UserPublicRepr),
}

fn manage_user(
  context: &Context,
  operation: ManageUser,
  integration: &mut Data,
) 
  -> ManageUserReturn
{
  if integration.is_user_managed(&operation.user_identification_method) {
    return ManageUserReturn::AlreadyManaged;
  }

  let user_info = retrieve_user_info(operation.user_identification_method);
  let user_info = match user_info {
    RetrieveUserInfoReturn::Error => {
      return ManageUserReturn::InternalError;
    }
    RetrieveUserInfoReturn::NoSuchUser => {
      return ManageUserReturn::InternalError;
    }
    RetrieveUserInfoReturn::Success(user_info) => {
      user_info
    }
  };

  let user = UserInfo {
    user_id: user_info.user_id,
    user_name: user_info.user_name,
    user_password: user_info.user_password,
    user_screen_access_regulation: screen_access_regulation::Regulation::new(Vec::new()),
    user_screen_access_regulation_application: screen_access_regulation_application::UserScreenAccessRegulationApplicationData::default(),
  };

  if let Err(error) = db::add_user(&daemon.database, &user) {
    daemon.log_internal_error(error);
    return Outcome::InternalError;
  }

  daemon.state.users.push(user.clone());
  ManageUserReturn::Success(user.into_public())
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



