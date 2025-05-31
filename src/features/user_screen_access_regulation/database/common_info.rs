use super::{
  Column, ColumnNamespace, CompoundValueSerializer, CommonInfo,
  SerializeContext, Duration, DeserializeContext,
  CompoundValueDeserializer, GenericError, UpdateStatement,
  WriteColumns, WriteColumnsContext,
};

pub struct CommonInfoSchema {
  private_password: Column,
  applying_interval: Column,
}

impl CommonInfoSchema {
  pub fn new(
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

  pub fn set_applying_interval(
    &self, 
    statement: &mut UpdateStatement,
    new_value: Duration,
  ) {
    statement.set(&self.applying_interval, &new_value);
  }
}

impl CompoundValueSerializer for CommonInfoSchema {
  type Input = CommonInfo;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.private_password, &value.private_password);
    context.serializable_scalar(&self.applying_interval, &value.applying_interval);
  }
}

impl CompoundValueDeserializer for CommonInfoSchema {
  type Output = CommonInfo;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(CommonInfo {
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
    context.write_scalar_type(&self.applying_interval)?;
    context.write_scalar_type(&self.private_password)?;
    Ok(())
  }
}