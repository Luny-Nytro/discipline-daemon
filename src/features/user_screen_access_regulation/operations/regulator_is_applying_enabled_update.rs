use super::{
  Serialize, Deserialize, Daemon, Uuid, DateTime, GenericError, IsOperation
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
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

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::NoSuchUser;
    };

    let regulator = &mut user.screen_access_regulator;

    if regulator.is_applying_enabled == self.new_value {
      return Outcome::NoActionNeeded;
    }

    let now = DateTime::now();
    if !self.new_value && regulator.are_some_policies_enabled(now) {
      return Outcome::MayNotSetToFalseWhenSomePoliciesAreEnabled;
    }

    let mut modifications_draft = daemon
      .state_database_specification
      .user_specification
      .create_modifications_draft();
    
    if let Err(error) = daemon
      .state_database_specification
      .user_specification
      .screen_access_regulator_field_specification()
      .update_is_applying_enabled(&mut modifications_draft, self.new_value)
    {
      return Outcome::InternalError(error);
    }
      
    if let Err(error) = daemon
      .state_database_specification
      .user_specification
      .apply_modifications_draft(
        &daemon.database_connection, 
        &modifications_draft, 
        &self.user_id,
      )
    {
      return Outcome::InternalError(error);
    }

    regulator.is_applying_enabled = self.new_value;
    Outcome::Success
  }
}
