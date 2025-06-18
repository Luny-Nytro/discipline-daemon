use super::{
  ScalarFieldSpecification, CompoundTypeDefiner, 
  CompoundTypeSerializer, CommonInfo, CompoundTypeNamespace,
  Duration, CompoundValueDeserializerContext, CollectionItemModificationsDraft,
  CompoundValueDeserializer, GenericError, CompoundTypeSerializerContext,
};

pub struct CommonInfoSpecification {
  private_password: ScalarFieldSpecification,
  applying_interval: ScalarFieldSpecification,
}

impl CommonInfoSpecification {
  pub fn new(
    namespace: &mut CompoundTypeNamespace,
    definer: &mut CompoundTypeDefiner,
  ) -> 
    Result<Self, GenericError>
  {
    Ok(Self {
      private_password: definer.define_required_writable_scalar_field(namespace, "PrivatePassword")?,
      applying_interval: definer.define_required_writable_scalar_field(namespace, "ApplyingInterval")?,
    })
  }

  pub fn set_applying_interval(
    &self, 
    draft: &mut CollectionItemModificationsDraft,
    new_value: Duration,
  ) ->
    Result<(), GenericError>
  {
    draft.set_scalar_field(&self.applying_interval, &new_value)
  }
}

impl CompoundTypeSerializer for CommonInfoSpecification {
  type CompoundType = CommonInfo;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
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