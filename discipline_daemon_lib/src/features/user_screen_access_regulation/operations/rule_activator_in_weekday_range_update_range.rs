use super::{
  Serialize, Deserialize, Daemon, IsOperation, Uuid, WeekdayRange, 
  RuleActivator, InternalOperationOutcome,
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  NoSuchRule,
  WrongActivatorType,
  MayNotMakeRuleLessRestrictive,
  Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  policy_id: Uuid,
  user_id: Uuid,
  new_weekday_range: WeekdayRange,
}

impl IsOperation for Operation {
  type Outcome = Outcome;

  // TODO: Refuse to execute this operation if it would result in making 
  // the user blocked for too long or for most of the time.
  //
  // This is crucial for safety to prevent the app user from accidently 
  // blocking himself outside of his account forever or most of the time.

  fn execute(self, daemon: &mut Daemon) -> InternalOperationOutcome<Outcome> {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchUser);
    };

    let Some(policy) = user
      .screen_access_regulator
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchPolicy);
    };

    let Some(rule) = policy
      .find_rule_by_id_mut(&self.rule_id) else 
    {
      return InternalOperationOutcome::public_outcome(Outcome::NoSuchRule);
    };

    let RuleActivator::InWeekdayRange(weekday_range) = &mut rule.activator else {
      return InternalOperationOutcome::public_outcome(Outcome::WrongActivatorType);
    };
    
    if self.new_weekday_range.is_narrower_than(weekday_range) {
      return InternalOperationOutcome::public_outcome(Outcome::MayNotMakeRuleLessRestrictive);
    }

    let mut modifications_draft = daemon
      .state_database_specification
      .user_screen_access_regulation
      .rule
      .create_modifications_draft();

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .rule
      .activator()
      .in_weekday_range()
      .update_range(&mut modifications_draft, &self.new_weekday_range)
    {
      return InternalOperationOutcome::internal_error(error);
    }

    if let Err(error) = daemon
      .state_database_specification
      .user_screen_access_regulation
      .rule
      .apply_modifications_draft(
        &daemon.database_connection, 
        &modifications_draft, 
        &self.user_id, 
        &self.policy_id, 
        &self.rule_id,
      )
    {
      return InternalOperationOutcome::internal_error(error);
    }

    *weekday_range = self.new_weekday_range;
    InternalOperationOutcome::public_outcome(Outcome::Success)
  }
}
