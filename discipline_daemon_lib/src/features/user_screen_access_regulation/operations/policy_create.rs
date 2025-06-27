use super::{
  Serialize, Deserialize, Uuid, Daemon, 
  PolicyCreator, DateTime, PolicyPublicRepr,
  IsOperation, IntoPublic, InternalOperationOutcome,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicy {
  user_id: Uuid,
  policy_creator: PolicyCreator
}

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  ReachedMaximumPolicesAllowed,
  Success(PolicyPublicRepr),
}

impl IsOperation for CreatePolicy {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser);
    };

    let regulator = &mut user.screen_access_regulator;

    if regulator.reached_maximum_polices_allowed() {
      return InternalOperationOutcome::public_outcome(Outcome::ReachedMaximumPolicesAllowed);
    }

    let now = DateTime::now();
    let policy = self.policy_creator.create(now);

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .add_policy(
        &daemon.database_connection, 
        &self.user_id,
        &policy, 
      )
    {
      return InternalOperationOutcome::internal_error(error);
    }

    regulator.add_policy(policy.clone());
    InternalOperationOutcome::public_outcome(Outcome::Success(policy.into_public()))
  }
}