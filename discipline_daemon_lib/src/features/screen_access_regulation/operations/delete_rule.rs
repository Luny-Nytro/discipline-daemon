use super::{
  Serialize, Deserialize, Uuid, IsRemoteProcedureCall,
  Daemon, DateTime, rule_db,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  NoSuchRule,
  MayNotDeleteRuleWhilePolicyEnabled,
  Success,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_id: Uuid,
  rule_id: Uuid,
}

impl IsRemoteProcedureCall for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    

    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else
    {
      return Outcome::NoSuchUser;
    };

    let regulation = &mut user.screen_access_regulation;

    let Some(policy) = regulation
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::NoSuchPolicy;
    };

    if policy.there_is_rule_with_id(&self.rule_id) {
      return Outcome::NoSuchRule;
    }

    // let now = DateTime::now();
    if policy.is_enabled() {
      return Outcome::MayNotDeleteRuleWhilePolicyEnabled;
    }

    if let Err(error) = rule_db::delete_rule(&daemon.database, &self.rule_id) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    policy.remove_rule_by_id(&self.rule_id);
    Outcome::Success
  }
}