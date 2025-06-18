use super::{
  PolicySpecification, RuleSpecification, DatabaseNamespace, GenericError,
  CommonInfoSpecification, CompoundTypeDefiner, Database, CompoundTypeNamespace,
};

pub struct Specification {
  pub common_info: CommonInfoSpecification,
  pub policy: PolicySpecification,
  pub rule: RuleSpecification,
}

impl Specification {
  pub fn new(
    database: &mut Database,
    database_namespace: &mut DatabaseNamespace,
    soleton_namespace: &mut CompoundTypeNamespace,
    soleton_definer: &mut CompoundTypeDefiner,
  ) ->
    Result<Self, GenericError>
  {
    let common_info = CommonInfoSpecification::new(
      soleton_namespace,
      soleton_definer,
    )?;

    let policy = PolicySpecification::new(
      database,
      database_namespace,
    )?;

    let rule = RuleSpecification::new(
      database,
      database_namespace,
    )?;

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