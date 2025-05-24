use crate::database::UpdateStatement;

use super::{
  Rule, RuleActivator, GenericError, Uuid, RuleActivatorSchema, 
  Column, ColumnNamesapce, CompoundValueSerializer, 
  CompoundValueDeserializer, DeserializeContext, SerializeContext
};

pub struct RuleSchema {
  pub(super) id: Column,
  pub(super) user_id: Column,
  pub(super) policy_id: Column,
  pub(super) position: Column,
  pub(super) activator: RuleActivatorSchema,
}

impl RuleSchema {
  pub(super) fn new(column_namespace: &ColumnNamesapce) -> Result<Self, GenericError> {
    Ok(Self {
      id: column_namespace
        .create_column_builder("id")
        .primary()
        .build()?,

      user_id: column_namespace
        .create_column_builder("user_id")
        .build()?,

      policy_id: column_namespace
        .create_column_builder("policy_id")
        .build()?,

      position: column_namespace
        .create_column_builder("position")
        .build()?,

      activator: RuleActivatorSchema::new(
        column_namespace.create_namespace("activator")
      )?,
    })
  }

  pub(super) fn columns(&self) -> Vec<&Column> {
    let mut columns = vec![&self.id, &self.position, &self.user_id];
    columns.extend_from_slice(&self.activator.columns());
    columns
  }

  pub fn activator(&self) -> &RuleActivatorSchema {
    &self.activator
  }
  
  pub fn set_position(
    &self, 
    statement: &mut UpdateStatement,
    new_value: u32
  ) {
    statement.set(&self.position, &new_value);
  }
}

pub struct RuleSerializer<'a> {
  rule_adapter: &'a RuleSchema,
  rule_position: u32,
  user_id: &'a Uuid,
  policy_id: &'a Uuid,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    rule_adapter: &'a RuleSchema,
    rule_position: u32,
    user_id: &'a Uuid,
    policy_id: &'a Uuid,
  ) -> Self {
    Self {
      rule_adapter,
      rule_position,
      user_id,
      policy_id,
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
    context.serializable_scalar(&self.rule_adapter.user_id, self.user_id);
    context.serializable_compound(&self.rule_adapter.activator, &value.activator);
    // context.serializable_compound(&self.rule_adapter.deactivator, &value.deactivator);
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) id: Uuid,
  pub(super) position: u32,
  pub(super) activator: RuleActivator,
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
}

impl NormalizedRule {
  pub fn finalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
    }
  }
}

impl CompoundValueDeserializer for RuleSchema {
  type Output = NormalizedRule;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedRule {
      id: context.deserializable_scalar(&self.id).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'id' field")
      )?,
      user_id: context.deserializable_scalar(&self.id).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'user_id' field")
      )?,
      policy_id: context.deserializable_scalar(&self.id).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'policy_id' field")
      )?,
      position: context.deserializable_scalar(&self.position).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'position' field")
      )?,
      activator: context.deserialize_compound(&self.activator).map_err(|error|
        error
          .change_context("Deserialize NormalizedRule")
          .add_error("Failed to deserialize the 'activator' field")
      )?,
    })
  }
}
