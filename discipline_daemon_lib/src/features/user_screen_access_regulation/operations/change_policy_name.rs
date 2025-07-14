use super::{
  Serialize, Deserialize, Uuid, PolicyName, Daemon,
  IsPRPC, policy_db
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
  InternalError,
}

impl IsPRPC for ChangePolicyName {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    let regulator = &mut user.screen_access_regulation;

    let Some(policy) = regulator.find_policy_by_id_mut(&self.policy_id) else {
      return Outcome::NoSuchPolicy;
    };

    if let Err(error) = policy_db::update_name(
      &daemon.database, 
      &self.policy_id, 
      &self.new_name,
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    policy.name = self.new_name;
    Outcome::Success
  }
}