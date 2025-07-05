use super::{
  GenericError, Field, PolicyEnablerSpecification,
  PolicyName, CompoundValueSerializer, CompoundValueDeserializer,
  Policy, Uuid, PolicyEnabler, CompoundValueDeserializerContext,
  CollectionItemDefiner, CollectionItemModificationsDraft,
  CompoundValueSerializerContext, NormalizedRule,
  IsCollectionItem
};

pub struct PolicySpecification {
  pub id: Field,
  pub name: Field,
  pub enabler: PolicyEnablerSpecification,
  pub user_id: Field,
}

impl IsCollectionItem for PolicySpecification {
  fn new(definer: &mut CollectionItemDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      id: definer.primary_scalar_field("Id")?,
      name: definer.writable_required_field("Name")?,
      enabler: definer.compound_field("Enabler")?,
      user_id: definer.primary_scalar_field("UserId")?,
    })
  }

  fn display_name(&self) -> &str {
    "Policy"
  }
}

impl PolicySpecification {
  pub fn set_name(
    &self,
    changes: &mut CollectionItemModificationsDraft,
    new_value: &PolicyName,
  ) -> 
    Result<(), GenericError>
  {
    changes.write_scalar_field(&self.name, new_value)
  }
}

pub struct PolicySerializer<'a> {
  user_id: &'a Uuid,
  // policy_position: usize,
  policy_specification: &'a PolicySpecification,
}

impl<'a> PolicySerializer<'a> {
  pub fn new(
    user_id: &'a Uuid,
    policy_specification: &'a PolicySpecification,
  ) -> Self {
    Self {
      user_id,
      policy_specification,
    }
  }
}

impl<'a> CompoundValueSerializer for PolicySerializer<'a> {
  type CompoundValue = Policy;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.policy_specification.id, &value.id)?;
    context.serializable_scalar(&self.policy_specification.name, &value.name)?;
    context.serializable_scalar(&self.policy_specification.user_id, self.user_id)?;
    context.serializable_compound(&self.policy_specification.enabler, &value.enabler)
  }
}

impl CompoundValueDeserializer for PolicySpecification {
  type CompoundValue = NormalizedPolicy;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
    Ok(NormalizedPolicy {
      id: context.deserializable_scalar(&self.id)?,
      name: context.deserializable_scalar(&self.name)?,
      user_id: context.deserializable_scalar(&self.user_id)?,
      enabler: context.deserialize_compound(&self.enabler)?,
    })
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedPolicy {
  pub(super) id: Uuid,
  pub(super) name: PolicyName,
  pub(super) user_id: Uuid,
  pub(super) enabler: PolicyEnabler,
}

impl NormalizedPolicy {
  pub fn denormalize(
    self, 
    user_id: &Uuid,
    normalized_rules: &Vec<NormalizedRule>,
  ) -> Policy {
    Policy {
      id: self.id,
      name: self.name,
      rules: normalized_rules
        .iter()
        .filter(|rule| rule.user_id == *user_id && rule.policy_id == self.id)
        .map(|rule| rule.clone().denormalize())
        .collect(),
      enabler: self.enabler,
    }
  }
}