use super::*;

pub(super) struct UserScreenAccessData {
  status: UserScreenAccessStatus
}

pub enum UserScreenAccessStatus {
  Unknown,
  LoginAllowed,
  LoginBlocked,
  LoginBlockedAndSessionTerminated,
}

pub enum UserScreenAccessRegulationAction {
  Allow,
  Block,
}

pub trait IsUserScreenAccessRegulation: Send {
  fn action(&self, operating_system_user_id: OperatingSystemUserId) -> UserScreenAccessRegulationAction;
}


fn block_user_screen_access(
  user: &mut OperatingSystemUser,
  blocked_user_password: &OperatingSystemUserPassword,
  // integration: &mut OperatingSystemIntegrationInternal<T>,
  // operating_system_user_id: OperatingSystemUserId,
) {
  // let Some(user) = integration
  //   .operating_system_users
  //   .iter_mut()
  //   .find(|user| user.operating_system_user_id == operating_system_user_id) else 
  // {
  //   // TODO: Log an error.
  //   return;
  // };

  // match user.screen_access_status {
  //   UserScreenAccessStatus::LoginAllowed => {
  //     if let Err(error) = change_user_password(
  //       &user.operating_system_user_name, 
  //       &blocked_user_password,
  //     ) {
  //       // TODO: Log the error somewhere
  //       return;
  //     }

  //     user.screen_access_status = UserScreenAccessStatus::LoginBlocked;

  //     if let Err(error) = terminate_user_session(&user.operating_system_user_name) {
  //       // TODO: Log the error somewhere
  //       return;
  //     }

  //     // TODO: Update the database too
  //     user.screen_access_status = UserScreenAccessStatus::LoginBlockedAndSessionTerminated;
  //   }
  //   UserScreenAccessStatus::LoginBlocked => {

  //     if let Err(error) = terminate_user_session(&user.operating_system_user_name) {
  //       // TODO: Log the error somewhere
  //       return;
  //     }

  //     // TODO: Update the database too
  //     user.screen_access_status = UserScreenAccessStatus::LoginBlockedAndSessionTerminated;
  //   }
  //   UserScreenAccessStatus::LoginBlockedAndSessionTerminated => {
  //     // noop
  //   }
  // }
}

fn allow_user_screen_access(
  integration: &mut OperationScheduler,
  // operating_system_user_id: OperatingSystemUserId,
) {
  // let Some(user) = integration
  //   .operating_system_users
  //   .iter_mut()
  //   .find(|user| user.operating_system_user_id == operating_system_user_id) else {
  //   return;
  // };

  // match user.screen_access_status {
  //   UserScreenAccessStatus::LoginAllowed => {
  //     // noop
  //   }
  //   UserScreenAccessStatus::LoginBlocked | 
  //   UserScreenAccessStatus::LoginBlockedAndSessionTerminated => {
  //     if let Err(error) = change_user_password(
  //       &user.operating_system_user_name, 
  //       &user.operating_system_user_password,
  //     ) {
  //       // TODO: Log the error somewhere
  //       return;
  //     }

  //     user.screen_access_status = UserScreenAccessStatus::LoginAllowed;
  //   }
  // }
}

pub(super) fn user_screen_access_task(
  integration: &mut OperatingSystemIntegrationData
) -> Duration {
  todo!()
// let action = integration
//             .user_screen_access_regulation
//             .action(user.operating_system_user_id);

//           match action {
//             UserScreenAccessRegulationAction::Allow => {
//               allow_user_screen_access(user);
//             }
//             UserScreenAccessRegulationAction::Block => {
//               block_user_screen_access(user, &integration.blocked_user_password);
//             }
//           }
}