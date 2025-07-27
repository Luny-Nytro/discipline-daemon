use super::*;
use crate::features::screen_access_regulation::Action;
use std::sync::{Arc, Mutex};

// TODO: Allow the user to pick from various methods to allow and block internet access such as
// - enabling and disabling NetworkManager using systemctl's terminal commands
// - enabling and disabling NetworkManager using systemctl's D-Bus api
// - using iptables
// - using nftables's terminal commands
// - using nftables's JSON api

pub struct CommonInternetAccessRegulationApplicationData {}

impl CommonInternetAccessRegulationApplicationData {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserInternetAccessRegulationApplicationStatus {
  Unknown,
  Blocked,
  Allowed,
}

#[derive(Debug, Clone)]
pub struct UserInternetAccessRegulationApplicationData {
  checking_interval: Duration,
  is_internet_regulation_application_enabled: bool,
  status: UserInternetAccessRegulationApplicationStatus,
}

impl UserInternetAccessRegulationApplicationData {
  pub fn new() -> Self {
    Self {
      checking_interval: Duration::from_minutes(5).unwrap(),
      is_internet_regulation_application_enabled: false,
      status: UserInternetAccessRegulationApplicationStatus::Unknown,
    }
  }

  pub fn from_fields(
    checking_interval: Duration,
    is_internet_regulation_application_enabled: bool,
    user_internet_access_regulation_application_status: UserInternetAccessRegulationApplicationStatus,
  ) -> Self {
    Self {
      checking_interval,
      is_internet_regulation_application_enabled,
      status: user_internet_access_regulation_application_status,
    }
  }

  pub fn checking_interval(&self) -> Duration {
    self.checking_interval
  }

  pub fn is_internet_regulation_application_enabled(&self) -> bool {
    self.is_internet_regulation_application_enabled
  }

  pub fn user_internet_access_regulation_application_status(
    &self,
  ) -> UserInternetAccessRegulationApplicationStatus {
    self.status
  }
}

pub struct InternetAccessRegulationAsyncTask {
  user_id: UserId,
}

impl Into<AsyncOperation> for InternetAccessRegulationAsyncTask {
  fn into(self) -> AsyncOperation {
    AsyncOperation::InternetAccessRegulationApplication(self)
  }
}

fn schedule_delayed(
  scheduler: &OperationScheduler,
  user_id: UserId,
  user_internet_access_regulation_application_checking_interval: Duration,
) {
  scheduler.add_delayed_operation(
    InternetAccessRegulationAsyncTask { user_id },
    user_internet_access_regulation_application_checking_interval.as_standard_duration(),
  );
}

fn schedule_immediate(scheduler: &OperationScheduler, user_id: UserId) {
  scheduler.add_immediate_operation(InternetAccessRegulationAsyncTask { user_id });
}

impl InternetAccessRegulationAsyncTask {
  pub fn execute(
    self,
    scheduler: Arc<OperationScheduler>,
    integration: Arc<Mutex<OperatingSystemIntegrationData>>,
  ) {
  }
}

fn execute_apply_regulation(
  user_id: UserId,
  scheduler: Arc<OperationScheduler>,
  integration: Arc<Mutex<OperatingSystemIntegrationData>>,
) {
  let (
    action,
    user_id,
    user_name,
    user_internet_access_regulation_application_status,
    user_internet_access_regulation_application_checking_interval,
  ) = {
    let mut integration = integration.lock().unwrap();

    let Some(user) = integration.users.get_mut(&user_id) else {
      // User was deleted. Don't schedule more operations for it.
      return;
    };

    let now = DateTime::now();
    (
      user.user_screen_access_regulation.calculate_action(now),
      user.user_id,
      user.user_name.clone(),
      user.user_internet_access_regulation_application.status,
      user
        .user_internet_access_regulation_application
        .checking_interval,
    )
  };

  match action {
    Action::Allow => {
      allow_internet_access_for_user(
        &*scheduler,
        user_id,
        user_name,
        user_internet_access_regulation_application_status,
        user_internet_access_regulation_application_checking_interval,
      );
    }
    Action::Block => {
      block_internet_access_for_user(
        &*scheduler,
        user_id,
        user_name,
        user_internet_access_regulation_application_status,
        user_internet_access_regulation_application_checking_interval,
      );
    }
  }
}

fn allow_internet_access_for_user(
  scheduler: &OperationScheduler,
  user_id: UserId,
  user_name: UserName,
  user_internet_access_regulation_application_status: UserInternetAccessRegulationApplicationStatus,
  user_internet_access_regulation_application_checking_interval: Duration,
) {
  if user_internet_access_regulation_application_status
    == UserInternetAccessRegulationApplicationStatus::Allowed
  {
    schedule_delayed(
      &*scheduler,
      user_id,
      user_internet_access_regulation_application_checking_interval,
    );
    return;
  }

  match allow_inbound_network_traffic_for_user(&user_id, &user_name) {
    Ok(_) => {
      // schedule_immediate_save_updates_after_allowing_internet_access()
    }
    Err(_) => {
      // schedule_delayed_retry_allow_internet_access_and_log_error()
    }
  }
}

fn block_internet_access_for_user(
  scheduler: &OperationScheduler,
  user_id: UserId,
  user_name: UserName,
  user_internet_access_regulation_application_status: UserInternetAccessRegulationApplicationStatus,
  user_internet_access_regulation_application_checking_interval: Duration,
) {
  match allow_inbound_network_traffic_for_user(&user_id, &user_name) {
    Ok(_) => {
      // schedule_immediate_update_after_blocking_user_internet_access()
    }
    Err(_) => {
      // schedule_delayed_retry_block_user_internet_access()
    }
  }
}

// let mut integration = integration.lock().unwrap();

// let Some(user) = integration.users.get_mut(&user_id) else {
//   return;
// };

// match maybe_error {
//   Ok(_) => {
//     // TODO: Update the database, too.
//     user.user_screen_access_regulation_application.status =
//       UserInternetAccessRegulationApplicationStatus::LoginBlocked;
//     // drop(user);
//     drop(integration);
//     scheduler
//       .add_immediate_operation(InternetAccessRegulationAsyncTask::TerminateSession(user_id));
//   }
//   Err(_) => {
//     // TODO: Log the error.

//     // The default action is to keep trying with an interval.
//     // TODO: Handle this situation better.
//     let interval = user
//       .user_screen_access_regulation_application
//       .checking_interval
//       .as_standard_duration();
//     drop(integration);
//     scheduler.add_delayed_operation(
//       InternetAccessRegulationAsyncTask::BlockInternetAccess(user_id),
//       interval,
//     );
//   }
// }

// let mut integration = integration.lock().unwrap();

// let Some(user) = integration.users.get_mut(&user_id) else {
//   return;
// };

// match maybe_error {
//   Ok(_) => {
//     // TODO: Update the database, too.
//     schedule_delayed(&*scheduler, user);
//     user
//       .user_internet_access_regulation_application
//       .user_internet_access_regulation_application_status =
//       UserInternetAccessRegulationApplicationStatus::Allowed;
//   }
//   Err(_) => {
//     // TODO: Log the error.

//     // The default action is to keep trying with an interval.
//     // TODO: Handle this situation better.
//     scheduler.add_delayed_operation(
//       InternetAccessRegulationAsyncTask::AllowInternetAccess(user_id),
//       user
//         .user_screen_access_regulation_application
//         .checking_interval
//         .as_standard_duration(),
//     );
//   }
// }
