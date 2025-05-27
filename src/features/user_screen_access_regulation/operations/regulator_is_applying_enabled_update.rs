use super::{
  Serialize, Deserialize, Daemon, Uuid, DateTime,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Outcome {
  UserNotFound,
  PolicyNotFound,
  NoActionNeeded,
  MayNotSetToFalseWhenSomePoliciesAreEnabled,

  NoSuchEnforcer,
  EnforcerAlreadyDisabled,
  SomeRulesAreEnabled,
  InternalError,
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
    if self.new_value && regulator.are_some_policies_enabled(now) {
      return Outcome::MayNotSetToFalseWhenSomePoliciesAreEnabled;
    }

    let mut updater = daemon
      .schema
      .user_screen_access_regulation_policies
      .create_policy_updater(self., user_id)
    
    let adapter = &daemon.schema.user_screen_access_regulation_common_info;
    let synchronize_context = daemon.synchronize_source.create_context_for_now();

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.id == self.id) else 
    {
      return Outcome::NoSuchEnforcer;
    };

    if !enforcer.is_enforcing_enabled {
      return Outcome::EnforcerAlreadyDisabled;
    }

    if enforcer.may_be_deleted(&synchronize_context) {
      return Outcome::SomeRulesAreEnabled;
    }

    if let Err(_) = adapter.update_enforcer_is_enforcing_enabled(
      &daemon.database_connection, 
      &self.id,
      false,
    ) {
      return Outcome::InternalError;
    }

    enforcer.is_enforcing_enabled = false;
    Ok(())
  }
}
