use std::collections::HashSet;
use super::*;

pub trait IsModule: Sized {
  fn new(definer: &mut ModuleDefiner) -> Result<Self, GenericError>;
}

pub struct ModuleDefiner {
  path: Path,
  collections: Vec<Collection>,
  defined_identifiers: HashSet<Identifier>,
}

impl ModuleDefiner {
  pub fn module<T>(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<T, GenericError>
  where 
    T: IsModule
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let mut definer = ModuleDefiner { 
      path: self.path.append_identifier(&identifier),
      collections: Vec::new(),
      defined_identifiers: HashSet::new(),
    };

    let namespace = T::new(&mut definer)?;

    self.collections.extend(definer.collections.into_iter());
    self.defined_identifiers.insert(identifier);

    Ok(namespace)
  }

  pub fn collection<CollectionItem>(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<(Collection, CollectionItem), GenericError>
  where 
    CollectionItem: IsCollectionItem
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let mut collection_item_definer = CollectionItemDefiner::new();

    let collection_item = CollectionItem::new(
      &mut collection_item_definer,
    )?;

    let collection = Collection::new(
      self.path.append_identifier(&identifier),
      identifier.clone(),
      collection_item_definer,
    );

    self.defined_identifiers.insert(identifier.clone());

    Ok((collection, collection_item))
  }
}

