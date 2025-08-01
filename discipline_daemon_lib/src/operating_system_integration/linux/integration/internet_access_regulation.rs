use serde::{Deserialize, Serialize};

use super::*;
use crate::logic::chronic::{DateTime, Duration};
use crate::logic::internet_access_regulation::Action;
use crate::Daemon;
use std::sync::Arc;

// TODO: Allow the user to pick from various methods to allow and block internet access such as
// - enabling and disabling NetworkManager using systemctl's terminal commands
// - enabling and disabling NetworkManager using systemctl's D-Bus api
// - using iptables
// - using nftables's terminal commands
// - using nftables's JSON api

static DEFAULT_APPLICATION_INTERVAL: Duration = Duration::from_minutes(5).unwrap();

pub struct CrossUserInfo {}

impl CrossUserInfo {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationStatus {
  Unknown,
  Blocked,
  Allowed,
}

#[derive(Debug, Clone)]
pub struct UserSpecificInfo {
  application_status: ApplicationStatus,
  application_interval: Duration,
  pub application_enabled: bool,
}

impl UserSpecificInfo {
  pub fn new() -> Self {
    Self {
      application_status: ApplicationStatus::Unknown,
      application_interval: DEFAULT_APPLICATION_INTERVAL,
      application_enabled: false,
    }
  }

  pub fn from_fields(
    status: ApplicationStatus,
    application_interval: Duration,
    is_application_enabled: bool,
  ) -> Self {
    Self {
      application_status: status,
      application_interval,
      application_enabled: is_application_enabled,
    }
  }

  pub fn application_status(&self) -> ApplicationStatus {
    self.application_status
  }

  pub fn application_interval(&self) -> Duration {
    self.application_interval
  }

  pub fn application_enabled(&self) -> bool {
    self.application_enabled
  }
}

pub enum AsyncTask {
  ApplyRegulationForUser(UserId),
  UpdateAfterAllowingInternetAccessForUser(UserId),
  UpdateAfterBlockingInternetAccessForUser(UserId),
}

impl Into<TopAsyncTask> for AsyncTask {
  fn into(self) -> TopAsyncTask {
    TopAsyncTask::InternetAccessRegulation(self)
  }
}

fn schedule_apply_regulation_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
  delay: Duration,
) {
  scheduler.add_delayed_operation(AsyncTask::ApplyRegulationForUser(user_id), delay);
}

fn schedule_update_after_allowing_internet_access_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
) {
  scheduler.add_immediate_operation(AsyncTask::UpdateAfterAllowingInternetAccessForUser(user_id));
}

fn schedule_update_after_blocking_internet_access_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
) {
  scheduler.add_immediate_operation(AsyncTask::UpdateAfterBlockingInternetAccessForUser(user_id));
}

fn execute_apply_regulation_for_user(
  user_id: UserId,
  daemon: Arc<Daemon>,
) {
  let (action, user_id, user_name, application_status, application_interval) = {
    let Ok(mut integration) = daemon.operating_system_integration().lock_data() else {
      schedule_apply_regulation_for_user(
        &daemon.operating_system_integration().async_scheduler(), 
        user_id, 
        DEFAULT_APPLICATION_INTERVAL,
      );
      return;
    };

    let Some(user) = integration.users.get_mut(&user_id) else {
      // User is no longer managed. Don't schedule more async tasks for it.
      return;
    };

    let now = DateTime::now();
    (
      user
        .user_internet_access_regulation_logic
        .calculate_action(now),
      user.user_id,
      user.user_name.clone(),
      user
        .user_internet_access_regulation_integration
        .application_status,
      user
        .user_internet_access_regulation_integration
        .application_interval,
    )
  };

  match action {
    Action::Allow => {
      allow_internet_access_for_user(
        &daemon.operating_system_integration().async_scheduler(),
        user_id,
        user_name,
        application_status,
        application_interval,
      );
    }
    Action::Block => {
      block_internet_access_for_user(
        &daemon.operating_system_integration().async_scheduler(),
        user_id,
        user_name,
        application_status,
        application_interval,
      );
    }
  }
}

