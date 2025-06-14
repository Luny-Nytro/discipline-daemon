use super::*;

pub struct Namespace {
  pub(super) identifier: String,
  pub(super) fully_qualified_identifier: String,
}

impl Namespace {
  pub fn namespace(&mut self, identifier: &str) -> Result<Namespace, GenericError> {
    verify_identifier(identifier)
      .map(|_| 
        Namespace { 
          identifier: identifier.into(), 
          fully_qualified_identifier: format!("{}_{}", self.fully_qualified_identifier, identifier),
        }
      )
      // TODO: Do proper error handling
  }

  pub fn collection(
    &mut self, 
    collection_identifier: &str,
    collection_item_fields_namespace: CollectionItemFieldsNamespace,
  ) -> 
    Result<CollectionSpecification, GenericError> 
  {
    todo!()
    // TODO: do proper error handling

    // if let Err(error) = verify_identifier(collection_identifier) {
    //   return Err(error);
    // }

    // if collection_item_fields_namespace.column_specifications.is_empty() {
    //   return Err(todo!());
    // }

    // Ok(CollectionSpecification::new(
    //   collection_identifier.into(), 
    //   format!("{}_{}", self.fully_qualified_identifier, collection_identifier), 
    //   collection_item_fields_namespace.column_specifications,
    // ))
  } 
}