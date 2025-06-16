use std::path::PathBuf;
use super::*;

pub struct Database {
  connection: rusqlite::Connection,
}

impl Database {
  pub fn open(database_directory_path: PathBuf) -> Result<Self, GenericError> {
    if database_directory_path.is_relative() {
      return Err(
        GenericError::new("open a database connection")
          .add_error("database directory path must be absolute")
          .add_attachment("database directory path", database_directory_path.to_string_lossy())
      )
    }

    match rusqlite::Connection::open(database_directory_path.join("data.db")) {
      Ok(connection) => {
        Ok(Database {
          connection
        })
      }

      Err(error) => {
        Err(
          GenericError::new("open a database connection")
            .add_error("the sqlite bindings crate, rusqlite, failed to open a sqlite database connection")
            .add_attachment("database directory path", database_directory_path.to_string_lossy())
            .add_attachment("error", error.to_string())
        )
      }
    }
  }

  fn execute(&self, code: &str) -> 
    Result<(), GenericError>
  {
    self.connection.execute_batch(code).map_err(|error|
      GenericError::new("execute sql code")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code)
    )
  }

  pub fn find_all_collection_items<Deserializer>(
    &self,
    collection_specification: &CollectionSpecification,
    collection_item_deserializer: &Deserializer,
  ) ->
    Result<Vec<Deserializer::Output>, GenericError>
  where 
    Deserializer: CompoundValueDeserializer
  {
    let mut code = String::new();
    generate_code_find_all_collection_items(
      &mut code, 
      collection_specification,
    ).map_err(|error|
      error
        .change_context("retrieving all the items of a collection")
    )?;

    let mut statement = self.connection.prepare(&code).map_err(|error|
      GenericError::new("creating sqlite query statement")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retrieving all the items of a collection")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
    )?;
    
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("creating sqlite iterator")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retrieving all the items of a collection")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
    )?;

    let mut collection_items = Vec::new();
    loop {
      let raw_collection_item = iterator.next().map_err(|error|
        GenericError::new("getting the next item of a sqlite row iterator")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retrieving all the items of a collection")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
      )?;

      let Some(raw_collection_item) = raw_collection_item else {
        break;
      };

      let collection_item = deserialize_compound_value(
        raw_collection_item, 
        collection_item_deserializer,
      ).map_err(|error|
        error
          .change_context("deserializing a collection item")
          .change_context("retrieving all the items of a collection")
          .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
      )?;

      collection_items.push(collection_item);
    }

    Ok(collection_items)
  }

  pub fn find_one_collection_item<Deserializer>(
    &self,
    collection_specification: &CollectionSpecification,
    collection_item_matcher: &CollectionItemMatcher,
    collection_item_deserializer: &Deserializer,
  ) ->
    Result<Option<Deserializer::Output>, GenericError>
  where 
    Deserializer: CompoundValueDeserializer
  {
    let mut code = String::new();
    generate_code_find_one_collection_item(
      &mut code,
      collection_specification,
      collection_item_matcher,
    ).map_err(|error|
      error.change_context("retrieving one collection item")
    )?;

    let mut statement = self.connection.prepare(&code).map_err(|error|
      GenericError::new("creating sqlite query statement")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retreive one collection item")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
    )?;
    
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("create sqlite query iterator")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retreive one collection item")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
    )?;

    loop {
      let raw_collection_item = iterator.next().map_err(|error|
        GenericError::new("getting the next item of a sqlite query iterator")
          .add_attachment("error", error.to_string())
          .add_attachment("code", code.clone())
          .change_context("retreive one collection item")
          .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
      )?;

      let Some(raw_collection_item) = raw_collection_item else {
        return Ok(None)
      };

      return deserialize_compound_value(
        raw_collection_item, 
        collection_item_deserializer,
      )
      .map(Some)
      .map_err(|error|
        error
          .change_context("retrieving one collection item")
          .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
      );
    }
  }

  pub fn update_collection_items(
    &self,
    collection_specification: &CollectionSpecification,
    collection_item_matcher: &CollectionItemMatcher,
    collection_item_modifications: &CollectionItemModificationsDraft,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
    generate_code_update_collection_item(
      &mut code, 
      collection_specification, 
      collection_item_matcher, 
      collection_item_modifications,
    ).map_err(|error|
      error.change_context("updating collection items")
    )?;

    self.execute(&code).map_err(|error| 
      error
        .change_context("updating collection items")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
    )
  }

  pub fn delete_collection_items(
    &self,
    collection_specification: &CollectionSpecification,
    collection_item_matcher: &CollectionItemMatcher,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
    generate_code_delete_collection_item(
      &mut code, 
      collection_specification, 
      collection_item_matcher, 
    ).map_err(|error|
      error.change_context("deleting collection items")
    )?;

    self.execute(&code).map_err(|error| 
      error
        .change_context("deleting collection items")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
    )
  }

  pub fn add_collection_item<Serializer: CompoundValueSerializer>(
    &self,
    collection_specification: &CollectionSpecification,
    collection_item_serializer: &Serializer,
    new_collection_item: &Serializer::CompoundValue,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
    generate_code_add_collection_item(
      &mut code, 
      collection_specification, 
      collection_item_serializer,
      new_collection_item, 
    ).map_err(|error|
      error.change_context("adding a new collection item")
    )?;

    self.execute(&code).map_err(|error| 
      error
        .change_context("adding collection a new item")
        .add_attachment("collection fully qualified identifier", collection_specification.path.clone())
    )
  }

  pub fn initialize_database_schema(
    &self,
    database_specifications_provider: &impl DatabaseSpecificationsProvider,
  ) -> 
    Result<(), GenericError>
  {
    let mut code = String::new();
    generate_code_define_database_schema(
      &mut code, 
      database_specifications_provider,
    )
    .and_then(|_|
      self.execute(&code)
    )
    .map_err(|error|
      error.change_context("initializing database schema")
    )
  }


  pub fn namespace(&mut self) -> &mut GlobalNamespace {
    todo!()
    // verify_identifier("main")
    //   .map(|_|
    //     GlobalNamespace {
    //       identifier: "main".into(),
    //       fully_qualified_identifier: "main".into(),
    //     }
    //   )
    // TODO: do proper error handling
  }
}