fn allow_internet_access_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
  user_name: UserName,
  application_status: ApplicationStatus,
  application_interval: Duration,
) {
  if application_status == ApplicationStatus::Allowed {
    schedule_apply_regulation_for_user(scheduler, user_id, application_interval);
    return;
  }

  match allow_inbound_network_traffic_for_user(&user_id, &user_name) {
    Ok(_) => {
      schedule_update_after_allowing_internet_access_for_user(scheduler, user_id);
    }
    Err(_) => {
      // The default behavior now is just to keep trying.
      // TODO: Find a better way to handl this situation.
      // TODO: Log the error somewhere to:
      //       - Let the user know an error occured while allowing internet access
      //       - Let the trubleshooter why exactly the error occured.

      schedule_apply_regulation_for_user(scheduler, user_id, application_interval);
    }
  }
}

fn block_internet_access_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
  user_name: UserName,
  application_status: ApplicationStatus,
  application_interval: Duration,
) {
  if application_status == ApplicationStatus::Blocked {
    schedule_apply_regulation_for_user(scheduler, user_id, application_interval);
    return;
  }

  match allow_inbound_network_traffic_for_user(&user_id, &user_name) {
    Ok(_) => {
      schedule_update_after_blocking_internet_access_for_user(scheduler, user_id);
    }
    Err(_) => {
      // The default behavior now is just to keep trying.
      // TODO: Find a better way to handl this situation.
      // TODO: Log the error somewhere to:
      //       - Let the user know an error occured while allowing internet access
      //       - Let the trubleshooter why exactly the error occured.

      schedule_apply_regulation_for_user(scheduler, user_id, application_interval);
    }
  }
}

fn execute_update_after_allowing_internet_access_for_user(
  user_id: UserId,
  daemon: Arc<Daemon>,
) {
  let Ok(mut integration) = daemon.operating_system_integration().lock_data() else {
    // TODO: Handle this error case.
    return;
  };

  let Some(user) = integration.users.get_mut(&user_id) else {
    // User is no longer managed. Don't schedule more async tasks for it.
    return;
  };

  user
    .user_internet_access_regulation_integration
    .application_status 
    = ApplicationStatus::Allowed;

  schedule_apply_regulation_for_user(
    &daemon.operating_system_integration().async_scheduler(), 
    user_id, 
    user.user_internet_access_regulation_integration.application_interval,
  );
}

fn execute_update_after_blocking_internet_access_for_user(
  user_id: UserId,
  daemon: Arc<Daemon>,
) {
  let Ok(mut integration) = daemon.operating_system_integration().lock_data() else {
    // TODO: Handle this error case.
    return;
  };

  let Some(user) = integration.users.get_mut(&user_id) else {
    // User is no longer managed. Don't schedule more async tasks for it.
    return;
  };

  user
    .user_internet_access_regulation_integration
    .application_status 
    = ApplicationStatus::Blocked;

  schedule_apply_regulation_for_user(
    &daemon.operating_system_integration().async_scheduler(), 
    user_id, 
    user.user_internet_access_regulation_integration.application_interval,
  );
}

impl AsyncTask {
  pub fn execute(
    self,
    daemon: Arc<Daemon>
  ) {
    match self {
      AsyncTask::ApplyRegulationForUser(user_id) => {
        execute_apply_regulation_for_user(user_id, daemon);
      }
      AsyncTask::UpdateAfterAllowingInternetAccessForUser(user_id) => {
        execute_update_after_allowing_internet_access_for_user(user_id, daemon);
      }
      AsyncTask::UpdateAfterBlockingInternetAccessForUser(user_id) => {
        execute_update_after_blocking_internet_access_for_user(user_id, daemon);
      }
    }
  }
}
