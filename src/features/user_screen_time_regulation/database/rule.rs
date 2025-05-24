use super::{
  Rule, RuleActivator, PolicyEnabler, GenericError, Uuid,
  RuleDeactivatorAdapter, RuleActivatorSchema, 
  Column, ColumnNamesapce, CompoundValueSerializer, 
  CompoundValueDeserializer, DeserializeContext, SerializeContext
};

pub struct RuleAdapter {
  pub(super) id: Column,
  pub(super) position: Column,
  pub(super) activator: RuleActivatorSchema,
  pub(super) deactivator: RuleDeactivatorAdapter,
  pub(super) enforcer_id: Column,
}

impl RuleAdapter {
  pub(super) fn new(column_namespace: &ColumnNamesapce) -> Result<Self, GenericError> {
    Ok(Self {
      id: column_namespace
        .create_column_builder("id")
        .primary()
        .build()?,

      position: column_namespace
        .create_column_builder("position")
        .build()?,

      activator: RuleActivatorSchema::new(
        column_namespace.create_namespace("activator")
      )?,

      deactivator: RuleDeactivatorAdapter::new(
        column_namespace.create_namespace("deactivator")
      )?,

      enforcer_id: column_namespace
        .create_column_builder("enforcer_id")
        .build()?,
    })
  }

  pub(super) fn columns(&self) -> Vec<&Column> {
    let mut columns = vec![&self.id, &self.position, &self.enforcer_id];
    columns.extend_from_slice(&self.activator.columns());
    columns.extend_from_slice(&self.deactivator.columns());
    columns
  }
}

pub struct RuleSerializer<'a> {
  rule_adapter: &'a RuleAdapter,
  rule_position: u32,
  enforcer_id: &'a Uuid,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    rule_adapter: &'a RuleAdapter,
    rule_position: u32,
    enforcer_id: &'a Uuid,
  ) -> Self {
    Self {
      rule_adapter,
      rule_position,
      enforcer_id,
    }
  }
}

impl<'a> CompoundValueSerializer for RuleSerializer<'a> {
  type Input = Rule;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.rule_adapter.id, &value.id);
    context.serializable_scalar(&self.rule_adapter.position, &self.rule_position);
    context.serializable_scalar(&self.rule_adapter.enforcer_id, self.enforcer_id);
    context.serializable_compound(&self.rule_adapter.activator, &value.activator);
    context.serializable_compound(&self.rule_adapter.deactivator, &value.deactivator);
  }
}

#[derive(Debug, Clone)]
pub struct RuleNormalized {
  pub(super) id: Uuid,
  pub(super) position: u32,
  pub(super) activator: RuleActivator,
  pub(super) deactivator: PolicyEnabler,
  pub(super) enforcer_id: Uuid,
}

impl RuleNormalized {
  pub fn finalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
      deactivator: self.deactivator,
    }
  }
}

impl CompoundValueDeserializer for RuleAdapter {
  type Output = RuleNormalized;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(RuleNormalized {
      id: context.deserializable_scalar(&self.id).map_err(|error|
        error.change_context("Failed to deserialize RuleNormalized: Failed to deserialize the 'id' field")
      )?,
      position: context.deserializable_scalar(&self.position).map_err(|error|
        error.change_context("Failed to deserialize RuleNormalized: Failed to deserialize the 'position' field")
      )?,
      activator: context.deserialize_compound(&self.activator).map_err(|error|
        error.change_context("Failed to deserialize RuleNormalized: Failed to deserialize the 'activator' field")
      )?,
      deactivator: context.deserialize_compound(&self.deactivator).map_err(|error|
        error.change_context("Failed to deserialize RuleNormalized: Failed to deserialize the 'deactivator' field")
      )?,
      enforcer_id: context.deserializable_scalar(&self.enforcer_id).map_err(|error|
        error.change_context("Failed to deserialize RuleNormalized: Failed to deserialize the 'enforcer_id' field")
      )?,
    })
  }
}
