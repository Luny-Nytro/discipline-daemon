use std::collections::HashSet;
use super::*;



pub struct Collection {
  path: Path,
  pub(super) columns: Vec<Column>,
  pub(super) identifier: Identifier,
  pub(super) primary_columns_number: usize,
}

impl Collection {
  pub(super) fn path(&self) -> &Path {
    &self.path
  }
}

pub trait IsCollection {
  
}

pub trait IsNamespace: Sized {
  fn new(definer: &mut DatabaseDefiner) -> Result<Self, GenericError>;
}

pub struct DatabaseDefiner {
  path: Path,
  collections: Vec<Collection>,
  defined_identifiers: HashSet<Identifier>,
}

impl DatabaseDefiner {
  pub fn namespace<T>(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<T, GenericError>
  where 
    T: IsNamespace
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let mut definer = DatabaseDefiner { 
      path: self.path.append_identifier(&identifier),
      collections: Vec::new(),
      defined_identifiers: HashSet::new(),
    };

    let namespace = T::new(&mut definer)?;

    self.collections.extend(definer.collections.into_iter());
    self.defined_identifiers.insert(identifier);

    Ok(namespace)
  }

  pub fn collection<T>(&mut self, identifier: &str) -> Result<(Collection, T), GenericError>
  where 
    T: IsCollectionItem
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let mut collection_item_definer = CollectionItemDefiner {
      path: Path::new(),
      columns: Vec::new(),
      defined_identifiers: HashSet::new(),
      primary_columns_number: 0,
    };

    let collection_item = T::new(&mut collection_item_definer)?;
    self.defined_identifiers.insert(identifier.clone());

    Ok((
      Collection {
        path: self.path.append_identifier(&identifier),
        identifier,
        columns: collection_item_definer.columns,
        primary_columns_number: collection_item_definer.primary_columns_number,
      }, 
      
      collection_item,
    ))
  }
}

// Examoles
pub struct CompoundTypeExample {
  a: Field,
  b: Field,
  c: Field,
  d: Field,
}

impl IsCompoundType for CompoundTypeExample {
  fn new(definer: &mut CompoundTypeDefiner) -> Tried<Self, GenericError> {
    Ok(Self {
      a: definer.readonly_required_field("a")?,
      b: definer.readonly_required_field("b")?,
      c: definer.readonly_required_field("c")?,
      d: definer.readonly_required_field("d")?,
    })
  }
}

pub struct CollectionItemExample {
  id: Field,
  name: Field,
  compound_field: CompoundTypeExample,
}

impl IsCollectionItem for CollectionItemExample {
  fn new(definer: &mut CollectionItemDefiner) -> Result<Self, GenericError> {
    Ok(Self {
      id: definer.primary_scalar_field("id")?,
      name: definer.writable_required_field("name")?,
      compound_field: definer.compound_field("lunar")?,
    })
  }
}

pub struct NamespaceExample {
  rule_collection: Collection,
  rule_collection_item: CollectionItemExample,
}

impl IsNamespace for NamespaceExample {
  fn new(definer: &mut DatabaseDefiner) -> Result<Self, GenericError> {
    let (rule_collection, rule_collection_item) = definer.collection("rules")?;

    Ok(Self {
      rule_collection,
      rule_collection_item,
    })
  }
}



// pub fn find_all_collection_items<Deserializer>(
//   &self,
//   collection_specification: &Collection,
//   collection_item_deserializer: &Deserializer,
// ) ->
//   Result<Vec<Deserializer::Output>, GenericError>
// where 
//   Deserializer: CompoundValueDeserializer
// {
//   let mut code = String::new();
//   generate_code_find_all_collection_items(
//     &mut code, 
//     collection_specification,
//   ).map_err(|error|
//     error
//       .change_context("retrieving all the items of a collection")
//   )?;

//   let mut statement = self.connection.prepare(&code).map_err(|error|
//     GenericError::new("creating sqlite query statement")
//       .add_attachment("error", error.to_string())
//       .add_attachment("code", code.clone())
//       .change_context("retrieving all the items of a collection")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//   )?;
  
//   let mut iterator = statement.query(()).map_err(|error|
//     GenericError::new("creating sqlite iterator")
//       .add_attachment("error", error.to_string())
//       .add_attachment("code", code.clone())
//       .change_context("retrieving all the items of a collection")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//   )?;

//   let mut collection_items = Vec::new();
//   loop {
//     let raw_collection_item = iterator.next().map_err(|error|
//       GenericError::new("getting the next item of a sqlite row iterator")
//       .add_attachment("error", error.to_string())
//       .add_attachment("code", code.clone())
//       .change_context("retrieving all the items of a collection")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//     )?;

//     let Some(raw_collection_item) = raw_collection_item else {
//       break;
//     };

