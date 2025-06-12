use crate::database::{
  ScalarFieldSpecification, CompoundValueDeserializer, CompoundValueSerializer, 
  GlobalNamespace, CompoundValueDeserializerContext, CompoundValueSerializerContext, 
  CollectionSpecification, CollectionItemFieldsNamespace, Database,
  CollectionItemMatcher,
};

use crate::{
  GenericError, user, 
  user_screen_access_regulation, State
};

pub struct Specification {
  collection_specification: CollectionSpecification,
  id_field_specification: ScalarFieldSpecification,
  pub user_specification: user::database::Specification,
  pub user_screen_access_regulation: user_screen_access_regulation::database::Specification,
  // pub user_screen_access_regulation_policy: user_screen_access_regulation::database::PolicySchema,
  // pub user_screen_access_regulation_rule: user_screen_access_regulation::database::RuleSchema,
}

impl Specification {
  pub fn new(namespace: &mut GlobalNamespace) -> Result<Self, GenericError> {
    let mut fields_namespace = CollectionItemFieldsNamespace::new();

    let id_column = fields_namespace
      .primary_scalar_field_specification("Id")
      .build()
      .map_err(|error| error.change_context("creating Specification"))?;

    
    let user = user::database::Specification::new(
      &mut namespace.namespace("user")?,
    )?;


    let user_screen_access_regulation = user_screen_access_regulation
      ::database
      ::Specification
      ::new(
        &mut namespace.namespace("UserScreenAccessRegulation")?, 
        &mut fields_namespace.compound_field_specification("UserScreenAccessRegulation")?
      )?;

    let collection_specification = namespace
      .collection("app", fields_namespace)
      .map_err(|error| error.change_context("creating Specification"))?;

    Ok(Specification {
      collection_specification, 
      id_field_specification: id_column, 
      user_specification: user,
      user_screen_access_regulation,
    })
  }
}

impl CompoundValueSerializer for Specification {
  type CompoundValue = State;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.id_field_specification, &ITEM_ID)?;
    context.serializable_compound(self.user_screen_access_regulation.singleton(), &value.user_screen_access_regulation_common_info)
  }
}

pub struct NormalizedState {
  id: u8,
  user_access: user_screen_access_regulation::CommonInfo,
}

const ITEM_ID: u8 = 0;

impl Default for NormalizedState {
  fn default() -> Self {
    Self {
      id: ITEM_ID,
      user_access: user_screen_access_regulation::CommonInfo::default(),
    }
  }
}

impl CompoundValueDeserializer for Specification {
  type Output = NormalizedState;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormalizedState {
      id: context.deserializable_scalar(&self.id_field_specification)?,
      user_access: context.deserialize_compound(self.user_screen_access_regulation.singleton())?,
    })
  }
}

impl Specification {
  // pub fn generate_sql_initialize(
  //   &self, 
  //   into: &mut String,
  // ) ->
  //   Result<(), GenericError>
  // {
  //   let mut statement = InitializeTableStatement::new(into, &self.collection_specification);
  //   statement
  //     .add_compound_type(self)
  //     .map_err(|error| 
  //       error
  //         .change_context("generate sql code that initializes the daemon state singleton table")
  //     )?;

  //   self
  //     .user_screen_access_regulation
  //     .generate_sql_initialize(into)
  //     .map_err(|error| 
  //       error
  //         .change_context("generate sql code that initializes the daemon state singleton table")
  //     )?;

  //   Ok(())
  // }

  // pub fn initialize(
  //   &self,
  //   connection: &Database,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   let mut code = String::new();
  //   self
  //     .generate_sql_initialize(&mut code)
  //     .map_err(|error|
  //       error.change_context("initialize database schema")
  //     )
  // }
  
  // fn load_normalized_state(
  //   &self,
  //   connection: &Connection,
  // ) -> 
  //   Result<NormaizedState, GenericError>
  // {
  //   connection.find_some_row(
  //     &self.collection_specification, 
  //     self,
  //   )
  // }

  fn initialize(&self, database: &Database) -> Result<State, GenericError> {
    let default_state = State {
      users: Vec::new(),
      user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo::default(),
    };

    database.add_collection_item(
      &self.collection_specification, 
      self,
      &default_state, 

    ).map_err(|error| 
      error
        .change_context("adding the default daemon state item to the collection")
    )?;

    Ok(default_state)
  }

  pub fn load(
    &self, 
    database: &Database,
  ) -> 
    Result<State, GenericError> 
  {
    let users = database.find_all_collection_items(
      &self.user_specification.collection_specification, 
      &self.user_specification,
    ).map_err(|error| 
      error
        .change_context("loading all users from the database")
        .change_context("loading daemon state")
    )?;
    
    let user_screen_access_regulation_policies = database.find_all_collection_items(
      &self.user_screen_access_regulation.policy.collection_specification, 
      &self.user_screen_access_regulation.policy,
    ).map_err(|error| 
      error
        .change_context("loading all user screen access regulation policies from the database in normalized form")
        .change_context("loading daemon state")
    )?;

        
    let user_screen_access_regulation_rules = database.find_all_collection_items(
      &self.user_screen_access_regulation.rule.collection_specification, 
      &self.user_screen_access_regulation.rule,
    ).map_err(|error| 
      error
        .change_context("loading all user screen access regulation rules from the database in normalized form")
        .change_context("loading daemon state")
    )?;

    let matcher = CollectionItemMatcher::match_by_scalar_field(
      &self.id_field_specification, 
      &ITEM_ID,
    ).map_err(|error| 
      error
        .change_context("creating a matcher that matches the single daemon state item in its singleton collection")
        .change_context("loading daemon state")
    )?;
    
    let state = database.find_one_collection_item(
      &self.collection_specification, 
      &matcher, 
      self,
    ).map_err(|error| 
      error
        .change_context("loading the single daemon state item in its singleton collection")
        .change_context("loading daemon state")
    )?;

    let Some(state) = state else {
      return self.initialize(database).map_err(|error|
        error.change_context("loading daemon state")
      );
    };

    let users = users
      .into_iter()
      .map(|user| user.denormalize(
        &user_screen_access_regulation_policies, 
        &user_screen_access_regulation_rules,
      ))
      .collect();

    let state = State {
      users,
      user_screen_access_regulation_common_info: state.user_access,
    };

    Ok(state)
  }

}