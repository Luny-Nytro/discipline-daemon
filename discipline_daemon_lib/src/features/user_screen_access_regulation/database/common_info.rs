use super::{
  Field, CompoundTypeDefiner, IsCompoundType,
  CompoundValueSerializer, CommonInfo,
  Duration, CompoundValueDeserializerContext, CollectionItemModificationsDraft,
  CompoundValueDeserializer, GenericError, CompoundValueSerializerContext,
};

pub struct CommonInfoSpecification {
  private_password: Field,
  applying_interval: Field,
}

impl IsCompoundType for CommonInfoSpecification {
  fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      private_password: definer.writable_required_field("PrivatePassword")?,
      applying_interval: definer.writable_required_field("ApplyingInterval")?,
    })
  }

  fn display_name(&self) -> &str {
    "CommonInfo"
  }
}

impl CommonInfoSpecification {
  pub fn write_applying_interval(
    &self, 
    draft: &mut CollectionItemModificationsDraft,
    new_value: Duration,
  ) ->
    Result<(), GenericError>
  {
    draft.write_scalar_field(&self.applying_interval, &new_value)
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
  type CompoundValue = CommonInfo;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
    Ok(CommonInfo {
      private_password: context.deserializable_scalar(&self.private_password)?,
      applying_interval: context.deserializable_scalar(&self.applying_interval)?,
    })
  }
}