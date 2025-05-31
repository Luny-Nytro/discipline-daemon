use super::{
  Serialize, Deserialize, Daemon, IsOperation, Uuid, WeekdayRange, 
  RuleActivator, GenericError,
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  NoSuchRule,
  WrongActivatorType,
  MayNotMakeRuleLessRestrictive,
  InternalError(GenericError),
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

    let Some(rule) = policy
      .get_rule_by_id_mut(&self.rule_id) else 
    {
      return Outcome::NoSuchRule;
    };

    let RuleActivator::InWeekdayRange(weekday_range) = &mut rule.activator else {
      return Outcome::WrongActivatorType;
    };
    
    if self.new_weekday_range.is_narrower_than(weekday_range) {
      return Outcome::MayNotMakeRuleLessRestrictive;
    }

    let mut updater = daemon
      .schema
      .user_screen_access_regulation
      .rule
      .create_updater(&self.rule_id, &self.policy_id, &self.user_id);

    daemon
      .schema
      .user_screen_access_regulation
      .rule
      .activator()
      .in_weekday_range()
      .set_range(&mut updater, &self.new_weekday_range);

    if let Err(error) = updater.execute(&daemon.database_connection) {
      return Outcome::InternalError(error);
    }

    *weekday_range = self.new_weekday_range;
    Outcome::Success
  }
}
