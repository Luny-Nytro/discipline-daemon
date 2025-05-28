use super::{
  Serialize, Deserialize, Daemon, IsOperation, Uuid,
  ToPublicRepr, RuleCreator, Policy, GenericError, RulePublicRepr,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  RuleCreationLimitReached,
  IdUnavailable,
  InternalError(GenericError),
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

  fn execute(self, daemon: &mut Daemon) -> Self::Outcome {
    let Some(user) = daemon
      .state
      .get_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::NoSuchUser;
    };
    
    let Some(policy) = user
      .screen_access_regulator
      .get_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::NoSuchPolicy;
    };

    if policy.rules.len() >= Policy::MAX_RULES {
      return Outcome::RuleCreationLimitReached;
    }

    if let Some(rule_id) = self.rule_creator.id {
      if policy.rules.iter().any(|rule| rule.id == rule_id) {
        return Outcome::IdUnavailable;
      }
    }

    let rule = self.rule_creator.create();
    if let Err(_) = daemon
      .schema
      .user_screen_access_regulation_rule
      .create_rule(
        &daemon.database_connection, 
        &rule, 
        policy.rules.len(), 
        &self.user_id, 
        &self.policy_id,
      ) 
    {
      return Err(Outcome::InternalError);
    }

    let public_repr = rule.to_public_repr();
    policy.rules.push(rule);
    Outcome::Success(public_repr)
  }
}
