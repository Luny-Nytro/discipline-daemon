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
  InternalError(GenericError),
  Success(PolicyPublicRepr),
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon.state.get_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    let regulator = &mut user.screen_access_regulator;

    if regulator.reached_maximum_polices_allowed() {
      return Outcome::ReachedMaximumPolicesAllowed;
    }

    let now = DateTime::now();
    let mut policy = self.policy_creator.create(now);

    if let Err(error) = daemon
      .schema
      .user_screen_access_regulation_policy
      .add_policy(&daemon.database_connection, &policy, &self.user_id)
    {
      return Outcome::InternalError(error);
    }

    let public_repr = policy.to_public_repr();
    regulator.add_policy(policy);
    Outcome::Success(policy)
  }
}