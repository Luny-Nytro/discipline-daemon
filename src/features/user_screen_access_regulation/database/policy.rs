use super::{
  GenericError, Column, ColumnNamespace, PolicyEnablerSchema,
  UpdateStatement, PolicyName, CompoundValueSerializer, CompoundValueDeserializer,
  SerializeContext, Policy, Uuid, DateTime, PolicyEnabler, DeserializeContext, Rule,
  WriteColumns, WriteColumnsContext, 
};

pub struct PolicySchema {
  pub id: Column,
  pub name: Column,
  pub enabler: PolicyEnablerSchema,
  pub user_id: Column,
  pub creation_time: Column,
}

impl PolicySchema {
  pub fn new(
    column_namespace: &ColumnNamespace
  ) -> 
    Result<Self, GenericError>
  {
    Ok(Self {
      id: column_namespace
        .create_column_builder("id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create PolicySchema"))?,
      
      name: column_namespace
        .create_column_builder("name")
        .build()
        .map_err(|error| error.change_context("create PolicySchema"))?,
        
      enabler: PolicyEnablerSchema
        ::new(column_namespace.create_namespace("enabler"))
        .map_err(|error| error.change_context("create PolicySchema"))?,
        
      user_id: column_namespace
        .create_column_builder("user_id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create PolicySchema"))?,
        
      creation_time: column_namespace
        .create_column_builder("creation_time")
        .build()
        .map_err(|error| error.change_context("create PolicySchema"))?,
    })
  }

  pub fn set_name(
    &self,
    statement: &mut UpdateStatement,
    new_value: &PolicyName,
  ) {
    statement.set(&self.name, new_value);
  }
}

pub struct PolicySerializer<'a> {
  user_id: &'a Uuid,
  policy_schema: &'a PolicySchema,
}

impl<'a> PolicySerializer<'a> {
  pub fn new(user_id: &'a Uuid, policy_schema: &'a PolicySchema) -> Self {
    Self {
      user_id,
      policy_schema,
    }
  }
}

impl<'a> CompoundValueSerializer for PolicySerializer<'a> {
  type Input = Policy;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.policy_schema.id, &value.id);
    context.serializable_scalar(&self.policy_schema.name, &value.name);
    context.serializable_scalar(&self.policy_schema.user_id, &self.user_id);
    context.serializable_scalar(&self.policy_schema.creation_time, &value.creation_time);
    context.serializable_compound(&self.policy_schema.enabler, &value.enabler);
  }
}

impl CompoundValueDeserializer for PolicySchema {
  type Output = NormalizedPolicy;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedPolicy {
      id: context.deserializable_scalar("id").map_err(|error|
        error
          .change_context("deserialize NormalizedPolicy")
          .add_error("failed to deserialize the 'id' field")
      )?,
      name: context.deserializable_scalar("name").map_err(|error|
        error
          .change_context("deserialize NormalizedPolicy")
          .add_error("failed to deserialize the 'name' field")
      )?,
      user_id: context.deserializable_scalar("user_id").map_err(|error|
        error
          .change_context("deserialize NormalizedPolicy")
          .add_error("failed to deserialize the 'user_id' field")
      )?,
      enabler: context.deserialize_compound("enabler").map_err(|error|
        error
          .change_context("deserialize NormalizedPolicy")
          .add_error("failed to deserialize the 'enabler' field")
      )?,
      creation_time: context.deserializable_scalar("creation_time").map_err(|error|
        error
          .change_context("deserialize NormalizedPolicy")
          .add_error("failed to deserialize the 'creation_time' field")
      )?,
    })
  }
}

pub struct NormalizedPolicy {
  id: Uuid,
  name: PolicyName,
  user_id: Uuid,
  creation_time: DateTime,
  enabler: PolicyEnabler,
}

impl NormalizedPolicy {
  pub fn finalize(self, rules: Vec<Rule>) -> Policy {
    Policy {
      id: self.id,
      name: self.name,
      rules,
      enabler: self.enabler,
      creation_time: self.creation_time,
    }
  }
}

impl WriteColumns for PolicySchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write_scalar_type(&self.id)?;
    context.write_scalar_type(&self.name)?;
    context.write_scalar_type(&self.user_id)?;
    context.write_scalar_type(&self.creation_time)?;
    context.write_compound_type(&self.enabler)?;
    Ok(())
  }
}