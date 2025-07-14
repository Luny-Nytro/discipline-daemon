use super::{
  Serialize, Deserialize, Uuid, Daemon, 
  PolicyCreator, DateTime, PolicyPublicRepr,
  IsPRPC, IntoPublic, policy_db,
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
  InternalError,
}

impl IsPRPC for CreatePolicy {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    let regulation = &mut user.screen_access_regulation;

    if regulation.reached_maximum_polices_allowed() {
      return Outcome::ReachedMaximumPolicesAllowed;
    }

    let now = DateTime::now();
    let policy = self.policy_creator.create(now);

    if let Err(error) = policy_db::add_policy(
      &daemon.database, 
      &policy, 
      &self.user_id
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    regulation.add_policy(policy.clone());
    Outcome::Success(policy.into_public())
  }
}