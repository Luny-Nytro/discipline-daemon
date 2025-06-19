use super::*;

pub struct Collection {
  pub(super) path: DatabaseEntityPath,
  pub(super) collection_item_namespace: CompoundTypeNamespace,
}

impl Collection {
  pub(super) fn new(
    path: DatabaseEntityPath,
    collection_item_namespace: CompoundTypeNamespace,
  ) -> 
    Result<Self, GenericError>
  {
    if collection_item_namespace.columns.is_empty() {
      return Err(
        GenericError::new("creating a collection")
          .add_error("you didn't define any fields for the collection item! define one or more by calling the 'define_*_field' methods of the CollectionItemDefiner")
          .add_attachment("collection path", path.as_str())
      );
    }
    if collection_item_namespace.primary_columns_number == 0 {
      return Err(
        GenericError::new("creating a collection")
          .add_error("you didn't define any primary fields for the collection item! define one or more by calling the 'define_primary_scalar_field' method of the CollectionItemDefiner")
          .add_attachment("collection path", path.as_str())
      );
    }

    Ok(Self {
      path,
      collection_item_namespace,
    })
  }
}
