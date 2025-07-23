use super::*;

// TODO: If terminating a user session takes too long, just kill the session.
// TODO: Reduce lock duration in "block_user_screen_access"
// TODO: Reduce lock duration in "allow_user_screen_access"
// TODO: Allow the user to select from various session termination methods
//       such as "pkill", "loginctl terminate-session" or just covering the screen after login in.

pub struct OperatingSystemUser {
  pub operating_system_user_id: OperatingSystemUserId,
  pub operating_system_user_name: OperatingSystemUserName,
  pub operating_system_user_password: OperatingSystemUserPassword,
  pub user_screen_access_data: UserScreenAccessData,
}

pub struct OperatingSystemIntegrationData {
  pub user_screen_access_regulation: Box<dyn IsUserScreenAccessRegulation>,
  pub blocked_user_password: OperatingSystemUserPassword,
  pub operating_system_users: Vec<Option<OperatingSystemUser>>,
  pub interval: Duration,
  pub dropped: bool,
}