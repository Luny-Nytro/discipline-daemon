use std::path::PathBuf;
use super::implementation::*;
use crate::*;

pub struct Database {
  // pub(super) connection: rusqlite::Connection,
  pub user: UserCollection,

}

impl Database {
  // pub fn open<Schema>(
  //   database_directory_path: PathBuf,
  // ) -> 
  //   Result<Self, GenericError> 
  // where 
  //   Schema: IsTopLevelCompoundValueSchema
  // {

    // if database_directory_path.is_relative() {
    //   return Err(
    //     GenericError::new("open a database connection")
    //       .add_error("database directory path must be absolute")
    //       .add_attachment("database directory path", database_directory_path.to_string_lossy())
    //   )
    // }

    // let connection = rusqlite::Connection::open(database_directory_path.join("data.db")).map_err(|error| {
    //   GenericError::new("open a database connection")
    //     .add_error("the sqlite bindings crate, rusqlite, failed to open a sqlite database connection")
    //     .add_attachment("database directory path", database_directory_path.to_string_lossy())
    //     .add_attachment("error", error.to_string())
    // })?;

    // let mut schema_definer = TopLevelCompoundValueSchemaDefiner::new();
    // let schema = Schema::new(&mut schema_definer);

    // Ok(Database {
    //   connection,
    //   collections: Vec::new(),
    //   singleton_collection: todo!(),
    // })
  // }

  pub(super) fn execute(&self, code: &str) -> 
    Result<(), GenericError>
  {
    todo!()
    // self.connection.execute_batch(code).map_err(|error|
    //   GenericError::new("execute sql code")
    //     .add_attachment("error", error.to_string())
    //     .add_attachment("code", code)
    // )
  }

  // pub fn create_modifications_draft(&self) -> DatabaseModificationsDraft {
  //   todo!()
  // }

  // pub fn load_top_level_compound_value<
  //   SingletonSpecification,
  //   SingletonSerializer,
  //   SingletonDeserializer,
  // >(
  //   &self, 
  //   singleton_specification: &SingletonSpecification,
  //   singleton_serializer: &SingletonSerializer,
  //   singleton_deserializer: &SingletonDeserializer,
  // ) -> 
  //   Result<SingletonSpecification::CompoundValue, GenericError> 
  // where 
  //   SingletonSpecification: IsTopLevelCompoundValueSchema,
  //   SingletonSerializer: CompoundValueSerializer<CompoundValue = SingletonSpecification::CompoundValue>,
  //   SingletonDeserializer: CompoundValueDeserializer<CompoundValue = SingletonSpecification::CompoundValue>
  // {
  //   let code = "SELECT ALL FROM Singleton";
  //   let mut statement = self.connection.prepare(code).map_err(|error|
  //     GenericError::new("action")
  //   )?;
  //   let mut iterator = statement.query(()).map_err(|error|
  //     GenericError::new("action")
  //   )?;
  //   let row = iterator.next().map_err(|error|
  //     GenericError::new("action")
  //   )?;
  //   let Some(row) = row else {
  //     return self.initialize(
  //       singleton_specification,
  //       singleton_serializer,
  //     );
  //   };

  //   deserialize_compound_value(row, singleton_deserializer)
  // }

  // fn initialize<
  //   SingletonSpecification,
  //   SingletonSerializer,
  // >(
  //   &self, 
  //   singleton_specification: &SingletonSpecification,
  //   singleton_serializer: &SingletonSerializer,
  // ) -> 
  //   Result<SingletonSpecification::CompoundValue, GenericError> 
  // where 
  //   SingletonSpecification: IsTopLevelCompoundValueSchema,
  //   SingletonSerializer: CompoundValueSerializer<CompoundValue = SingletonSpecification::CompoundValue>
  // {
  //   let mut code  = String::new();
  //   generate_code_define_collection(&mut code, &self.singleton_collection);

  //   for collection in &self.collections {
  //     generate_code_define_collection(&mut code, collection);
  //   }

  //   let initial_singleton_instance = singleton_specification.create_initial_instance();

  //   generate_code_add_collection_item(
  //     &mut code, 
  //     &self.singleton_collection, 
  //     singleton_serializer, 
  //     &initial_singleton_instance,
  //   )?;

  //   self.execute(&code)?;

  //   Ok(initial_singleton_instance)
  // }
}