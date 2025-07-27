use super::{Daemon, IsRemoteProcedureCall, Serialize, Deserialize, Uuid, UserName, db};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  new_user_name: UserName
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  NoSuchUser,
  Success,
  InternalError,
}

impl IsRemoteProcedureCall for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon.state.find_user_by_id_mut(&self.user_id) else {
      return Outcome::NoSuchUser;
    };

    if let Err(error) = db::update_name(
      &daemon.database, 
      &self.user_id, 
      &self.new_user_name
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    user.name = self.new_user_name;
    Outcome::Success
  }
}