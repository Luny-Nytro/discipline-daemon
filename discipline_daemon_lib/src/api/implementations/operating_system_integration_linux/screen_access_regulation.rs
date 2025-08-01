use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::Daemon;
use crate::api::IntoPublic;
use crate::chronic::{Duration};
use crate::operating_system_integration::{UserId};
use crate::operating_system_integration::screen_access_regulation::*;
use crate::database::operating_system_integration_linux_user as user_db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSpecificInfoPublicRepr {
  application_status: ApplicationStatus,
  application_enabled: bool,
  application_interval: Duration,
}

impl IntoPublic for UserSpecificInfo {
  type Output = UserSpecificInfoPublicRepr;

  fn into_public(self) -> Self::Output {
    UserSpecificInfoPublicRepr {
      application_status: self.application_status(),
      application_enabled: self.application_enabled(),
      application_interval: self.application_interval(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnableApplication {
  user_id: UserId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnableApplicationReturn {
  NoSuchUser { user_id: UserId },
  AlreadyEnabled,
  Success,
  InternalError,
}

impl EnableApplication {
  pub const HUMAN_READABLE_ID: &'static str = "OperatingSystemIntegrationScreenAccessRegulationEnableApplication";

  pub fn execute(self, daemon: Arc<Daemon>) -> EnableApplicationReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return EnableApplicationReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return EnableApplicationReturn::NoSuchUser { user_id: self.user_id };
    };

    if user
      .user_screen_access_regulation_integration
      .application_enabled() 
    {
      return EnableApplicationReturn::AlreadyEnabled;
    }

    if let Err(error) = user_db::update_user_screen_access_regulation_application_enabled(
      daemon.database(), 
      self.user_id, 
      true
    ) {
      daemon.internal_logger().log_error(error);
      return EnableApplicationReturn::InternalError;
    }

    user
      .user_screen_access_regulation_integration
      .application_enabled
      = true;

    EnableApplicationReturn::Success
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableApplication {
  user_id: UserId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisableApplicationReturn {
  NoSuchUser { user_id: UserId },
  AlreadyDisabled,
  SomeScreenAccessRegulationPoliciesAreStillEnabled,
  InternalError,
  Success,
}

impl DisableApplication {
  pub fn execute(self, daemon: Arc<Daemon>) -> DisableApplicationReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return DisableApplicationReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return DisableApplicationReturn::NoSuchUser { user_id: self.user_id };
    };

    if !user
      .user_screen_access_regulation_integration
      .application_enabled 
    {
      return DisableApplicationReturn::AlreadyDisabled;
    }

    if user
      .user_screen_access_regulation_logic
      .are_some_policies_enabled() 
    {
      return DisableApplicationReturn::SomeScreenAccessRegulationPoliciesAreStillEnabled;
    }

    if let Err(error) = user_db::update_user_screen_access_regulation_application_enabled(
      daemon.database(), 
      self.user_id, 
      false,
    ) {
      daemon.internal_logger().log_error(error);
      return DisableApplicationReturn::InternalError;
    }

    user
      .user_screen_access_regulation_integration
      .application_enabled = true;
    
    DisableApplicationReturn::Success
  }
}

// TODO: Create operations to let the user modify the check_interval field