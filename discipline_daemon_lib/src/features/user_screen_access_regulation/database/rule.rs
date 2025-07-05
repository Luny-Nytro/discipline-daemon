use super::{
  Rule, RuleActivator, GenericError, Uuid, CompoundValueSerializerContext,
  Field, CompoundValueSerializer, IsCollectionItem,
  CompoundValueDeserializer, CompoundValueDeserializerContext,
  CollectionItemDefiner, RuleActivatorSpecification
};

pub struct RuleSpecification {
  pub(super) id: Field,
  pub(super) user_id: Field,
  pub(super) policy_id: Field,
  pub(super) activator: RuleActivatorSpecification,
}

impl IsCollectionItem for RuleSpecification {
  fn new(definer: &mut CollectionItemDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      id: definer.primary_scalar_field("Id")?,
      user_id: definer.primary_scalar_field("UserId")?,
      policy_id: definer.primary_scalar_field("PolicyId")?,
      activator: definer.compound_field("Activator")?,
    })
  }

  fn display_name(&self) -> &str {
    "Rule"
  }
}

impl RuleSpecification {
  pub fn activator(&self) -> &RuleActivatorSpecification {
    &self.activator
  }
}

pub struct RuleSerializer<'a> {
  rule_specification: &'a RuleSpecification,
  user_id: &'a Uuid,
  policy_id: &'a Uuid,
}

impl<'a> RuleSerializer<'a> {
  pub fn new(
    user_id: &'a Uuid,
    policy_id: &'a Uuid,
    rule_specification: &'a RuleSpecification,
  ) -> Self {
    Self {
      user_id,
      policy_id,
      rule_specification,
    }
  }
}

impl<'a> CompoundValueSerializer for RuleSerializer<'a> {
  type CompoundValue = Rule;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) -> 
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.rule_specification.id, &value.id)?;
    context.serializable_scalar(&self.rule_specification.user_id, self.user_id)?;
    context.serializable_scalar(&self.rule_specification.policy_id, self.policy_id)?;
    context.serializable_compound(&self.rule_specification.activator, &value.activator)
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedRule {
  pub(super) id: Uuid,
  pub(super) user_id: Uuid,
  pub(super) policy_id: Uuid,
  pub(super) activator: RuleActivator,
}

impl NormalizedRule {
  pub fn denormalize(self) -> Rule {
    Rule {
      id: self.id,
      activator: self.activator,
    }
  }
}

impl CompoundValueDeserializer for RuleSpecification {
  type CompoundValue = NormalizedRule;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
    Ok(NormalizedRule {
      id: context.deserializable_scalar(&self.id)?,
      user_id: context.deserializable_scalar(&self.id)?,
      policy_id: context.deserializable_scalar(&self.id)?,
      activator: context.deserialize_compound(&self.activator)?,
    })
  }
}
