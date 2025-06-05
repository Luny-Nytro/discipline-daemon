use super::{
  PolicySchema, RuleSchema, DatabaseNamespace, GenericError,
  CommonInfoSchema, CompoundTypeSpecificationCreator,
};

pub struct FeatureSchema {
  pub common_info: CommonInfoSchema,
  pub policy: PolicySchema,
  pub rule: RuleSchema,
}

impl FeatureSchema {
  pub fn new(
    database_namespace: &DatabaseNamespace,
    column_namespace: &CompoundTypeSpecificationCreator,
  ) ->
    Result<Self, GenericError>
  {
    let common_info = CommonInfoSchema::new(column_namespace)
      .map_err(|error| error.change_context("create user screen access regulation schema"))?;

    let policy = PolicySchema::new(database_namespace)
      .map_err(|error| error.change_context("create user screen access regulation schema"))?;

    let rule = RuleSchema::new(database_namespace)
      .map_err(|error| error.change_context("create user screen access regulation schema"))?;

    Ok(Self { common_info, policy, rule })
  }

  pub fn singleton(&self) -> &CommonInfoSchema {
    &self.common_info
  }

  pub fn generate_sql_initialize(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError>
  {
    // self.common_info.generate_sql_initialize(into)?;
    self.policy.generate_sql_initialize(into)?;
    self.rule.generate_sql_initialize(into)?;
    Ok(())
  }

}