//     let collection_item = deserialize_compound_value(
//       raw_collection_item, 
//       collection_item_deserializer,
//     ).map_err(|error|
//       error
//         .change_context("deserializing a collection item")
//         .change_context("retrieving all the items of a collection")
//         .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//     )?;

//     collection_items.push(collection_item);
//   }

//   Ok(collection_items)
// }

// pub fn find_one_collection_item<Deserializer>(
//   &self,
//   collection_specification: &Collection,
//   collection_item_matcher: &CollectionItemMatcher,
//   collection_item_deserializer: &Deserializer,
// ) ->
//   Result<Option<Deserializer::Output>, GenericError>
// where 
//   Deserializer: CompoundValueDeserializer
// {
//   let mut code = String::new();
//   generate_code_find_one_collection_item(
//     &mut code,
//     collection_specification,
//     collection_item_matcher,
//   ).map_err(|error|
//     error.change_context("retrieving one collection item")
//   )?;

//   let mut statement = self.connection.prepare(&code).map_err(|error|
//     GenericError::new("creating sqlite query statement")
//       .add_attachment("error", error.to_string())
//       .add_attachment("code", code.clone())
//       .change_context("retreive one collection item")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//   )?;
  
//   let mut iterator = statement.query(()).map_err(|error|
//     GenericError::new("create sqlite query iterator")
//       .add_attachment("error", error.to_string())
//       .add_attachment("code", code.clone())
//       .change_context("retreive one collection item")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//   )?;

//   loop {
//     let raw_collection_item = iterator.next().map_err(|error|
//       GenericError::new("getting the next item of a sqlite query iterator")
//         .add_attachment("error", error.to_string())
//         .add_attachment("code", code.clone())
//         .change_context("retreive one collection item")
//         .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//     )?;

//     let Some(raw_collection_item) = raw_collection_item else {
//       return Ok(None)
//     };

//     return deserialize_compound_value(
//       raw_collection_item, 
//       collection_item_deserializer,
//     )
//     .map(Some)
//     .map_err(|error|
//       error
//         .change_context("retrieving one collection item")
//         .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//     );
//   }
// }

// pub fn update_collection_items(
//   &self,
//   collection_specification: &Collection,
//   collection_item_matcher: &CollectionItemMatcher,
//   collection_item_modifications: &CollectionItemModificationsDraft,
// ) -> 
//   Result<(), GenericError> 
// {
//   let mut code = String::new();
//   generate_code_update_collection_item(
//     &mut code, 
//     collection_specification, 
//     collection_item_matcher, 
//     collection_item_modifications,
//   ).map_err(|error|
//     error.change_context("updating collection items")
//   )?;

//   self.execute(&code).map_err(|error| 
//     error
//       .change_context("updating collection items")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//   )
// }

// pub fn delete_collection_items(
//   &self,
//   collection_specification: &Collection,
//   collection_item_matcher: &CollectionItemMatcher,
// ) -> 
//   Result<(), GenericError> 
// {
//   let mut code = String::new();
//   generate_code_delete_collection_item(
//     &mut code, 
//     collection_specification, 
//     collection_item_matcher, 
//   ).map_err(|error|
//     error.change_context("deleting collection items")
//   )?;

//   self.execute(&code).map_err(|error| 
//     error
//       .change_context("deleting collection items")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//   )
// }

// pub fn add_collection_item<Serializer: CompoundTypeSerializer>(
//   &self,
//   collection_specification: &Collection,
//   collection_item_serializer: &Serializer,
//   new_collection_item: &Serializer::CompoundType,
// ) -> 
//   Result<(), GenericError> 
// {
//   let mut code = String::new();
//   generate_code_add_collection_item(
//     &mut code, 
//     collection_specification, 
//     collection_item_serializer,
//     new_collection_item, 
//   ).map_err(|error|
//     error.change_context("adding a new collection item")
//   )?;

//   self.execute(&code).map_err(|error| 
//     error
//       .change_context("adding collection a new item")
//       .add_attachment("collection fully qualified identifier", collection_specification.path.as_string())
//   )
// }

// // pub fn initialize_database_schema(
// //   &self,
// //   database_specifications_provider: &impl DatabaseSpecificationsProvider,
// // ) -> 
// //   Result<(), GenericError>
// // {
// //   let mut code = String::new();
// //   generate_code_define_database_schema(
// //     &mut code, 
// //     database_specifications_provider,
// //   )
// //   .and_then(|_|
// //     self.execute(&code)
// //   )
// //   .map_err(|error|
// //     error.change_context("initializing database schema")
// //   )
// // }

// pub fn define_namespace(
//   &mut self, 
//   identifier: &str,
// ) -> 
//   Result<DatabaseNamespace, GenericError> 
// {
//   DatabaseEntityPath::new(identifier)
//     .map(|path| DatabaseNamespace::new(path))
//     .map_err(|error|
//       error
//         .change_context("definning a new namespace")
//         .add_error("invalid namespace identifier")
//     )
// }