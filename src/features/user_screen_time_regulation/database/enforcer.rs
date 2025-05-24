use super::{
  Column, ColumnNamesapce, GenericError, CompoundValueSerializer,
  Regulator, SerializeContext, Uuid, OperatingSystemUsername,
  OperatingSystemPassword, OperatingSystemUserId, OperatingSystemCalls,
  CompoundValueDeserializer, DeserializeContext, Rule,
};

pub struct EnforcerAdapter {
  pub(super) id: Column,
  pub(super) user_id: Column,
  pub(super) username: Column,
  pub(super) password: Column,
  pub(super) is_enforcing_enabled: Column,
  pub(super) is_user_access_blocked: Column,
}

impl EnforcerAdapter {
  pub(super) fn new(column_namespace: &ColumnNamesapce) -> Result<Self, GenericError> {
    Ok(Self {
      id: column_namespace
        .create_column_builder("id")
        .build()?,

      user_id: column_namespace
        .create_column_builder("user_id")
        .build()?,

      username: column_namespace
        .create_column_builder("username")
        .build()?,

      password: column_namespace
        .create_column_builder("password")
        .build()?,

      is_enforcing_enabled: column_namespace
        .create_column_builder("is_enforcing_enabled")
        .build()?,

      is_user_access_blocked: column_namespace
        .create_column_builder("is_user_access_blocked")
        .build()?,
    })
  }

  pub(super) fn columns(&self) -> Vec<&Column> {
    vec![ 
      &self.id,
      &self.user_id,
      &self.username, 
      &self.password, 
      &self.is_enforcing_enabled, 
      &self.is_user_access_blocked, 
    ]
  }

  pub(super) fn columns_iterator(&self) -> impl Iterator<Item = &Column> {
    [
      &self.id,
      &self.user_id,
      &self.username, 
      &self.password, 
      &self.is_enforcing_enabled, 
      &self.is_user_access_blocked,
    ].into_iter()
  }
}

impl CompoundValueSerializer for EnforcerAdapter {
  type Input = Regulator;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.id, &value.id);
    context.serializable_scalar(&self.user_id, &value.user_id);
    context.serializable_scalar(&self.username, &value.username);
    context.serializable_scalar(&self.password, &value.password);
    context.serializable_scalar(&self.is_enforcing_enabled, &value.is_applying_enabled);
    context.serializable_scalar(&self.is_user_access_blocked, &value.is_user_screen_access_blocked);
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedEnforcer {
  pub(super) id: Uuid,
  pub(super) user_id: OperatingSystemUserId,
  pub(super) username: OperatingSystemUsername,
  pub(super) password: OperatingSystemPassword,
  pub(super) is_enforcing_enable: bool,
  pub(super) is_user_access_blocked: bool,
}

impl CompoundValueDeserializer for EnforcerAdapter {
  type Output = NormalizedEnforcer;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedEnforcer {
      id: context.deserializable_scalar(&self.id).map_err(|error|
        error.change_context("Failed to deserialize NormalizedEnforcer: Failed to deserialize the 'id' field")
      )?,
      user_id: context.deserializable_scalar(&self.user_id).map_err(|error|
        error.change_context("Failed to deserialize NormalizedEnforcer: Failed to deserialize the 'user_id' field")
      )?,
      username: context.deserializable_scalar(&self.user_id).map_err(|error|
        error.change_context("Failed to deserialize NormalizedEnforcer: Failed to deserialize the 'username' field")
      )?,
      password: context.deserializable_scalar(&self.user_id).map_err(|error|
        error.change_context("Failed to deserialize NormalizedEnforcer: Failed to deserialize the 'password' field")
      )?,
      is_enforcing_enable: context.deserializable_scalar(&self.user_id).map_err(|error|
        error.change_context("Failed to deserialize NormalizedEnforcer: Failed to deserialize the 'is_enforcing_enabled' field")
      )?,
      is_user_access_blocked: context.deserializable_scalar(&self.user_id).map_err(|error|
        error.change_context("Failed to deserialize NormalizedEnforcer: Failed to deserialize the 'is_user_access_blocked' field")
      )?,
    })
  }
}

impl NormalizedEnforcer {
  pub fn finalize(self, rules: Vec<Rule>) -> Regulator {
    Regulator {
      id: self.id,
      rules,
      user_id: self.user_id,
      username: self.username,
      password: self.password,
      is_applying_enabled: self.is_enforcing_enable,
      is_user_screen_access_blocked: self.is_user_access_blocked,
      operating_system_calls: OperatingSystemCalls::new(),
    }
  }
}
