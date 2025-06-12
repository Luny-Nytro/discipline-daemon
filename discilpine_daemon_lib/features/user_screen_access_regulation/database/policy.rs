use super::{
  GenericError, ScalarFieldSpecification, PolicyEnablerSpecification, CollectionSpecification,
  Namespace, PolicyName, CompoundValueSerializer, CompoundValueDeserializer,
  Policy, Uuid, PolicyEnabler, CompoundValueDeserializerContext,
  CollectionItemFieldsNamespace, CollectionItemModificationsDraft, Database,
  CompoundValueSerializerContext, NormalizedRule, CollectionItemMatcher,
};

pub struct PolicySpecification {
  pub collection_specification: CollectionSpecification,
  pub id_field_specification: ScalarFieldSpecification,
  pub name_field_specification: ScalarFieldSpecification,
  pub enabler_field_specification: PolicyEnablerSpecification,
  pub user_id_field_specification: ScalarFieldSpecification,
  // pub position_field_specification: ScalarFieldSpecification,
}

impl PolicySpecification {
  pub fn new(
    namespace: &mut Namespace
  ) -> 
    Result<Self, GenericError>
  {
    let mut fields_namespace = CollectionItemFieldsNamespace::new();

    let id_field_specification = fields_namespace
      .primary_scalar_field_specification("Id")
      .build()
      .map_err(|error| error.change_context("creating PolicySpecification"))?;
    
    let name_field_specification = fields_namespace
      .scalar_field_specification("Name")
      .build()
      .map_err(|error| error.change_context("creating PolicySpecification"))?;
      
    let enabler_field_specification = PolicyEnablerSpecification
      ::new(&mut fields_namespace.compound_field_specification("Enabler")?)
      .map_err(|error| error.change_context("creating PolicySpecification"))?;
      
    let user_id_field_specification = fields_namespace
      .primary_scalar_field_specification("UserId")
      .build()
      .map_err(|error| error.change_context("creating PolicySpecification"))?;
      
    // let position_field_specification = fields_namespace
    //   .scalar_field_specification("Position")
    //   .build()
    //   .map_err(|error| error.change_context("creating PolicySpecification"))?;

    let collection_specification = namespace
      .collection("Policies", fields_namespace)
      .map_err(|error| error.change_context("creating PolicySpecification"))?;

    Ok(Self {
      // position_field_specification,
      enabler_field_specification,
      id_field_specification,
      name_field_specification,
      user_id_field_specification,
      collection_specification,
    })
  }

  pub fn update_name(
    &self,
    modifications: &mut CollectionItemModificationsDraft,
    new_value: &PolicyName,
  ) -> 
    Result<(), GenericError>
  {
    modifications.modify_scalar_field(&self.name_field_specification, new_value)
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
    // policy_position: usize, 
    policy_specification: &'a PolicySpecification,
  ) -> Self {
    Self {
      user_id,
      // policy_position,
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
    context.serializable_scalar(&self.policy_specification.id_field_specification, &value.id)?;
    context.serializable_scalar(&self.policy_specification.name_field_specification, &value.name)?;
    context.serializable_scalar(&self.policy_specification.user_id_field_specification, self.user_id)?;
    // context.serializable_scalar(&self.policy_specification.position_field_specification, &self.policy_position)?;
    context.serializable_compound(&self.policy_specification.enabler_field_specification, &value.enabler)
  }
}

impl CompoundValueDeserializer for PolicySpecification {
  type Output = NormalizedPolicy;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedPolicy {
      id: context.deserializable_scalar(&self.id_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedPolicy")
          .add_error("failed to deserialize the 'Id' field")
      )?,
      name: context.deserializable_scalar(&self.name_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedPolicy")
          .add_error("failed to deserialize the 'Name' field")
      )?,
      user_id: context.deserializable_scalar(&self.user_id_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedPolicy")
          .add_error("failed to deserialize the 'UserId' field")
      )?,
      enabler: context.deserialize_compound(&self.enabler_field_specification).map_err(|error|
        error
          .change_context("deserializing NormalizedPolicy")
          .add_error("failed to deserialize the 'Enabler' field")
      )?,
      // position: context.deserializable_scalar(&self.position_field_specification).map_err(|error|
      //   error
      //     .change_context("deserializing NormalizedPolicy")
      //     .add_error("failed to deserialize the 'Position' field")
      // )?,
    })
  }
}

