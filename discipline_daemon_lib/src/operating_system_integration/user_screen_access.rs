use std::sync::Arc;
use crate::features::screen_access_regulation::Action;
use super::*;

// TODO: If terminating a user session takes too long, just kill the session.
// TODO: Reduce lock duration in "block_user_screen_access"
// TODO: Reduce lock duration in "allow_user_screen_access"
// TODO: Allow the user to select from various session termination methods
//       such as "pkill", "loginctl terminate-session" or just covering the screen after login in.

pub struct CommonScreenAccessRegulationApplicationData {
  private_password: OperatingSystemUserPassword,
}

impl CommonScreenAccessRegulationApplicationData {
  pub fn new() -> Self {
    Self {
      private_password: OperatingSystemUserPassword::generate_random_password()
    }
  }
}

pub struct UserScreenAccessRegulationApplicationData {
  status: UserScreenAccessRegulationApplictionStatus,
  interval: Duration,
}

pub enum UserScreenAccessRegulationApplictionStatus {
  Unknown,
  LoginAllowed,
  LoginBlocked,
  LoginBlockedAndSessionTerminated,
}

pub enum ScreenAccessRegulationScheduledTask {
  CheckAll,
  RecheckOne(OperatingSystemUserId),
  Allow(OperatingSystemUserId),
  BlockLogin(OperatingSystemUserId),
  TerminateSession(OperatingSystemUserId),
}

impl Into<ScheduledTask> for ScreenAccessRegulationScheduledTask {
  fn into(self) -> ScheduledTask {
    ScheduledTask::ScreenAccessRegulationApplication(self)
  }
}

impl ScreenAccessRegulationScheduledTask {
  pub fn execute(
    self, 
    scheduler: Arc<OperationScheduler>,
    integration: &mut IntegrationData,
  ) {
    match self {
      Self::CheckAll => {

      }
    }
  }
}

fn execute_check_all(
  scheduler: Arc<OperationScheduler>,
  integration: &mut IntegrationData,
) {
  let now = DateTime::now();
  for user in integration.users.values_mut() {
    let action = user.user_screen_access_regulation.calculate_action(now);

    match user.user_screen_access_regulation_application.status {
      UserScreenAccessRegulationApplictionStatus::Unknown => {
        match action {
          Action::Allow => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationScheduledTask::Allow(user.user_id)
            );
          }
          Action::Block => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationScheduledTask::BlockLogin(user.user_id)
            );
          }
        }
      }
      UserScreenAccessRegulationApplictionStatus::LoginAllowed => {
        match action {
          Action::Allow => {
            scheduler.add_delayed_operation(
              ScreenAccessRegulationScheduledTask::RecheckOne(user.user_id), 
              user.user_screen_access_regulation_application.interval.as_standard_duration(),
            );
          }
          Action::Block => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationScheduledTask::BlockLogin(user.user_id)
            );
          }
        }
      }
      UserScreenAccessRegulationApplictionStatus::LoginBlocked => {
        match action {
          Action::Allow => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationScheduledTask::Allow(user.user_id)
            );
          }
          Action::Block => {
            scheduler.add_delayed_operation(
              ScreenAccessRegulationScheduledTask::RecheckOne(user.user_id), 
              user.user_screen_access_regulation_application.interval.as_standard_duration(),
            );
          }
        }
      }
      UserScreenAccessRegulationApplictionStatus::LoginBlockedAndSessionTerminated => {
        match action {
          Action::Allow => {
            scheduler
          }
        }
      }
    }
  }
}

fn block_user_screen_access(
  user: &mut UserInfo,
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
  integration: &mut IntegrationData
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