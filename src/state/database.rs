use crate::database::{
  ScalarFieldSpecification, CompoundValueDeserializer, CompoundValueSerializer, 
  GlobalNamespace, CompoundValueDeserializerContext, CompoundValueSerializerContext, 
  CollectionSpecfication, CollectionItemFieldsScope
};

use crate::{
  GenericError, user, 
  user_screen_access_regulation, State
};

pub struct Specification {
  collection_specification: CollectionSpecfication,
  id_field_specification: ScalarFieldSpecification,
  user_specification: user::database::Specification,
  pub user_screen_access_regulation: user_screen_access_regulation::database::Specification,
  // pub user_screen_access_regulation_policy: user_screen_access_regulation::database::PolicySchema,
  // pub user_screen_access_regulation_rule: user_screen_access_regulation::database::RuleSchema,
}

impl Specification {
  pub fn new(namespace: &mut GlobalNamespace) -> Result<Self, GenericError> {
    let mut fields_namespace = CollectionItemFieldsScope::new();

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
    context.serializable_scalar(&self.id_field_specification, &ROW_ID)?;
    context.serializable_compound(self.user_screen_access_regulation.singleton(), &value.user_screen_access_regulation_common_info)
  }
}

pub struct NormaizedState {
  id: u8,
  user_access: user_screen_access_regulation::CommonInfo,
}

const ROW_ID: u8 = 0;

impl Default for NormaizedState {
  fn default() -> Self {
    Self {
      id: ROW_ID,
      user_access: user_screen_access_regulation::CommonInfo::default(),
    }
  }
}

impl CompoundValueDeserializer for Specification {
  type Output = NormaizedState;

  fn deserialize(&self, context: &CompoundValueDeserializerContext) -> Result<Self::Output, GenericError> {
    Ok(NormaizedState {
      id: context.deserializable_scalar(&self.id_field_specification)?,
      user_access: context.deserialize_compound(self.user_screen_access_regulation.singleton())?,
    })
  }
}

// impl Specification {
//   pub fn generate_sql_initialize(
//     &self, 
//     into: &mut String,
//   ) ->
//     Result<(), GenericError>
//   {
//     let mut statement = InitializeTableStatement::new(into, &self.collection_specification);
//     statement
//       .add_compound_type(self)
//       .map_err(|error| 
//         error
//           .change_context("generate sql code that initializes the app state singleton table")
//       )?;

//     self
//       .user_screen_access_regulation
//       .generate_sql_initialize(into)
//       .map_err(|error| 
//         error
//           .change_context("generate sql code that initializes the app state singleton table")
//       )?;

//     Ok(())
//   }

//   pub fn initialize(
//     &self,
//     connection: &Connection
//   ) -> 
//     Result<(), GenericError>
//   {
//     let mut code = String::new();
//     self
//       .generate_sql_initialize(&mut code)
//       .map_err(|error|
//         error.change_context("initialize database schema")
//       )
//   }
  
//   fn load_normalized_state(
//     &self,
//     connection: &Connection,
//   ) -> 
//     Result<NormaizedState, GenericError>
//   {
//     connection.find_some_row(
//       &self.collection_specification, 
//       self,
//     )
//   }

//   fn load_denormalized_state(
//     &self, 
//     connection: &Connection,
//   ) -> 
//     Result<State, GenericError> 
//   {
//     let users_in_normalized_form = self
//       .user_specification
//       .retrieve_all_normalized(connection)
//       .map_err(|error| 
//         error
//           .change_context("load all users from the database")
//           .change_context("load denormalized state")
//       )?;
    
//     let user_screen_access_regulation_policies_in_normalized_form = self
//       .user_screen_access_regulation
//       .policy
//       .load_all_normalized_policies(connection)
//       .map_err(|error| 
//         error
//           .change_context("load all user screen access regulation policies from the database in normalized form")
//           .change_context("load denormalized state")
//       )?;

        
//     let user_screen_access_regulation_rules_in_normalized_form = self
//       .user_screen_access_regulation
//       .rule
//       .load_all_rules_normalized(connection)
//       .map_err(|error| 
//         error
//           .change_context("load all user screen access regulation rules from the database in normalized form")
//           .change_context("load denormalized state")
//       )?;

//     let state_in_normalized_form = self
//       .load_normalized_state(connection)
//       .map_err(|error| 
//         error
//           .change_context("load state from the database in normalized form")
//           .change_context("load denormalized state")
//       )?;

//     let users_in_denormalized_form = users_in_normalized_form
//       .into_iter()
//       .map(|user| user.denormalize(
//         &user_screen_access_regulation_policies_in_normalized_form, 
//         &user_screen_access_regulation_rules_in_normalized_form,
//       ))
//       .collect();

//     let denormalized_state = State {
//       users: users_in_denormalized_form,
//       user_screen_access_regulation_common_info: state_in_normalized_form.user_access,
//     };

//     Ok(denormalized_state)
//   }

//   pub fn load(
//     &self, 
//     connection: &Connection,
//   ) -> 
//     Result<State, GenericError>
//   {
//     self.load_denormalized_state(connection)
//   }
// }