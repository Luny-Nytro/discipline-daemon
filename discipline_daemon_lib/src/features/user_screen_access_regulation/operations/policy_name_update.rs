use super::{
  Serialize, Deserialize, Uuid, PolicyName, Daemon,
  InternalOperationOutcome, IsOperation,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePolicyName {
  user_id: Uuid,
  policy_id: Uuid,
  new_name: PolicyName
}

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  Success,
}

impl IsOperation for ChangePolicyName {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser);
    };

    let regulator = &mut user.screen_access_regulator;

    let Some(policy) = regulator.find_policy_by_id_mut(&self.policy_id) else {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchPolicy);
    };

    let mut modifications_draft = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .create_modifications_draft();
    
    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .update_name(&mut modifications_draft, &self.new_name)
    {
      return InternalOperationOutcome::internal_error(error);
    }

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .apply_modifications_draft(
        &daemon.database_connection, 
        &modifications_draft, 
        &self.user_id,
        &self.policy_id, 
      ) 
    {
      return InternalOperationOutcome::internal_error(error);
    }

    policy.name = self.new_name;
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}