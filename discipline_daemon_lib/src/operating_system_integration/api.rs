use super::*;

pub enum AnyOperatingSystemUserIdentificationMethod {
  Id(OperatingSystemUserId),
  Name(OperatingSystemUserName),
}

pub enum ApiOperation {
  AddUserGivenId { id: OperatingSystemUserId },
  AddUserGivenName { name: OperatingSystemUserName },
  DeleteUserGivenId { id: OperatingSystemUserId },
  DeleteUserGivenName { id: OperatingSystemUserName },
}

pub(super) fn api_task(
  integration: &mut OperatingSystemIntegrationData,
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