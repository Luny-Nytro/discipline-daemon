use serde::{Serialize, Deserialize};
use crate::database::operating_system_integration_linux_user as user_db;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserScreenAccessRegulationApplicationDataPublicRepr {
  enabled: bool,
  login_blocked: bool,
  checking_interval: Duration,
}

impl IntoPublic for UserScreenAccessRegulationApplicationData {
  type Output = UserScreenAccessRegulationApplicationDataPublicRepr;

  fn into_public(self) -> Self::Output {
    UserScreenAccessRegulationApplicationDataPublicRepr {
      enabled: self.enabled(),
      login_blocked: self.login_blocked(),
      checking_interval: self.check_interval(),
    }
  }
}

pub struct EnableRegulationApplication {
  user_id: UserId,
}

pub enum EnableRegulationApplicationReturn {
  NoSuchUser { user_id: UserId },
  AlreadyEnabled,
  Success,
  InternalError,
}

impl EnableRegulationApplication {
  pub fn execute(Self { user_id }: Self, daemon: Daemon) -> EnableRegulationApplicationReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return EnableRegulationApplicationReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&user_id) else {
      return EnableRegulationApplicationReturn::NoSuchUser { user_id };
    };

    if user.user_screen_access_regulation_application.enabled {
      return EnableRegulationApplicationReturn::AlreadyEnabled;
    }

    if let Err(error) = user_db::update_user_screen_access_regulation_application_enabled(
      daemon.database(), 
      user_id, 
      true
    ) {
      daemon.internal_logger().log_error(error);
      return EnableRegulationApplicationReturn::InternalError;
    }

    user.user_screen_access_regulation_application.enabled = true;
    EnableRegulationApplicationReturn::Success
  }
}

pub struct DisableRegulationApplication {
  user_id: UserId,
}

pub enum DisableRegulationApplicationReturn {
  NoSuchUser { user_id: UserId },
  AlreadyDisabled,
  SomeScreenAccessRegulationPoliciesAreStillEnabled,
  InternalError,
  Success,
}

impl DisableRegulationApplication {
  pub fn execute(Self { user_id }: Self, daemon: Daemon) -> DisableRegulationApplicationReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return DisableRegulationApplicationReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&user_id) else {
      return DisableRegulationApplicationReturn::NoSuchUser { user_id };
    };

    if !user.user_screen_access_regulation_application.enabled {
      return DisableRegulationApplicationReturn::AlreadyDisabled;
    }

    if user.user_screen_access_regulation.are_some_policies_enabled() {
      return DisableRegulationApplicationReturn::SomeScreenAccessRegulationPoliciesAreStillEnabled;
    }

    if let Err(error) = user_db::update_user_screen_access_regulation_application_enabled(
      daemon.database(), 
      user_id, 
      false,
    ) {
      daemon.internal_logger().log_error(error);
      return DisableRegulationApplicationReturn::InternalError;
    }

    user.user_screen_access_regulation_application.enabled = true;
    DisableRegulationApplicationReturn::Success
  }
}

// TODO: Create operations to let the user modify the check_interval field