use crate::{CountdownTimer, SynchronizeContext};

// change_user_password
// enforce_rules_event
// enforce_rules_event_listener
pub struct DailyUserAccessAllowance {
  countdown_timer: CountdownTimer,
  is_user_blocked: bool,
}

impl DailyUserAccessAllowance {
  pub fn on_synchronize(&mut self, event: &SynchronizeContext) {
    
  }

  pub fn on_user_access_blocked(&mut self) {
    self.is_user_blocked = true;
  }

  pub fn on_user_access_allowed(&mut self) {
    self.is_user_blocked = false;
  }
}