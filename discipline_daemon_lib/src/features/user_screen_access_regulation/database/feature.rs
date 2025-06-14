use super::{
  PolicySpecification, RuleSpecification, Namespace, GenericError,
  CommonInfoSpecification, CompoundTypeFieldsScope,
};

pub struct Specification {
  pub common_info: CommonInfoSpecification,
  pub policy: PolicySpecification,
  pub rule: RuleSpecification,
}

impl Specification {
  pub fn new(
    namespace: &mut Namespace,
    scope: &mut CompoundTypeFieldsScope,
  ) ->
    Result<Self, GenericError>
  {
    let common_info = CommonInfoSpecification::new(scope)?;
    let policy = PolicySpecification::new(namespace)?;
    let rule = RuleSpecification::new(namespace)?;

    Ok(Self { common_info, policy, rule })
  }

  pub fn singleton(&self) -> &CommonInfoSpecification {
    &self.common_info
  }

  // pub fn generate_sql_initialize(
  //   &self,
  //   into: &mut String,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   // self.common_info.generate_sql_initialize(into)?;
  //   self.policy.generate_sql_initialize(into)?;
  //   self.rule.generate_sql_initialize(into)?;
  //   Ok(())
  // }
}