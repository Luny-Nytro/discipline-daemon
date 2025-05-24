use super::{
  Column, ColumnNamesapce, CompoundValueSerializer, 
  CompoundValueDeserializer, DeserializeContext, 
  SerializeContext, UpdateStatementSetClause, CountdownTimerAdapter,
  PolicyEnabler, Duration, GenericError
};

pub struct RuleDeactivatorAdapter {
  timer: CountdownTimerAdapter,
}

impl RuleDeactivatorAdapter {
  pub(super) fn new(column_namespace: ColumnNamesapce) -> Result<Self, GenericError> {
    Ok(Self {
      timer: CountdownTimerAdapter::new(column_namespace)?,
    })
  }

  fn for_duration_timer(&self) -> &CountdownTimerAdapter {
    &self.timer
  }
  
  pub(super) fn columns(&self) -> Vec<&Column> {
    self.timer.columns()
  }
  
  pub(super) fn columns_iterator(&self) -> impl Iterator<Item = &Column> {
    self.timer.columns_iterator()
  }

  pub(super) fn update_remaining_duration(
    &self,
    update_statement_set_clause: &mut UpdateStatementSetClause,
    new_remaining_duration: &Duration
  ) -> 
    Result<(), GenericError> 
  {
    self.timer.update_remaining_duration(update_statement_set_clause, new_remaining_duration)
  }

  pub(super) fn update_after_synchronize(
    &self,
    update_statement_set_clause: &mut UpdateStatementSetClause,
    deactivator: &PolicyEnabler 
  ) -> 
    Result<(), GenericError>
  {
    self.timer.update_after_synchronize(
      update_statement_set_clause, 
      &deactivator.timer
    )
  }
}

impl CompoundValueSerializer for RuleDeactivatorAdapter {
  type Input = PolicyEnabler;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_compound(&self.timer, &value.timer);
  }
}

impl CompoundValueDeserializer for RuleDeactivatorAdapter {
  type Output = PolicyEnabler;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(PolicyEnabler {
      timer: self.timer.deserialize(context).map_err(|error|
        error.change_context("Failed to deserialize RuleDeactivator: Failed to deserialize the 'timer' field")
      )?,
    })
  }
}