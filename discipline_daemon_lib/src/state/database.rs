use crate::database::{
  CompoundValueDeserializer, CompoundValueSerializer, 
  Database, CompoundValueDeserializerContext, CompoundValueSerializerContext, 
  CollectionItemMatcher, IsTopLevelCompoundValueSchema, TopLevelCompoundValueSchemaDefiner,
};

use crate::{
  GenericError, user, 
  user_screen_access_regulation, State
};

pub struct StateSpecification {
  user_screen_access_regulation_common_info: user_screen_access_regulation::database::CommonInfoSpecification,
  user_module: user::database::UserModule,
}

impl IsTopLevelCompoundValueSchema for StateSpecification {
  type CompoundValue = NormalizedState;

  fn new(definer: &mut TopLevelCompoundValueSchemaDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      user_screen_access_regulation_common_info: definer.compound_field("UserScreenAccessRegulation")?,
      user_module: definer.module("Users")?,
    })
  }

  fn display_name(&self) -> &str {
    "DisciplineState"
  }

  fn create_initial_instance(&self) -> Self::CompoundValue {
    NormalizedState::default()    
  }
}

impl StateSpecification {
  pub fn user_module(&self) -> &user::database::UserModule {
    &self.user_module
  }

  pub fn user_screen_access_regulator(&self) -> &user_screen_access_regulation::database::RegulatorSpecification {
    self.user_module.user_screen_access_regulator()
  }

  pub fn user_screen_access_regulation_common_info(&self) -> &user_screen_access_regulation::database::CommonInfoSpecification {
    &self.user_screen_access_regulation_common_info
  }
}

impl CompoundValueSerializer for StateSpecification {
  type CompoundValue = State;

  fn serialize_into(
    &self, 
    value: &Self::CompoundValue,
    context: &mut CompoundValueSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_compound(&self.user_screen_access_regulation_common_info, &value.user_screen_access_regulation_common_info)
  }
}

pub struct NormalizedState {
  user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo,
}

impl Default for NormalizedState {
  fn default() -> Self {
    Self {
      // id: ITEM_ID,
      user_screen_access_regulation_common_info: user_screen_access_regulation::CommonInfo::default(),
    }
  }
}

impl CompoundValueDeserializer for StateSpecification {
  type CompoundValue = NormalizedState;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::CompoundValue, GenericError> {
    Ok(NormalizedState {
      user_screen_access_regulation_common_info: context.deserialize_compound(&self.user_screen_access_regulation_common_info)?,
    })
  }
}

impl StateSpecification {
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

  pub fn load(
    &self, 
    database: &Database,
  ) -> 
    Result<State, GenericError> 
  {
    database.load_top_level_compound_value(
      self, 
      , singleton_deserializer)
    let users = database.find_all_collection_items(
      &self.user_module.collection, 
      &self.user_module,
    ).map_err(|error| 
      error
        .change_context("loading all users from the database")
        .change_context("loading daemon state")
    )?;
    
    let user_screen_access_regulation_policies = database.find_all_collection_items(
      &self.user_screen_access_regulation_common_info.policy.collection_specification, 
      &self.user_screen_access_regulation_common_info.policy,
    ).map_err(|error| 
      error
        .change_context("loading all user screen access regulation policies from the database in normalized form")
        .change_context("loading daemon state")
    )?;

        
    let user_screen_access_regulation_rules = database.find_all_collection_items(
      &self.user_screen_access_regulation_common_info.rule.collection, 
      &self.user_screen_access_regulation_common_info.rule,
    ).map_err(|error| 
      error
        .change_context("loading all user screen access regulation rules from the database in normalized form")
        .change_context("loading daemon state")
    )?;

    let matcher = CollectionItemMatcher::match_by_scalar_field(
      &self.id, 
      &ITEM_ID,
    ).map_err(|error| 
      error
        .change_context("creating a matcher that matches the single daemon state item in its singleton collection")
        .change_context("loading daemon state")
    )?;
    
    let state = database.find_one_collection_item(
      &self.singleton_collection, 
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