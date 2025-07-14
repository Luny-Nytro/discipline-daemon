use super::{
  Serialize, Deserialize, Daemon, Uuid, DateTime, 
  update_screen_access_regulation_is_applying_enabled, IsPRPC
};


#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoActionNeeded,
  SomePoliciesAreEnabled,
  Success,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  new_value: bool,
}

impl IsPRPC for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    let regulation = &mut user.screen_access_regulation;

    if regulation.is_applying_enabled == self.new_value {
      return Outcome::NoActionNeeded;
    }

    let now = DateTime::now();
    if !self.new_value && regulation.are_some_policies_enabled(now) {
      return Outcome::SomePoliciesAreEnabled;
    }

    if let Err(error) = update_screen_access_regulation_is_applying_enabled(
      &daemon.database,
      &self.user_id,
      self.new_value,
    ) {
      daemon.log_internal_error(error.to_debug_string());
      return Outcome::InternalError;
    }

    regulation.is_applying_enabled = self.new_value;
    Outcome::Success
  }
}
