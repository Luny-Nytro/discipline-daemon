use super::{
  Serialize, Deserialize, Daemon, IsOperation, Uuid,
  ToPublicRepr, RuleCreator, Policy, GenericError, RulePublicRepr,
};

#[derive(Debug, Clone)]
pub enum Outcome {
  ThereIsNoUserWithId(Uuid),
  ThereIsNoPolicyWithId(Uuid),
  RuleCreationLimitReached,
  ProvidedRuleIdIsUsedByAnotherRule,
  Success(RulePublicRepr)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  user_id: Uuid,
  policy_id: Uuid,
  rule_creator: RuleCreator,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  fn execute(self, daemon: &mut Daemon) -> Result<Outcome, GenericError> {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return Ok(Outcome::ThereIsNoUserWithId(self.user_id));
    };
    
    let Some(policy) = user
      .screen_access_regulator
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Ok(Outcome::ThereIsNoPolicyWithId(self.policy_id));
    };

    if policy.rules.len() >= Policy::MAX_RULES {
      return Ok(Outcome::RuleCreationLimitReached);
    }

    if let Some(rule_id) = self.rule_creator.id {
      if policy.rules.iter().any(|rule| rule.id == rule_id) {
        return Ok(Outcome::ProvidedRuleIdIsUsedByAnotherRule);
      }
    }

    let mut rule = self.rule_creator.create();
    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .rule
      .add_rule(
        &daemon.database_connection, 
        &self.user_id, 
        &self.policy_id,
        &rule, 
      ) 
    {
      return Err(error);
    }

    let public_repr = rule.to_public_repr();
    policy.rules.push(rule);
    Ok(Outcome::Success(public_repr))
  }
}
