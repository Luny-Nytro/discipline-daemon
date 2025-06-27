use crate::database::{
  Field, CompoundValueDeserializer, CompoundTypeSerializer, 
  Database, CompoundValueDeserializerContext, CompoundTypeSerializerContext, 
  CollectionSpecification, CollectionItemDefiner, DatabaseNamespace,
  CollectionItemMatcher, CompoundTypeNamespace,
};

use crate::{
  GenericError, user, 
  user_screen_access_regulation, State
};

pub struct Specification {
  collection: CollectionSpecification,
  id: Field,
  pub user: user::database::UserCollection,
  pub user_screen_access_regulation: user_screen_access_regulation::database::Module,
}

impl Specification {
  pub fn new(
    database: &mut Database,
    database_namespace: &mut DatabaseNamespace,
  ) -> 
    Result<Self, GenericError> 
  {
    let mut soleton_namespace = CompoundTypeNamespace::new();
    let mut soleton_definer = CollectionItemDefiner::new();

    let id = soleton_definer
      .define_primary_scalar_field(&mut soleton_namespace, "Id")?;

    let mut user_namespace = database_namespace.define_namespace(
      database, 
      "User",
    )?;

    let user = user::database::UserCollection::new(
      database, 
      &mut user_namespace,
    );

    let user_screen_access_regulation_definer = soleton_definer
      .define_required_writable_compound_field(
        &mut soleton_namespace, 
        "UserScreenAccessRegulation"
      )?;

    let user_screen_access_regulation = user_screen_access_regulation
      ::database::Module::new(
        database, 
        database_namespace, 
        &mut soleton_namespace, 
        &mut user_screen_access_regulation_definer,
      )?;

    let collection_specification = database_namespace
      .collection("app", soleton_namespace)
      .map_err(|error| error.change_context("creating Specification"))?;

    Ok(Specification {
      collection: collection_specification, 
      id, 
      user,
      user_screen_access_regulation,
    })
  }
}

impl CompoundTypeSerializer for Specification {
  type CompoundType = State;

  fn serialize_into(
    &self, 
    value: &Self::CompoundType,
    context: &mut CompoundTypeSerializerContext, 
  ) ->
    Result<(), GenericError>
  {
    context.serializable_scalar(&self.id, &ITEM_ID)?;
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
      id: context.deserializable_scalar(&self.id)?,
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
      &self.collection, 
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
      &self.user.collection, 
      &self.user,
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
      &self.user_screen_access_regulation.rule.collection, 
      &self.user_screen_access_regulation.rule,
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
      &self.collection, 
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