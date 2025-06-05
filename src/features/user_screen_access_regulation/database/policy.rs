use super::{
  GenericError, ScalarFieldSpecification, PolicyEnablerSchema, CollectionSpecfication,
  UpdateStatement, PolicyName, CompoundValueSerializer, CompoundValueDeserializer,
  SerializeContext, Policy, Uuid, DateTime, PolicyEnabler, CompoundValueDeserializerContext, Rule,
  WriteColumns, WriteColumnsContext, DatabaseNamespace, Connection,
  generate_sql_initialize_table_given_columns_writer,
  generate_sql_add_row,
  generate_sql_delete_where_2_columns
};

pub struct PolicySchema {
  pub table: CollectionSpecfication,
  pub id: ScalarFieldSpecification,
  pub name: ScalarFieldSpecification,
  pub enabler: PolicyEnablerSchema,
  pub user_id: ScalarFieldSpecification,
  pub creation_time: ScalarFieldSpecification,
}

impl PolicySchema {
  pub fn new(
    database_namespace: &DatabaseNamespace
  ) -> 
    Result<Self, GenericError>
  {
    let table = database_namespace
      .create_table("policies")
      .map_err(|error| error.change_context("create policy schema"))?;

    let column_namespace = table.column_namespace();

    Ok(Self {
      id: column_namespace
        .create_column_builder("id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create policy schema"))?,
      
      name: column_namespace
        .create_column_builder("name")
        .build()
        .map_err(|error| error.change_context("create policy schema"))?,
        
      enabler: PolicyEnablerSchema
        ::new(column_namespace.create_namespace("enabler"))
        .map_err(|error| error.change_context("create policy schema"))?,
        
      user_id: column_namespace
        .create_column_builder("user_id")
        .primary()
        .build()
        .map_err(|error| error.change_context("create policy schema"))?,
        
      creation_time: column_namespace
        .create_column_builder("creation_time")
        .build()
        .map_err(|error| error.change_context("create policy schema"))?,

      table,
    })
  }

  pub fn set_name(
    &self,
    modifications: &mut CollectionItemModifications,
    new_value: &PolicyName,
  ) {
    modifications.modify_scalar_field(&self.name, new_value);
  }
}

pub struct PolicySerializer<'a> {
  user_id: &'a Uuid,
  policy_schema: &'a PolicySchema,
}

impl<'a> PolicySerializer<'a> {
  pub fn new(user_id: &'a Uuid, policy_schema: &'a PolicySchema) -> Self {
    Self {
      user_id,
      policy_schema,
    }
  }
}

impl<'a> CompoundValueSerializer for PolicySerializer<'a> {
  type Input = Policy;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) {
    context.serializable_scalar(&self.policy_schema.id, &value.id);
    context.serializable_scalar(&self.policy_schema.name, &value.name);
    context.serializable_scalar(&self.policy_schema.user_id, self.user_id);
    context.serializable_scalar(&self.policy_schema.creation_time, &value.creation_time);
    context.serializable_compound(&self.policy_schema.enabler, &value.enabler);
  }
}

impl CompoundValueDeserializer for PolicySchema {
  type Output = NormalizedPolicy;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedPolicy {
      id: context.deserializable_scalar(&self.id).map_err(|error|
        error
          .change_context("deserialize normalized policy")
          .add_error("failed to deserialize the 'id' field")
      )?,
      name: context.deserializable_scalar(&self.name).map_err(|error|
        error
          .change_context("deserialize normalized policy")
          .add_error("failed to deserialize the 'name' field")
      )?,
      user_id: context.deserializable_scalar(&self.user_id).map_err(|error|
        error
          .change_context("deserialize normalized policy")
          .add_error("failed to deserialize the 'user_id' field")
      )?,
      enabler: context.deserialize_compound(&self.enabler).map_err(|error|
        error
          .change_context("deserialize normalized policy")
          .add_error("failed to deserialize the 'enabler' field")
      )?,
      creation_time: context.deserializable_scalar(&self.creation_time).map_err(|error|
        error
          .change_context("deserialize normalized policy")
          .add_error("failed to deserialize the 'creation_time' field")
      )?,
    })
  }
}

pub struct NormalizedPolicy {
  id: Uuid,
  name: PolicyName,
  user_id: Uuid,
  creation_time: DateTime,
  enabler: PolicyEnabler,
}

impl NormalizedPolicy {
  pub fn finalize(self, rules: Vec<Rule>) -> Policy {
    Policy {
      id: self.id,
      name: self.name,
      rules,
      enabler: self.enabler,
      creation_time: self.creation_time,
    }
  }
}

impl WriteColumns for PolicySchema {
  fn write_columns(&self, context: &mut WriteColumnsContext) -> Result<(), GenericError> {
    context.write_scalar_type(&self.id)?;
    context.write_scalar_type(&self.name)?;
    context.write_scalar_type(&self.user_id)?;
    context.write_scalar_type(&self.creation_time)?;
    context.write_compound_type(&self.enabler)?;
    Ok(())
  }
}

impl PolicySchema {
  pub fn generate_sql_initialize(
    &self,
    into: &mut String,
  ) -> Result<(), GenericError> {
    generate_sql_initialize_table_given_columns_writer(
      into,
      &self.table,
      self,
    )
    .map_err(|error| 
      error.change_context("generate sql code that initializes everything related to the policies table")
    )
  }

  pub fn generate_sql_insert_policy(
    &self,
    into: &mut String,
    policy: &Policy,
    user_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    let serializer = PolicySerializer::new(user_id, self);
    generate_sql_add_row(into, &self.table, &serializer, policy)
      .map_err(|error| 
        error.change_context("generate sql code that inserts a policy")
      )
  }

  pub fn add_policy(
    &self,
    connection: &Connection,
    policy: &Policy,
    user_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    let mut sql = String::new();

    self
      .generate_sql_insert_policy(&mut sql, policy, user_id)
      .map_err(|error|
        error.change_context("insert a policy into the database")
      )?;

    connection 
      .execute(&sql)
      .map_err(|error|
        error.change_context("insert a policy into the database")
      )
  }

  pub fn generate_sql_delete_policy(
    &self,
    into: &mut String,
    policy_id: &Uuid,
    user_id: &Uuid,
  ) {
    generate_sql_delete_where_2_columns(
      into,
      &self.table,
      &self.id,
      policy_id,
      &self.user_id,
      user_id,
    )
  }

  pub fn delete_policy(
    &self,
    connection: &Connection,
    policy_id: &Uuid,
    user_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    let mut sql = String::new();

    self
      .generate_sql_delete_policy(&mut sql, policy_id, user_id);

    connection
      .execute(&sql)
      .map_err(|error|
        error.change_context("delete a policy from the database")
      )
  }

  pub fn load_all_normalized_policies(
    &self,
    connection: &Connection,
  ) -> 
    Result<Vec<NormalizedPolicy>, GenericError>
  {
    connection
      .find_all_rows(&self.table, self)
      .map_err(|error| 
        error.change_context("retrieve all policies from the database in normalized form")
      )
  }

  pub fn create_updater(
    &self,
    policy_id: &Uuid,
    user_id: &Uuid,
  ) -> 
    UpdateStatement
  {
    UpdateStatement::new_given_two_where_columns(
      &self.id,
      policy_id, 
      &self.user_id, 
      user_id,
    )
  }
}
