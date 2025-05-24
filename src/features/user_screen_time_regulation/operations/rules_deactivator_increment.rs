use super::{
  Serialize, Deserialize, App, Duration, IsOperation, Uuid
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
  NoSuchRule,
  NoSuchEnforcer,
  WouldBeEffectiveForTooLong,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  rule_id: Uuid,
  enforcer_id: Uuid,
  increment: Duration,
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    // Get feature.
    let feature = &mut app.state.user_access;
    let adapter = &app.state_database_adapter.user_access;

    // Get rule enforcer.
    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.id == self.enforcer_id) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    // Get rule.
    let Some(rule) = enforcer
      .rules
      .iter_mut()
      .find(|rule| rule.id == self.rule_id) else 
    {
      return Err(Error::NoSuchRule);
    };

    let Some(new_remaining_duration) = rule
      .deactivator
      .timer
      .remaining_duration()
      .checked_add(&self.increment) else 
    {
      return Err(Error::WouldBeEffectiveForTooLong);
    };

    if new_remaining_duration.total_weeks() > 3 {
      return Err(Error::WouldBeEffectiveForTooLong);
    }

    if let Err(_) = adapter.rules_deactivator_remaining_duration_update(
      &app.database_connection,
      &self.rule_id,
      &new_remaining_duration
    ) {
      return Err(Error::InternalError);
    }

    rule.deactivator.timer.change_remaining_duration(new_remaining_duration);
    Ok(())
  }
}
