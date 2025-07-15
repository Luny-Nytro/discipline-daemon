use super::{
  Serialize, Deserialize, Daemon, IsRemoteProcedureCall, Uuid,
  IntoPublic, RuleCreator, rule_db, RulePublicRepr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
  NoSuchUser { user_id: Uuid },
  NoSuchPolicy { policy_id: Uuid },
  RuleCreationLimitReached,
  ProvidedRuleIdIsUsedByAnotherRule,
  Success(RulePublicRepr),
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_id: Uuid,
  rule_creator: RuleCreator,
}

impl IsRemoteProcedureCall for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::NoSuchUser { user_id: self.user_id };
    };
    
    let Some(policy) = user
      .screen_access_regulation
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::NoSuchPolicy { policy_id: self.policy_id };
    };

    if policy.reached_maximum_rules_allowed() {
      return Outcome::RuleCreationLimitReached;
    }

    let rule = self.rule_creator.create();
    // Note: The database will handle verifing whether "self.creator.id" is available
    // or taken.
    //
    // TODO: Let's do that ourselves so we can return "ProvidedRuleIdIsUsedByAnotherRule"
    // since if the database were to fail, it won't tell us if it is because of a duplicate id. 
    if let Err(error) = rule_db::add_rule(
      &daemon.database, 
      &rule, 
      &self.user_id, 
      &self.policy_id, 
      policy.rules_number(),
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    policy.add_rule(rule.clone());
    Outcome::Success(rule.into_public())
  }
}
