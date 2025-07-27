use serde::{Deserialize, Serialize};
use crate::database::operating_system_integration_linux_user as user_db;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublicRepr {
  user_id: UserId,
  user_name: UserName,
  // TODO: Should we keep the password private?
  user_password: UserPassword,
  screen_access_regulation: 
    screen_access_regulation
    ::RegulationPublicRepr,  
  user_screen_access_regulation_application: 
    screen_access_regulation_application
    ::api
    ::UserScreenAccessRegulationApplicationDataPublicRepr,
}

impl IntoPublic for UserInfo {
  type Output = UserPublicRepr;

  fn into_public(self) -> Self::Output {
    UserPublicRepr {
      user_id: self.user_id,
      user_name: self.user_name,
      user_password: self.user_password,
      screen_access_regulation: self.user_screen_access_regulation.into_public(),
      user_screen_access_regulation_application: self.user_screen_access_regulation_application.into_public(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManageUser {
  user_identification_method: UserIdentificationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManageUserReturn {
  NoSuchUser,
  AlreadyManaged,
  InternalError,
  Success(UserPublicRepr),
}

impl ManageUser {
  pub fn execute(self, daemon: &Daemon) -> ManageUserReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return ManageUserReturn::InternalError;
      }
    };

    if data.is_user_managed(&self.user_identification_method) {
      return ManageUserReturn::AlreadyManaged;
    }

    let user_info = retrieve_user_info(self.user_identification_method);
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

    if let Err(error) = user_db::add_user(daemon.database(), &user) {
      daemon.internal_logger().log_error(error);
      return ManageUserReturn::InternalError;
    }

    data.users.insert(user.user_id, user.clone());
    ManageUserReturn::Success(user.into_public())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmanageUser {
  user_id: UserId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnmanageUserReturn {
  AlreadyNotManaged,
  InternalError,
  SomeScreenAccessRegulationPoliciesAreStillEnabled,
  Success,
}

impl UnmanageUser {
  pub fn execute(self, daemon: &Daemon) -> UnmanageUserReturn {
    let mut data = match daemon.operating_system_integration().lock_data() {
      Ok(data) => {
        data
      }
      Err(error) => {
        daemon.internal_logger().log_error(error);
        return UnmanageUserReturn::InternalError;
      }
    };

    let Some(user) = data.users.get_mut(&self.user_id) else {
      return UnmanageUserReturn::AlreadyNotManaged;
    };

    if user.user_screen_access_regulation.are_some_policies_enabled() {
      return UnmanageUserReturn::SomeScreenAccessRegulationPoliciesAreStillEnabled;
    }

    if let Err(error) = user_db::delete_user(
      daemon.database(), 
      self.user_id
    ) {
      daemon.internal_logger().log_error(error);
      return UnmanageUserReturn::InternalError;
    }

    data.users.remove(&self.user_id);
    UnmanageUserReturn::Success
  }
}