use super::{
  Serialize, Deserialize, Uuid, Daemon, ToPublicRepr,
  GenericError, PolicyCreator, DateTime, PolicyPublicRepr,
  IsOperation,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_creator: PolicyCreator
}

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  ReachedMaximumPolicesAllowed,
  Success(PolicyPublicRepr),
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Result<Outcome, GenericError> {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Ok(Outcome::NoSuchUser);
    };

    let regulator = &mut user.screen_access_regulator;

    if regulator.reached_maximum_polices_allowed() {
      return Ok(Outcome::ReachedMaximumPolicesAllowed);
    }

    let now = DateTime::now();
    let mut policy = self.policy_creator.create(now);

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .policy
      .add_policy(
        &daemon.database_connection, 
        &self.user_id,
        &policy, 
      )
    {
      return Err(error);
    }

    let public_repr = policy.to_public_repr();
    regulator.add_policy(policy);
    Ok(Outcome::Success(public_repr))
  }
}

#[derive(Debug, Clone)]
pub enum OutcomePublicRepr {
  NoSuchUser,
  ReachedMaximumPolicesAllowed,
  Success(PolicyPublicRepr),
  InternalError,
}
