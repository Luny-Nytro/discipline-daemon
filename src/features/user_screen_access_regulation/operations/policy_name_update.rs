use super::{
  Serialize, Deserialize, Uuid, PolicyName, Daemon,
  GenericError, IsOperation,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_id: Uuid,
  new_name: PolicyName
}

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  InternalError(GenericError),
  Success,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Self::Outcome {
    let Some(user) = daemon.state.get_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    let regulator = &mut user.screen_access_regulator;

    let Some(policy) = regulator.get_policy_by_id_mut(&self.policy_id) else {
      return Outcome::NoSuchPolicy;
    };

    let mut updater = daemon
      .schema
      .user_screen_access_regulation
      .policy
      .create_updater(&self.policy_id, &self.user_id);
    
    daemon
      .schema
      .user_screen_access_regulation
      .policy
      .set_name(&mut updater, &self.new_name);

    if let Err(error) = updater.execute(&daemon.database_connection) {
      return Outcome::InternalError(error);
    }

    policy.name = self.new_name;
    Outcome::Success
  }
}