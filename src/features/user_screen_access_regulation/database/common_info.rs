use super::{
  Column, ColumnNamespace, CompoundValueSerializer, CommonInfo,
  SerializeContext, OperatingSystemPassword, Duration, DeserializeContext,
  CompoundValueDeserializer, GenericError, UpdateStatement,
  DatabaseNamespace, WriteColumns, WriteColumnsContext,
};

pub struct CommonInfoSchema {
  private_password: Column,
  applying_interval: Column,
}

impl CommonInfoSchema {
  pub fn new(
    database_namespace: &DatabaseNamespace,
    column_namespace: &ColumnNamespace,
  ) -> 
    Result<Self, GenericError>
  {
    Ok(Self {
      private_password: column_namespace
        .create_column_builder("private_password")
        .build()?,

      applying_interval: column_namespace
        .create_column_builder("applying_interval")
        .build()?,
    })
  }

  pub fn columns(&self) -> Vec<&Column> {
    vec![
      &self.applying_interval, 
      &self.private_password,
    ]
  }

  pub fn set_applying_interval(
    &self, 
    statement: &mut UpdateStatement,
    new_value: Duration,
  ) {
    statement.set(&self.applying_interval, &new_value);
  }
}

impl CompoundValueSerializer for CommonInfoSchema {
  type Input = NormalizedFeature;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.private_password, &value.private_password);
    context.serializable_scalar(&self.applying_interval, &value.applying_interval);
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedFeature {
  private_password: OperatingSystemPassword,
  applying_interval: Duration,
}

impl Default for NormalizedFeature {
  fn default() -> Self {
    Self {
      private_password: CommonInfo::generate_private_password(),
      applying_interval: CommonInfo::default_applying_interval(),
    }
  }
}

impl CompoundValueDeserializer for CommonInfoSchema {
  type Output = NormalizedFeature;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedFeature {
      applying_interval: context.deserializable_scalar(&self.applying_interval).map_err(|error|
        error
          .change_context("deserialize CommonInfo")
          .add_error("failed to deserialize the 'applying_interval' field")
      )?,
      private_password: context.deserializable_scalar(&self.private_password).map_err(|error|
        error
          .change_context("deserialize CommonInfo")
          .add_error("failed to deserialize the 'private_password' field")
      )?,
    })
  }
}

impl WriteColumns for CommonInfoSchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write(&self.applying_interval)?;
    context.write(&self.private_password)?;
    Ok(())
  }
}