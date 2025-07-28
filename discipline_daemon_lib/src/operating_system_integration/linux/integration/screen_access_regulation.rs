use super::*;
use crate::logic::chronic::{DateTime, Duration};
use crate::logic::screen_access_regulation::Action;
use crate::database::operating_system_integration_linux_user as db;
use crate::Daemon;
use std::sync::Arc;

// TODO: If terminating a user session takes too long, just kill the session.
// TODO: Reduce lock duration in "block_user_screen_access"
// TODO: Reduce lock duration in "allow_user_screen_access"
// TODO: Allow the user to select from various session termination methods
//       such as "pkill", "loginctl terminate-session" or just covering the screen after login in.

static DEFAULT_APPLICATION_INTERVAL: Duration = Duration::from_minutes(5).unwrap();

pub struct CrossUserInfo {
  blocked_state_password: UserPassword,
}

impl CrossUserInfo {
  pub fn new() -> Self {
    Self {
      blocked_state_password: UserPassword::generate_random_password(),
    }
  }

  pub fn from_fields(blocked_state_password: UserPassword) -> Self {
    Self {
      blocked_state_password,
    }
  }

  pub fn blocked_state_password(&self) -> &UserPassword {
    &self.blocked_state_password
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationStatus {
  Unknown,
  Allowed,
  LoginBlocked,
  LoginBlockedAndSessionTerminated,
}

#[derive(Debug, Clone)]
pub struct UserSpecificInfo {
  pub(super) application_status: ApplicationStatus,
  pub(super) application_enabled: bool,
  pub(super) application_interval: Duration,
}

impl UserSpecificInfo {
  pub fn new() -> Self {
    Self {
      application_status: ApplicationStatus::Unknown,
      application_enabled: false,
      application_interval: DEFAULT_APPLICATION_INTERVAL,
    }
  }

  pub fn from_fields(
    application_status: ApplicationStatus,
    application_enabled: bool,
    application_interval: Duration,
  ) -> Self {
    Self {
      application_status,
      application_enabled,
      application_interval,
    }
  }

  pub fn enabled(&self) -> bool {
    self.application_enabled
  }

  pub fn login_blocked(&self) -> bool {
    match self.application_status {
      ApplicationStatus::Unknown => false,
      ApplicationStatus::Allowed => false,
      ApplicationStatus::LoginBlocked => true,
      ApplicationStatus::LoginBlockedAndSessionTerminated => true,
    }
  }

  pub fn application_status(&self) -> ApplicationStatus {
    self.application_status
  }

  pub fn application_interval(&self) -> Duration {
    self.application_interval
  }
}

pub enum AsyncTask {
  ApplyRegulationForUser(UserId),
  UpdateAfterAllowingScreenAccessForUser(UserId),
  UpdateAfterBlockingLoginForUser(UserId),
  UpdateAfterTerminatingUser(UserId),
}

impl Into<TopAsyncTask> for AsyncTask {
  fn into(self) -> TopAsyncTask {
    TopAsyncTask::ScreenAccessRegulation(self)
  }
}

fn schedule_apply_regulation_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
  delay: Duration,
) {
  scheduler.add_delayed_operation(
    AsyncTask::ApplyRegulationForUser(user_id), 
    delay,
  );
}

fn schedule_update_after_allowing_screen_access_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
) {
  scheduler.add_immediate_operation(
    AsyncTask::UpdateAfterAllowingScreenAccessForUser(user_id),
  );
}

fn schedule_update_after_blocking_login_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
) {
  scheduler.add_immediate_operation(
    AsyncTask::UpdateAfterBlockingLoginForUser(user_id),
  );
}

fn schedule_update_after_terminating_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
) {
  scheduler.add_immediate_operation(
    AsyncTask::UpdateAfterTerminatingUser(user_id),
  );
}

fn execute_apply_regulation_for_user(
  user_id: UserId,
  daemon: Arc<Daemon>
) {
  let (
    action,
    user_id,
    user_name,
    user_password,
    user_blocked_state_password,
    application_status,
    application_interval,
  ) = {
    let Ok(mut integration) = daemon.operating_system_integration().lock_data() else {
      // TODO: Recover from this error case.
      return;
    };

    let integration = &mut *integration;

    let Some(user) = integration.users.get_mut(&user_id) else {
      // User is no longer managed. Don't schedule more async tasks for it.
      return;
    };

    let now = DateTime::now();
    (
      user
        .user_screen_access_regulation_logic
        .calculate_action(now),
      user.user_id,
      user.user_name.clone(),
      user.user_password.clone(),
      integration
        .screen_access_regulation_integration
        .blocked_state_password
        .clone(),
      user
        .user_screen_access_regulation_integration
        .application_status,
      user
        .user_screen_access_regulation_integration
        .application_interval,
    )
  };

  match action {
    Action::Allow => {
      allow_screen_access_for_user(
        &daemon.operating_system_integration().async_scheduler(), 
        user_id, 
        user_name, 
        user_password, 
        application_status, 
        application_interval,
      );
    }
    Action::Block => {
      block_screen_access_for_user(
        &daemon.operating_system_integration().async_scheduler(), 
        user_id,
        user_name,
        user_blocked_state_password, 
        application_status, 
        application_interval,
      );
    }
  }
}

