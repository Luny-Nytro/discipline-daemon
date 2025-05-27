use super::{
  Table, GenericError, PolicySchema, DatabaseNamespace,
  Policy, Uuid, PolicySerializer, Connection,
  UpdateStatement,
  generate_sql_initialize_table_given_columns_writer,
  generate_sql_insert_row,
  generate_sql_delete_where_2_columns
};

pub struct PoliciesSchema {
  table: Table,
  policy: PolicySchema,
}

impl PoliciesSchema {
  pub fn new(
    database_namespace: &DatabaseNamespace
  ) -> 
    Result<Self, GenericError> 
  {
    let table = database_namespace
      .create_table("policies")
      .map_err(|error| error.change_context("create PoliciesSchema"))?;

    let policy = PolicySchema::new(table.column_namespace())
      .map_err(|error| error.change_context("create PoliciesSchema"))?;

    Ok(Self {
      table,
      policy,
    })
  }

  pub fn generate_sql_initialize(
    &self,
    into: &mut String,
  ) -> Result<(), GenericError> {
    generate_sql_initialize_table_given_columns_writer(
      into,
      &self.table,
      &self.policy,
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
    let serializer = PolicySerializer::new(user_id, &self.policy);
    generate_sql_insert_row(into, &self.table, &serializer, policy)
      .map_err(|error| 
        error.change_context("generate sql code that inserts a policy")
      )
  }

  pub fn insert_policy(
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
      &self.policy.id,
      policy_id,
      &self.policy.user_id,
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

  pub fn create_policy_updater(
    &self,
    policy_id: &Uuid,
    user_id: &Uuid,
  ) -> 
    UpdateStatement
  {
    UpdateStatement::new_given_two_where_columns(
      &self.policy.id,
      policy_id, 
      &self.policy.user_id, 
      user_id,
    )
  }
}
