use super::{
  Serialize, Deserialize, Daemon, IsOperation, Uuid,
  IntoPublic, RuleCreator, Policy, InternalOperationOutcome, RulePublicRepr,
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

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::ThereIsNoUserWithId(self.user_id));
    };
    
    let Some(policy) = user
      .screen_access_regulator
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::ThereIsNoPolicyWithId(self.policy_id));
    };

    if policy.rules.len() >= Policy::MAX_RULES {
      return InternalOperationOutcome::public_outcome(Outcome::RuleCreationLimitReached);
    }

    if let Some(rule_id) = self.rule_creator.id {
      if policy.rules.iter().any(|rule| rule.id == rule_id) {
        return InternalOperationOutcome::public_outcome(Outcome::ProvidedRuleIdIsUsedByAnotherRule);
      }
    }

    let rule = self.rule_creator.create();
    if let Err(error) = daemon
      .database_specification
      .user_screen_access_regulation
      .add_rule(
        &daemon.database_connection, 
        &self.user_id, 
        &self.policy_id,
        &rule, 
      ) 
    {
      return InternalOperationOutcome::internal_error(error);
    }

    policy.rules.push(rule.clone());
    InternalOperationOutcome::public_outcome(Outcome::Success(rule.into_public()))
  }
}
