use super::{
  Column, ColumnNamespace, GenericError, CompoundValueSerializer,
  Regulator, SerializeContext, OperatingSystemCalls,
  CompoundValueDeserializer, DeserializeContext, Policy,
  WriteColumns, WriteColumnsContext,
};

pub struct RegulatorSchema {
  pub(super) is_applying_enabled: Column,
  pub(super) is_user_screen_access_blocked: Column,
}

impl RegulatorSchema {
  pub fn new(column_namespace: &ColumnNamespace) -> Result<Self, GenericError> {
    Ok(Self {
      is_applying_enabled: column_namespace
        .create_column_builder("is_applying_enabled")
        .build()?,

      is_user_screen_access_blocked: column_namespace
        .create_column_builder("is_user_screen_access_blocked")
        .build()?,
    })
  }
}

impl CompoundValueSerializer for RegulatorSchema {
  type Input = Regulator;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.is_applying_enabled, &value.is_applying_enabled);
    context.serializable_scalar(&self.is_user_screen_access_blocked, &value.is_user_screen_access_blocked);
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRegulator {
  pub(super) is_applying_enabled: bool,
  pub(super) is_user_screen_access_blocked: bool,
}

impl CompoundValueDeserializer for RegulatorSchema {
  type Output = NormalizedRegulator;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedRegulator {
      // id: context.deserializable_scalar(&self.id).map_err(|error|
      //   error
        // .change_context("deserialize Regulator: Failed to deserialize the 'id' field")
        // .add_error("")
      // )?,
      // user_id: context.deserializable_scalar(&self.user_id).map_err(|error|
      //   error
        // .change_context("deserialize Regulator: Failed to deserialize the 'user_id' field")
        // .add_error("")
      // )?,
      // username: context.deserializable_scalar(&self.user_id).map_err(|error|
      //   error
        // .change_context("deserialize Regulator: Failed to deserialize the 'username' field")
        // .add_error("")
      // )?,
      // password: context.deserializable_scalar(&self.user_id).map_err(|error|
      //   error
        // .change_context("deserialize Regulator: Failed to deserialize the 'password' field")
        // .add_error("")
      // )?,
      is_applying_enabled: context.deserializable_scalar(&self.is_applying_enabled).map_err(|error|
        error
          .change_context("deserialize NormalizedRegulator")
          .add_error("Failed to deserialize the 'is_applying_enabled' field")
      )?,
      is_user_screen_access_blocked: context.deserializable_scalar(&self.is_user_screen_access_blocked).map_err(|error|
        error
          .change_context("deserialize NormalizedRegulator")
          .add_error("Failed to deserialize the 'is_user_screen_access_blocked' field")
      )?,
    })
  }
}

impl NormalizedRegulator {
  pub fn finalize(self, policies: Vec<Policy>) -> Regulator {
    Regulator {
      policies,
      is_applying_enabled: self.is_applying_enabled,
      operating_system_calls: OperatingSystemCalls::new(),
      is_user_screen_access_blocked: self.is_user_screen_access_blocked,
    }
  }
}

impl WriteColumns for RegulatorSchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write(&self.is_applying_enabled)?;
    context.write(&self.is_user_screen_access_blocked)?;
    Ok(())
  }
}