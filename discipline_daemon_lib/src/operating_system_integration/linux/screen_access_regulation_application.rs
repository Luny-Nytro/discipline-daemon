use std::sync::{Arc, Mutex};
use crate::features::screen_access_regulation::Action;
use super::*;

// TODO: If terminating a user session takes too long, just kill the session.
// TODO: Reduce lock duration in "block_user_screen_access"
// TODO: Reduce lock duration in "allow_user_screen_access"
// TODO: Allow the user to select from various session termination methods
//       such as "pkill", "loginctl terminate-session" or just covering the screen after login in.

pub struct CommonScreenAccessRegulationApplicationData {
  blocked_user_password: OperatingSystemUserPassword,
}

impl CommonScreenAccessRegulationApplicationData {
  pub fn new() -> Self {
    Self {
      blocked_user_password: OperatingSystemUserPassword::generate_random_password()
    }
  }
}

pub struct UserScreenAccessRegulationApplicationData {
  status: UserScreenAccessRegulationApplictionStatus,
  rechecking_interval: Duration,
}

pub enum UserScreenAccessRegulationApplictionStatus {
  Unknown,
  LoginAllowed,
  LoginBlocked,
  LoginBlockedAndSessionTerminated,
}

pub enum ScreenAccessRegulationAsyncOperation {
  CheckAll,
  CheckOne(OperatingSystemUserId),
  AllowLogin(OperatingSystemUserId),
  BlockLogin(OperatingSystemUserId),
  TerminateSession(OperatingSystemUserId),
  // KillSession(OperatingSystemUserId),
}

impl Into<AsyncOperation> for ScreenAccessRegulationAsyncOperation {
  fn into(self) -> AsyncOperation {
    AsyncOperation::ScreenAccessRegulationApplication(self)
  }
}

impl ScreenAccessRegulationAsyncOperation {
  pub fn execute(
    self, 
    scheduler: Arc<OperationScheduler>,
    integration: Arc<Mutex<Data>>,
  ) {
    match self {
      ScreenAccessRegulationAsyncOperation::CheckAll => {
        execute_check_all(scheduler, integration);
      }
      ScreenAccessRegulationAsyncOperation::AllowLogin(user_id) => {
        execute_allow_login(user_id, scheduler, integration);
      }
      ScreenAccessRegulationAsyncOperation::BlockLogin(user_id) => {
        execute_block_login(user_id, scheduler, integration);
      }
      ScreenAccessRegulationAsyncOperation::CheckOne(user_id) =>{
        execute_check_one(user_id, scheduler, integration);
      }
      ScreenAccessRegulationAsyncOperation::TerminateSession(user_id) => {
        execute_terminate_session(user_id, scheduler, integration);
      }
    }
  }
}

fn execute_check_all(
  scheduler: Arc<OperationScheduler>,
  integration: Arc<Mutex<Data>>,
) {
  let now = DateTime::now();
  let mut integration = integration.lock().unwrap();

  for user in integration.users.values_mut() {
    let action = user.user_screen_access_regulation.calculate_action(now);

    match user.user_screen_access_regulation_application.status {
      UserScreenAccessRegulationApplictionStatus::Unknown => {
        match action {
          Action::Allow => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationAsyncOperation::AllowLogin(user.user_id)
            );
          }
          Action::Block => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationAsyncOperation::BlockLogin(user.user_id)
            );
          }
        }
      }
      UserScreenAccessRegulationApplictionStatus::LoginAllowed => {
        match action {
          Action::Allow => {
            scheduler.add_delayed_operation(
              ScreenAccessRegulationAsyncOperation::CheckOne(user.user_id), 
              user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration(),
            );
          }
          Action::Block => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationAsyncOperation::BlockLogin(user.user_id)
            );
          }
        }
      }
      UserScreenAccessRegulationApplictionStatus::LoginBlocked => {
        match action {
          Action::Allow => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationAsyncOperation::AllowLogin(user.user_id)
            );
          }
          Action::Block => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationAsyncOperation::TerminateSession(user.user_id), 
            );
          }
        }
      }
      UserScreenAccessRegulationApplictionStatus::LoginBlockedAndSessionTerminated => {
        match action {
          Action::Allow => {
            scheduler.add_immediate_operation(
              ScreenAccessRegulationAsyncOperation::AllowLogin(user.user_id)
            );
          }
          Action::Block => {
            scheduler.add_delayed_operation(
              ScreenAccessRegulationAsyncOperation::CheckOne(user.user_id), 
              user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration(),
            );
          }
        }
      }
    }
  }
}

