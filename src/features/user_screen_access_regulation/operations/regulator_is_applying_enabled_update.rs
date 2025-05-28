use super::{
  Serialize, Deserialize, Daemon, Uuid, DateTime, GenericError
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Outcome {
  UserNotFound,
  NoActionNeeded,
  MayNotSetToFalseWhenSomePoliciesAreEnabled,
  InternalError(GenericError),
  Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  new_value: bool,
}

impl Operation {
  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon
      .state
      .get_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::UserNotFound;
    };

    let regulator = &mut user.screen_access_regulator;

    if regulator.is_applying_enabled == self.new_value {
      return Outcome::NoActionNeeded;
    }

    let now = DateTime::now();
    if !self.new_value && regulator.are_some_policies_enabled(now) {
      return Outcome::MayNotSetToFalseWhenSomePoliciesAreEnabled;
    }

    let mut updater = daemon
      .schema
      .user
      .create_updater(&self.user_id);
    
    daemon
      .schema
      .user
      .screen_access_regulator_type
      .set_is_applying_enabled(&mut updater, self.new_value);
      
    if let Err(error) = updater.execute(&daemon.database_connection) {
      return Outcome::InternalError(error);
    }

    user.name = self.new_value;
    Outcome::Success
  }
}
