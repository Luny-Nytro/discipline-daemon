use super::{
  Column, ColumnNamesapce, CompoundValueSerializer, CommonInfo,
  SerializeContext, OperatingSystemPassword, Duration, DeserializeContext,
  CompoundValueDeserializer, GenericError, Regulator, EnforcersAdapter,
  RuleTableSchema, DatabaseNamespace, Uuid, Connection, Rule, TimeRange,
  WeekdayRange
};

pub struct FeatureAdapter {
  private_password: Column,
  enforcing_interval: Column,
  enforcers: EnforcersAdapter,
  rules: RuleTableSchema,
}

impl FeatureAdapter {
  pub fn new(
    database_namespace: &DatabaseNamespace,
    column_namespace: &ColumnNamesapce,
  ) -> 
    Result<Self, GenericError>
  {
    Ok(Self {
      private_password: column_namespace
        .create_column_builder("private_password")
        .build()?,

      enforcing_interval: column_namespace
        .create_column_builder("enforcing_interval")
        .build()?,

      rules: RuleTableSchema::new(&database_namespace)?,
      enforcers: EnforcersAdapter::new(&database_namespace)?,
    })
  }

  pub fn columns(&self) -> Vec<&Column> {
    vec![&self.enforcing_interval, &self.private_password]
  }

  pub fn columns_iterator(&self) -> impl Iterator<Item = &Column> {
    [&self.enforcing_interval, &self.private_password].into_iter()
  }
}

impl CompoundValueSerializer for FeatureAdapter {
  type Input = NormalizedFeature;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  ) {
    context.serializable_scalar(&self.private_password, &value.private_password);
    context.serializable_scalar(&self.enforcing_interval, &value.enforcing_interval);
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedFeature {
  private_password: OperatingSystemPassword,
  enforcing_interval: Duration,
}

impl Default for NormalizedFeature {
  fn default() -> Self {
    Self {
      private_password: CommonInfo::generate_private_password(),
      enforcing_interval: CommonInfo::default_enforcing_interval(),
    }
  }
}

impl CompoundValueDeserializer for FeatureAdapter {
  type Output = NormalizedFeature;

  fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedFeature {
      enforcing_interval: context.deserializable_scalar(&self.enforcing_interval).map_err(|error|
        error.change_context("Failed to deserialize FeatureNormalized: Failed to deserialize the 'enforcing_interval' field")
      )?,
      private_password: context.deserializable_scalar(&self.private_password).map_err(|error|
        error.change_context("Failed to deserialize FeatureNormalized: Failed to deserialize the 'private_password' field")
      )?,
    })
  }
}

impl NormalizedFeature {
  pub fn finalize(self, enforcers: Vec<Regulator>) -> CommonInfo {
    CommonInfo {
      private_password: self.private_password,
      enforcing_interval: self.enforcing_interval,
      enforcers,
    }
  }
}

impl FeatureAdapter {
  pub fn generate_initialize_sql(
    &self,
    into: &mut String,
  ) -> 
    Result<(), GenericError>
  {
    self.rules.generate_sql_initialize(into)?;
    self.enforcers.generate_ensure_table_created_sql(into)?;
    Ok(())
  }
  
  pub fn generate_delete_enforcer_and_owned_rows_sql(
    &self,
    into: &mut String, 
    enforcer_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    self.enforcers.generate_delete_enforcer_by_id_statement(
      into, 
      enforcer_id
    )?;
  
    self.rules.generate_delete_rules_by_enforcer_id_statement(
      into, 
      enforcer_id
    )?;

    Ok(())
  }
  
  pub fn delete_enforcer_and_owned_rows(
    &self,
    connection: &Connection, 
    enforcer_id: &Uuid
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
  
    self.generate_delete_enforcer_and_owned_rows_sql(
      &mut code, 
      enforcer_id,
    )?;
  
    connection.execute(&code).map_err(|error| 
      error.change_context("Failed to delete an Enforcer and its owned rows: Sqlite error")
        .add_attachment("enforcer", enforcer_id.to_string())
    )
  }

  pub fn generate_update_after_synchronize_sql(
    &self,
    into: &mut String,
    feature: &CommonInfo,
  ) -> 
    Result<(), GenericError> 
  {
    for enforcer in &feature.enforcers {
      self.enforcers.generate_update_after_synchronize_sql(
        into,
        &self.rules,
        enforcer,
      )?;
    }

    Ok(())
  }

  pub fn finalize(
    &self,
    connection: &Connection,
    feature: NormalizedFeature,
  ) -> 
    Result<CommonInfo, GenericError> 
  {
    let mut rule_ir = self.rules.get_all(connection)?;
    rule_ir.sort_by(|a, b| a.position.cmp(&b.position));
  
    let enforcer_ir = self.enforcers.get_all(connection)?;
    let mut enforcers = Vec::new();
  
    for enforcer_ir in enforcer_ir {
      let mut rules = Vec::new();
  
      for rule_ir in &rule_ir {
        if rule_ir.enforcer_id == enforcer_ir.id {
          rules.push(rule_ir.clone().finalize());
        }
      }
  
      enforcers.push(enforcer_ir.finalize(rules));
    }
  
    Ok(feature.finalize(enforcers))
  }  

  pub fn create_enforcer(
    &self, 
    connection: &Connection,
    enforcer: &Regulator,
  ) -> 
    Result<(), GenericError>
  {
    self.enforcers.create_enforcer(connection, enforcer)
  }

  pub fn delete_enforcer(
    &self, 
    connection: &Connection,
    enforcer: &Regulator,
  ) -> 
    Result<(), GenericError>
  {
    let mut code = String::new();
    // TODO: Add more error context
    self.generate_delete_enforcer_and_owned_rows_sql(&mut code, &enforcer.id)?;
    // TODO: Add more error context
    connection.execute(&code)?;
    Ok(())
  }

  pub fn update_enforcer_is_enforcing_enabled(
    &self, 
    connection: &Connection,
    enforcer_id: &Uuid,
    new_value: bool,
  ) -> 
    Result<(), GenericError>
  {
    self.enforcers.update_enforcer_is_user_access_blocked(
      connection, 
      enforcer_id, 
      new_value,
    )
  }

  pub fn delete_rule(
    &self,
    connection: &Connection,
    rule_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    self.rules.delete_rule(connection, rule_id)
  }

  pub fn create_rule(
    &self,
    connection: &Connection,
    rule: &Rule,
    rule_position: u32,
    enforcer_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    self.rules.create_rule(connection, rule, rule_position, enforcer_id)
  }

  pub fn rules_deactivator_remaining_duration_update(
    &self,
    connection: &Connection,
    rule_id: &Uuid,
    new_remaining_duration: &Duration
  ) ->
    Result<(), GenericError>
  {
    self.rules.update_deactivator_remaining_duration(
      connection, 
      rule_id, 
      new_remaining_duration,
    )
  }

  pub fn rules_activator_in_time_range_update(
    &self,
    connection: &Connection,
    rule_id: &Uuid,
    new_time_range: &TimeRange
  ) -> 
    Result<(), GenericError>
  {
    self.rules.replace_rule_activator_in_time_range_by_rule_id(
      connection, 
      rule_id, 
      new_time_range,
    )
  }
  
  pub fn rules_activator_in_weekday_range_update(
    &self,
    connection: &Connection,
    rule_id: &Uuid,
    new_weekday_range: &WeekdayRange
  ) -> 
    Result<(), GenericError>
  {
    self.rules.replace_rule_activator_in_weekday_range(
      connection, 
      rule_id, 
      new_weekday_range,
    )
  }
}