fn allow_screen_access_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
  user_name: UserName,
  user_password: UserPassword,
  application_status: ApplicationStatus,
  application_interval: Duration,
) {
  if application_status == ApplicationStatus::Allowed {
    schedule_apply_regulation_for_user(scheduler, user_id, application_interval);
    return;
  }

  match change_user_password(&user_name, &user_password) {
    Ok(_) => {
      schedule_update_after_allowing_screen_access_for_user(scheduler, user_id);
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

fn block_screen_access_for_user(
  scheduler: &AsyncScheduler,
  user_id: UserId,
  user_name: UserName,
  user_blocked_state_password: UserPassword,
  application_status: ApplicationStatus,
  application_interval: Duration,
) {
  if application_status == ApplicationStatus::LoginBlockedAndSessionTerminated {
    schedule_apply_regulation_for_user(scheduler, user_id, application_interval);
    return;
  }
  
  if application_status == ApplicationStatus::Unknown
  || application_status == ApplicationStatus::Allowed
  {
    if let Err(_) = change_user_password(&user_name, &user_blocked_state_password) {
      // The default behavior now is just to keep trying.
      // TODO: Find a better way to handl this situation.
      // TODO: Log the error somewhere to:
      //       - Let the user know an error occured while allowing internet access
      //       - Let the trubleshooter why exactly the error occured.

      schedule_apply_regulation_for_user(scheduler, user_id, application_interval);
      return;
    }
  }

  match terminate_user_sessions_using_loginctl(user_id) {
    Ok(_) => {
      schedule_update_after_terminating_user(scheduler, user_id);
    }
    Err(_) => {
      schedule_update_after_blocking_login_for_user(scheduler, user_id);
      // The default behavior now is just to keep trying.
      // TODO: Find a better way to handl this situation.
      // TODO: Log the error somewhere to:
      //       - Let the user know an error occured while allowing internet access
      //       - Let the trubleshooter why exactly the error occured.

      schedule_apply_regulation_for_user(scheduler, user_id, application_interval);

      // TODO: Update the database 
      return;
    }
  }
}

fn execute_update_after_allowing_screen_access_for_user(
  user_id: UserId,
  daemon: Arc<Daemon>
) {
  {
    let scheduler = daemon.operating_system_integration().async_scheduler();

    let Ok(mut integration) = daemon.operating_system_integration().lock_data() else {
      // We recover from this by keeping trying. 
      // TODO: Make this delayed.
      schedule_update_after_allowing_screen_access_for_user(&*scheduler, user_id);
      return;
    };

    let Some(user) = integration.users.get_mut(&user_id) else {
      // User is no longer managed. Don't schedule more tasks for it.
      return;
    };

    user
      .user_screen_access_regulation_integration
      .application_status 
      = ApplicationStatus::Allowed;
  }

  if let Err(error) = db::update_screen_access_regulation_application_status(
    daemon.database(), 
    user_id, 
    ApplicationStatus::Allowed,
  ) {
    // TODO: Try to recover from this error if possible.
    // TODO: Add more context to `error`.
    daemon.internal_logger().log_error(error);
  }
}

fn execute_update_after_blocking_login_for_user(
  user_id: UserId,
  daemon: Arc<Daemon>,
) {
  {
    let scheduler = daemon.operating_system_integration().async_scheduler();

    let Ok(mut integration) = daemon.operating_system_integration().lock_data() else {
      // Recover from this by keeping trying. 
      // TODO: make this delayed.
      schedule_update_after_allowing_screen_access_for_user(&*scheduler, user_id);
      return;
    };

    let Some(user) = integration.users.get_mut(&user_id) else {
      // User is no longer managed. Don't schedule more tasks for it.
      return;
    };

    user
      .user_screen_access_regulation_integration
      .application_status 
      = ApplicationStatus::LoginBlocked;

    schedule_apply_regulation_for_user(
      &*scheduler, 
      user_id, 
      user.user_screen_access_regulation_integration.application_interval,
    );
  }

  if let Err(error) = db::update_screen_access_regulation_application_status(
    daemon.database(), 
    user_id, 
    ApplicationStatus::LoginBlocked,
  ) {
    // TODO: Try to recover from this error if possible.
    // TODO: Add more context to `error`.
    daemon.internal_logger().log_error(error);
  }
}

fn execute_update_after_terminating_user(
  user_id: UserId,
  daemon: Arc<Daemon>,
) {
  {
    let scheduler = daemon.operating_system_integration().async_scheduler();

    let Ok(mut integration) = daemon.operating_system_integration().lock_data() else {
      // Recover from this by keeping trying. 
      // TODO: make this delayed.
      schedule_update_after_terminating_user(&*scheduler, user_id);
      return;
    };

    let Some(user) = integration.users.get_mut(&user_id) else {
      // User is no longer managed. Don't schedule more tasks for it.
      return;
    };

    user
      .user_screen_access_regulation_integration
      .application_status 
      = ApplicationStatus::LoginBlockedAndSessionTerminated;

    schedule_apply_regulation_for_user(
      &*scheduler, 
      user_id, 
      user.user_screen_access_regulation_integration.application_interval,
    );
  }

  if let Err(error) = db::update_screen_access_regulation_application_status(
    daemon.database(), 
    user_id, 
    ApplicationStatus::LoginBlockedAndSessionTerminated,
  ) {
    // TODO: Try to recover from this error if possible.
    // TODO: Add more context to `error`.
    daemon.internal_logger().log_error(error);
  }
}

impl AsyncTask {
  pub fn execute(self, daemon: Arc<Daemon>) {
    match self {
      AsyncTask::ApplyRegulationForUser(user_id) => {
        execute_apply_regulation_for_user(user_id, daemon);
      }
      AsyncTask::UpdateAfterAllowingScreenAccessForUser(user_id) => {
        execute_update_after_allowing_screen_access_for_user(user_id, daemon);
      }
      AsyncTask::UpdateAfterBlockingLoginForUser(user_id) => {
        execute_update_after_blocking_login_for_user(user_id, daemon);
      }
      AsyncTask::UpdateAfterTerminatingUser(user_id) => {
        execute_update_after_terminating_user(user_id, daemon);
      }
    }
  }
}