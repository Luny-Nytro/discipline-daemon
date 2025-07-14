use super::{
  Serialize, Deserialize, Daemon, IsPRPC, Uuid, WeekdayRange, 
  RuleActivator, rule_db,
};

#[derive(Debug, Clone)]
pub enum Outcome {
  NoSuchUser,
  NoSuchPolicy,
  NoSuchRule,
  WrongActivatorType,
  MayNotMakeRuleLessRestrictive,
  Success,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  policy_id: Uuid,
  user_id: Uuid,
  new_weekday_range: WeekdayRange,
}

impl IsPRPC for Operation {
  type Outcome = Outcome;

  // TODO: Refuse to execute this operation if it would result in making 
  // the user blocked for too long or for most of the time.
  //
  // This is crucial for safety to prevent the app user from accidently 
  // blocking himself outside of his account forever or most of the time.

  fn execute(self, daemon: &mut Daemon) -> Outcome {
    let Some(user) = daemon
      .state
      .find_user_by_id_mut(&self.user_id) else 
    {
      return Outcome::NoSuchUser;
    };

    let Some(policy) = user
      .screen_access_regulation
      .find_policy_by_id_mut(&self.policy_id) else 
    {
      return Outcome::NoSuchPolicy;
    };

    let Some(rule) = policy
      .find_rule_by_id_mut(&self.rule_id) else 
    {
      return Outcome::NoSuchRule;
    };

    let RuleActivator::InWeekdayRange(weekday_range) = &mut rule.activator else {
      return Outcome::WrongActivatorType;
    };
    
    if self.new_weekday_range.is_narrower_than(weekday_range) {
      return Outcome::MayNotMakeRuleLessRestrictive;
    }

    if let Err(error) = rule_db::update_activator_weekday_range(
      &daemon.database, 
      &self.rule_id, 
      &self.new_weekday_range,
    ) {
      daemon.log_internal_error(error);
      return Outcome::InternalError;
    }

    *weekday_range = self.new_weekday_range;
    Outcome::Success
  }
}
