use super::{
  Serialize, Deserialize, Uuid, Daemon, ToPublicRepr,
  GenericError, PolicyCreator, DateTime, PolicyPublicRepr
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_creator: PolicyCreator
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
  UserNotFound,
  ReachedMaximumPolicesAllowed,
  InternalError(GenericError)
}

impl Operation {
  pub fn execute(self, daemon: &mut Daemon) -> Result<PolicyPublicRepr, Error> {
    let Some(user) = daemon.state.get_user_by_id_mut(&self.user_id) else {
      return Err(Error::UserNotFound);
    };

    let regulator = &mut user.screen_access_regulator;

    if regulator.reached_maximum_polices_allowed() {
      return Err(Error::ReachedMaximumPolicesAllowed);
    }

    let now = DateTime::now();
    let mut policy = self.policy_creator.create(now);

    if let Err(error) = daemon
      .schema
      .user_screen_access_regulation_policy
      .insert_policy(&daemon.database_connection, &policy, &self.user_id)
    {
      return Err(Error::InternalError(error));
    }

    let public_repr = policy.to_public_repr();
    regulator.add_policy(policy);
    Ok(policy)
  }
}