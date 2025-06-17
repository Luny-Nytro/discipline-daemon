use super::*;

pub struct Namespace {
  pub(super) path: DatabaseEntityPath,
}

impl Namespace {
  pub fn define_namespace(
    &mut self, 
    database: &mut Database,
    new_namespace_identifier: &str,
  ) -> 
    Result<Namespace, GenericError> 
  {
    // TODO: check whether there is a namespace with the given identifier

    self.path.then(new_namespace_identifier)
      .map(|path| 
        Namespace { 
          path 
        }
      )
      .map_err(|error|
        error
          .change_context("creating a new namespace within a non-global namespace")
          .add_error("invalid namespace identifier")
          .add_attachment("super namespace fully qualified identifier", self.path.as_str())
      )
  }

  pub fn define_collection(
    &mut self, 
    database: &mut Database,
    collection_identifier: &str,
    collection_item_namespace: CompoundTypeNamespace,
  ) -> 
    Result<CollectionSpecification, GenericError> 
  {
    // TODO: check if there is a collection or a namespace with the given identifier
    self.path.then(collection_identifier)
      .map_err(|error|
        error
          .change_context("definning a new collection")
          .add_error("invalid collection identifier")
          .add_attachment("namespace path", self.path.as_str())
      )
      .and_then(|collection_path|
        CollectionSpecification::new(
          collection_path,
          collection_item_namespace,
        )
      )
  } 
}