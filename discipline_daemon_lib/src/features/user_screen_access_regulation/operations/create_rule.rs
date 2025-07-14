use super::{
  Serialize, Deserialize, Daemon, IsPRPC, Uuid,
  IntoPublic, RuleCreator, Policy, rule_db, RulePublicRepr,
};

#[derive(Debug, Clone)]
pub enum Outcome {
  ThereIsNoUserWithId(Uuid),
  ThereIsNoPolicyWithId(Uuid),
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

impl IsPRPC for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::ThereIsNoUserWithId(self.user_id);
    };
    
    let Some(policy) = user
      .screen_access_regulation
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::ThereIsNoPolicyWithId(self.policy_id);
    };

    if policy.rules.len() >= Policy::MAX_RULES {
      return Outcome::RuleCreationLimitReached;
    }

    if let Some(rule_id) = self.rule_creator.id {
      if policy.rules.iter().any(|rule| rule.id == rule_id) {
        return Outcome::ProvidedRuleIdIsUsedByAnotherRule;
      }
    }

    let rule = self.rule_creator.create();
    if let Err(error) = rule_db::add_rule(
      &daemon.database, 
      &rule, 
      &self.user_id, 
      &self.policy_id, 
      policy.rules.len(),
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    policy.rules.push(rule.clone());
    Outcome::Success(rule.into_public())
  }
}