fn execute_check_one(
  user_id: OperatingSystemUserId,
  scheduler: Arc<OperationScheduler>,
  integration: Arc<Mutex<Data>>,
) {
  let mut integration = integration.lock().unwrap();

  let Some(user) = integration.users.get_mut(&user_id) else {
    return;
  };

  let now = DateTime::now();
  let action = user.user_screen_access_regulation.calculate_action(now);
  match user.user_screen_access_regulation_application.status {
    UserScreenAccessRegulationApplictionStatus::Unknown => {
      match action {
        Action::Allow => {
          scheduler.add_immediate_operation(
            ScreenAccessRegulationAsyncOperation::AllowLogin(user_id)
          );
        }
        Action::Block => {
          scheduler.add_immediate_operation(
            ScreenAccessRegulationAsyncOperation::BlockLogin(user_id)
          );
        }
      }
    }
    UserScreenAccessRegulationApplictionStatus::LoginAllowed => {
      match action {
        Action::Allow => {
          scheduler.add_delayed_operation(
            ScreenAccessRegulationAsyncOperation::CheckOne(user_id), 
            user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration(),
          );
        }
        Action::Block => {
          scheduler.add_immediate_operation(
            ScreenAccessRegulationAsyncOperation::BlockLogin(user_id)
          ); 
        }
      }
    }
    UserScreenAccessRegulationApplictionStatus::LoginBlocked => {
      match action {
        Action::Allow => {
          scheduler.add_immediate_operation(
            ScreenAccessRegulationAsyncOperation::AllowLogin(user_id)
          );
        }
        Action::Block => {
          scheduler.add_immediate_operation(
            ScreenAccessRegulationAsyncOperation::TerminateSession(user_id)
          );
        }
      }
    }
    UserScreenAccessRegulationApplictionStatus::LoginBlockedAndSessionTerminated => {
      match action {
        Action::Allow => {
          scheduler.add_immediate_operation(
            ScreenAccessRegulationAsyncOperation::AllowLogin(user_id)
          );
        }
        Action::Block => {
          scheduler.add_delayed_operation(
            ScreenAccessRegulationAsyncOperation::CheckOne(user_id), 
            user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration(),
          );
        }
      }
    }
  }
}

fn execute_allow_login(
  user_id: OperatingSystemUserId,
  scheduler: Arc<OperationScheduler>,
  integration: Arc<Mutex<Data>>,
) {
  let (user_name, user_blocked_password) = {
    let integration = integration.lock().unwrap();

    let Some(user) = integration.users.get(&user_id) else {
      return;
    };

    (
      user.user_name.clone(), 
      integration.screen_access_regulation_application_common_info.blocked_user_password.clone()
    )
  };

  let maybe_error = change_user_password(
    &user_name, 
    &user_blocked_password,
  );

  let mut integration = integration.lock().unwrap();
  
  let Some(user) = integration.users.get_mut(&user_id) else {
    return;
  };

  match maybe_error {
    Ok(_) => {
      // TODO: Update the database, too.
      user.user_screen_access_regulation_application.status = UserScreenAccessRegulationApplictionStatus::LoginBlocked;
      scheduler.add_delayed_operation(
        ScreenAccessRegulationAsyncOperation::CheckOne(user_id),
        user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration(),
      );
    }
    Err(_) => {
      // TODO: Log the error.

      // The default action is to keep trying with an interval.
      // TODO: Handle this situation better.
      scheduler.add_delayed_operation(
        ScreenAccessRegulationAsyncOperation::AllowLogin(user_id),
        user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration(),
      );
    }
  }
}

fn execute_block_login(
  user_id: OperatingSystemUserId,
  scheduler: Arc<OperationScheduler>,
  integration: Arc<Mutex<Data>>,
) {
  let (user_name, user_password) = {
    let integration = integration.lock().unwrap();

    let Some(user) = integration.users.get(&user_id) else {
      return;
    };

    (user.user_name.clone(), user.user_password.clone())
  };

  let maybe_error = change_user_password(
    &user_name, 
    &user_password,
  );

  let mut integration = integration.lock().unwrap();
  
  let Some(user) = integration.users.get_mut(&user_id) else {
    return;
  };

  match maybe_error {
    Ok(_) => {
      // TODO: Update the database, too.
      user.user_screen_access_regulation_application.status = UserScreenAccessRegulationApplictionStatus::LoginBlocked;
      // drop(user);
      drop(integration);
      scheduler.add_immediate_operation(
        ScreenAccessRegulationAsyncOperation::TerminateSession(user_id)
      );
    }
    Err(_) => {
      // TODO: Log the error.

      // The default action is to keep trying with an interval.
      // TODO: Handle this situation better.
      let interval = user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration();
      drop(integration);
      scheduler.add_delayed_operation(
        ScreenAccessRegulationAsyncOperation::BlockLogin(user_id),
        interval,
      );
    }
  }
}

fn execute_terminate_session(
  user_id: OperatingSystemUserId,
  scheduler: Arc<OperationScheduler>,
  integration: Arc<Mutex<Data>>,
) {
  let user_name = {
    let integration = integration.lock().unwrap();

    let Some(user) = integration.users.get(&user_id) else {
      return;
    };

    user.user_name.clone()
  };

  let maybe_error = terminate_user_session(&user_name);

  let mut integration = integration.lock().unwrap();
  let Some(user) = integration.users.get_mut(&user_id) else {
    return;
  };

  match maybe_error {
    Ok(_) => {
      // Note: Don't update the database because we store the "LoginBlocked" 
      // variant for both "LoginBlocked" and "LoginBlockedAndSessionTerminated".
      user.user_screen_access_regulation_application.status = UserScreenAccessRegulationApplictionStatus::LoginBlockedAndSessionTerminated;
      let interval = user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration();
      drop(integration);
      scheduler.add_delayed_operation(
        ScreenAccessRegulationAsyncOperation::CheckOne(user_id), 
        interval,
      );
    }
    Err(_) => {
      // TODO: Log the error

      // The default action is just to keep trying with an interval.
      // TODO: Handle this situation better.
      
      let interval = user.user_screen_access_regulation_application.rechecking_interval.as_standard_duration();
      drop(integration);
      scheduler.add_delayed_operation(
        ScreenAccessRegulationAsyncOperation::TerminateSession(user_id), 
        interval,
      );
    }
  }
}