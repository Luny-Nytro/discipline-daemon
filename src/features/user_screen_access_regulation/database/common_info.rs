use super::{
  ScalarFieldSpecification, CompoundTypeFieldsScope, 
  CompoundValueSerializer, CommonInfo, 
  Duration, CompoundValueDeserializerContext, CollectionItemModifications,
  CompoundValueDeserializer, GenericError, CompoundValueSerializerContext,
};

pub struct CommonInfoSpecification {
  private_password: ScalarFieldSpecification,
  applying_interval: ScalarFieldSpecification,
}

impl CommonInfoSpecification {
  pub fn new(
    scope: &mut CompoundTypeFieldsScope,
  ) -> 
    Result<Self, GenericError>
  {
    Ok(Self {
      private_password: scope
        .scalar_field_specification("PrivatePassword")
        .build()?,

      applying_interval: scope
        .scalar_field_specification("ApplyingInterval")
        .build()?,
    })
  }

  pub fn update_applying_interval(
    &self, 
    modifications: &mut CollectionItemModifications,
    new_value: Duration,
  ) ->
    Result<(), GenericError>
  {
    modifications.modify_scalar_field(&self.applying_interval, &new_value)
  }
}

impl CompoundValueSerializer for CommonInfoSpecification {
  type CompoundValue = CommonInfo;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.private_password, &value.private_password)?;
    context.serializable_scalar(&self.applying_interval, &value.applying_interval)
  }
}

impl CompoundValueDeserializer for CommonInfoSpecification {
  type Output = CommonInfo;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(CommonInfo {
      applying_interval: context.deserializable_scalar(&self.applying_interval).map_err(|error|
        error
          .change_context("deserializing CommonInfo")
          .add_error("failed to deserialize the 'ApplyingInterval' field")
      )?,
      private_password: context.deserializable_scalar(&self.private_password).map_err(|error|
        error
          .change_context("deserializing CommonInfo")
          .add_error("failed to deserialize the 'PrivatePassword' field")
      )?,
    })
  }
}