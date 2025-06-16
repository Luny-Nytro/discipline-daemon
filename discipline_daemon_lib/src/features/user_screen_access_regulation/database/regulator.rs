use super::{
  ScalarFieldSpecification, CompoundTypeDefiner, GenericError, 
  Regulator, CollectionItemModificationsDraft, CompoundValueSerializerContext,
  CompoundValueDeserializer, CompoundValueDeserializerContext,
  NormalizedPolicy, NormalizedRule, CompoundValueSerializer,
  CompoundTypeSpecificationProvider, CompoundTypeFieldsSpecification,
  OperatingSystemCalls, Uuid,
};

pub struct RegulatorSpecification {
  pub is_applying_enabled: ScalarFieldSpecification,
  pub is_user_screen_access_blocked: ScalarFieldSpecification,
}

impl RegulatorSpecification {
  pub fn new(creator: &mut CompoundTypeDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      is_applying_enabled: creator
        .scalar_field_specification("IsApplyingEnabled")
        .build()?,

      is_user_screen_access_blocked: creator
        .scalar_field_specification("IsUserScreenAccessBlocked")
        .build()?,
    })
  }

  pub fn update_is_applying_enabled(
    &self, 
    modifications: &mut CollectionItemModificationsDraft,
    new_value: bool,
  ) ->
    Result<(), GenericError>
  {
    modifications.modify_scalar_field(&self.is_applying_enabled, &new_value)
  }

  pub fn update_is_user_screen_access_blocked(
    &self, 
    modifications: &mut CollectionItemModificationsDraft,
    new_value: bool,
  ) ->
    Result<(), GenericError>
  {
    modifications.modify_scalar_field(&self.is_user_screen_access_blocked, &new_value)
  }
}

impl CompoundValueSerializer for RegulatorSpecification {
  type CompoundValue = Regulator;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) -> 
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.is_applying_enabled, &value.is_applying_enabled)?;
    context.serializable_scalar(&self.is_user_screen_access_blocked, &value.is_user_screen_access_blocked)
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRegulator {
  pub(super) is_applying_enabled: bool,
  pub(super) is_user_screen_access_blocked: bool,
}

impl CompoundValueDeserializer for RegulatorSpecification {
  type Output = NormalizedRegulator;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedRegulator {
      is_applying_enabled: context.deserializable_scalar(&self.is_applying_enabled).map_err(|error|
        error
          .change_context("deserializing NormalizedRegulator")
          .add_error("failed deserialize the 'IsApplyingEnabled' field")
      )?,
      is_user_screen_access_blocked: context.deserializable_scalar(&self.is_user_screen_access_blocked).map_err(|error|
        error
          .change_context("deserializing NormalizedRegulator")
          .add_error("failed deserialize the 'IsUserScreenAccessBlocked' field")
      )?,
    })
  }
}

impl NormalizedRegulator {
  pub fn denormalize(
    self, 
    user_id: &Uuid,
    normalized_policies: &Vec<NormalizedPolicy>,
    normalized_rules: &Vec<NormalizedRule>,
  ) -> Regulator {
    // normalized_policies.sort_by(|a, b| a.position.cmp(&b.position));
    // normalized_rules.sort_by(|a, b| a.position.cmp(&b.position));

    Regulator {
      policies: normalized_policies
        .iter()
        .filter(|policy| policy.user_id == *user_id)
        .map(|policy| policy.clone().denormalize(user_id, &normalized_rules))
        .collect(),
      is_applying_enabled: self.is_applying_enabled,
      operating_system_calls: OperatingSystemCalls::new(),
      is_user_screen_access_blocked: self.is_user_screen_access_blocked,
    }
  }
}

impl CompoundTypeSpecificationProvider for RegulatorSpecification {
  fn add_fields(&self, context: &mut CompoundTypeFieldsSpecification) -> Result<(), GenericError> {
    context.add_scalar_field(&self.is_applying_enabled)?;
    context.add_scalar_field(&self.is_user_screen_access_blocked)
  }
}