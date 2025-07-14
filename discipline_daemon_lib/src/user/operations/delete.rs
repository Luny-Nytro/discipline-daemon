use super::{
  Uuid, Serialize, Deserialize, db, DateTime, IsPRPC, 
  Daemon,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  NoSuchUser { user_id: Uuid },
  SomePoliciesAreEnabled,
  InternalError,
  Success,
}

impl IsPRPC for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user_index) = daemon.state.users.iter()
      .position(|user| user.id == self.user_id) else 
    {
      return Outcome::NoSuchUser { 
        user_id: self.user_id 
      };
    };

    let user = &mut daemon.state.users[user_index];
    let now = DateTime::now();
    if user.screen_access_regulation.are_some_policies_enabled(now) {
      return Outcome::SomePoliciesAreEnabled;
    }
    
    if let Err(error) = db::delete_user(&daemon.database, &self.user_id) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    daemon.state.users.remove(user_index);
    Outcome::Success
  }
}