#[derive(Debug, Clone)]
pub struct NormalizedPolicy {
  pub(super) id: Uuid,
  pub(super) name: PolicyName,
  pub(super) user_id: Uuid,
  // pub(super) position: usize,
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

impl PolicySpecification {
  pub fn create_modifications_draft(&self) -> CollectionItemModificationsDraft {
    CollectionItemModificationsDraft::new()
  }

  pub fn apply_modifications_draft(
    &self,
    database: &Database,
    modifications_draft: &CollectionItemModificationsDraft,
    user_id: &Uuid,
    policy_id: &Uuid,
  ) -> 
    Result<(), GenericError>
  {
    database.update_collection_items(
      &self.collection_specification, 
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .and_scalar_field_is(&self.id_field_specification, policy_id)?
        .and_scalar_field_is(&self.user_id_field_specification, user_id)?
        .finalize()?, 
      modifications_draft,
    )
  }
  // pub fn generate_sql_initialize(
  //   &self,
  //   into: &mut String,
  // ) -> Result<(), GenericError> {
  //   generate_sql_initialize_table_given_columns_writer(
  //     into,
  //     &self.collection_specification,
  //     self,
  //   )
  //   .map_err(|error| 
  //     error.change_context("generate sql code that initializes everything related to the policies table")
  //   )
  // }

  // pub fn generate_sql_insert_policy(
  //   &self,
  //   into: &mut String,
  //   policy: &Policy,
  //   user_id: &Uuid,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   let serializer = PolicySerializer::new(user_id, self);
  //   generate_sql_add_row(into, &self.collection_specification, &serializer, policy)
  //     .map_err(|error| 
  //       error.change_context("generate sql code that inserts a policy")
  //     )
  // }

  pub fn add_policy(
    &self,
    database: &Database,
    user_id: &Uuid,
    // policy_position: usize,
    policy: &Policy,
  ) -> 
    Result<(), GenericError>
  {
    database.add_collection_item(
      &self.collection_specification, 
      &PolicySerializer::new(user_id, self), 
      policy,
    )
  }

  // pub fn generate_sql_delete_policy(
  //   &self,
  //   into: &mut String,
  //   policy_id: &Uuid,
  //   user_id: &Uuid,
  // ) {
  //   generate_sql_delete_where_2_columns(
  //     into,
  //     &self.collection_specification,
  //     &self.id_field_specification,
  //     policy_id,
  //     &self.user_id_field_specification,
  //     user_id,
  //   )
  // }

  pub fn delete_policy(
    &self,
    database: &Database,
    policy_id: &Uuid,
    user_id: &Uuid,
  ) -> 
    Result<(), GenericError> 
  {
    database.delete_collection_items(
      &self.collection_specification, 
      &CollectionItemMatcher::match_by_multiple_scalar_fields()
        .and_scalar_field_is(&self.id_field_specification, policy_id)?
        .and_scalar_field_is(&self.user_id_field_specification, user_id)?
        .finalize()?
    )
  }

  // pub fn load_all_normalized_policies(
  //   &self,
  //   connection: &Connection,
  // ) -> 
  //   Result<Vec<NormalizedPolicy>, GenericError>
  // {
  //   connection
  //     .find_all_rows(&self.collection_specification, self)
  //     .map_err(|error| 
  //       error.change_context("retrieve all policies from the database in normalized form")
  //     )
  // }

  // pub fn create_updater(
  //   &self,
  //   policy_id: &Uuid,
  //   user_id: &Uuid,
  // ) -> 
  //   UpdateStatement
  // {
  //   UpdateStatement::new_given_two_where_columns(
  //     &self.id_field_specification,
  //     policy_id, 
  //     &self.user_id_field_specification, 
  //     user_id,
  //   )
  // }
}
