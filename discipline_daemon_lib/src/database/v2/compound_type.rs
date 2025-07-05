use std::collections::HashSet;
use super::*;

pub trait IsCompoundType: Sized {
  fn new(definer: &mut CompoundTypeDefiner) -> Result<Self, GenericError>;
  fn display_name(&self) -> &str;
}

pub struct CompoundTypeDefiner {
  path: Path,
  columns: Vec<Column>,
  defined_identifiers: HashSet<Identifier>,
}

impl CompoundTypeDefiner {
  pub(super) fn new(path: Path) -> Self {
    Self {
      path,
      columns: Vec::new(),
      defined_identifiers: HashSet::new(),
    }
  }

  pub fn readonly_required_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field::new(
      self.path.append_identifier(&identifier),
      FieldSemantics::ReadonlyRequired,
      identifier.clone(),
    );

    self.columns.push(Column::required(field.path().clone()));
    self.defined_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn readonly_optional_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field::new(
      self.path.append_identifier(&identifier),
      FieldSemantics::ReadonlyOptional,
      identifier.clone(),
    );

    self.columns.push(Column::optional(field.path().clone()));
    self.defined_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn writable_required_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field::new(
      self.path.append_identifier(&identifier),
      FieldSemantics::WritableRequired,
      identifier.clone(),
    );

    self.columns.push(Column::required(field.path().clone()));
    self.defined_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn writable_optional_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field::new(
      self.path.append_identifier(&identifier),
      FieldSemantics::WrirableOptional,
      identifier.clone(),
    );

    self.columns.push(Column::optional(field.path().clone()));
    self.defined_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn compound_field<T>(&mut self, identifier: &str) -> Result<T, GenericError> 
    where 
      T: IsCompoundType
  {
    let identifier = Identifier::new(identifier)?;

    let mut builder = CompoundTypeDefiner {
      path: self.path.append_identifier(&identifier),
      columns: Vec::new(),
      defined_identifiers: HashSet::new(),
    };

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.columns.into_iter());
    self.defined_identifiers.insert(identifier);
    Ok(compound_field)
  }

  pub fn optional_compound_field<T>(&mut self, identifier: &str) -> Result<T, GenericError> 
    where 
      T: IsCompoundType
  {
    let identifier = Identifier::new(identifier)?;

    let mut builder = CompoundTypeDefiner {
      path: self.path.append_identifier(&identifier),
      columns: Vec::new(),
      defined_identifiers: HashSet::new(),
    };

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.columns.into_iter());
    self.defined_identifiers.insert(identifier);
    Ok(compound_field)
  }

  pub fn module<T>(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<T, GenericError>
  where 
    T: IsModule
  {
    todo!()
    // let identifier = Identifier::new(identifier)?;

    // if self.defined_identifiers.contains(&identifier) {
    //   return Err(GenericError::new(""));
    // }

    // let mut definer = ModuleDefiner { 
    //   path: self.path.append_identifier(&identifier),
    //   collections: Vec::new(),
    //   defined_identifiers: HashSet::new(),
    // };

    // let namespace = T::new(&mut definer)?;

    // self.collections.extend(definer.collections.into_iter());
    // self.defined_identifiers.insert(identifier);

    // Ok(namespace)
  }

  pub fn collection<CollectionItem>(
    &mut self, 
    identifier: &str,
  ) -> 
    Result<(Collection, CollectionItem), GenericError>
  where 
    CollectionItem: IsCollectionItem
  {
    todo!()
    // let identifier = Identifier::new(identifier)?;

    // if self.defined_identifiers.contains(&identifier) {
    //   return Err(GenericError::new(""));
    // }

    // let mut collection_item_definer = CollectionItemDefiner::new();

    // let collection_item = CollectionItem::new(
    //   &mut collection_item_definer,
    // )?;

    // let collection = Collection::new(
    //   self.path.append_identifier(&identifier),
    //   identifier.clone(),
    //   collection_item_definer,
    // );

    // self.defined_identifiers.insert(identifier.clone());

    // Ok((collection, collection_item))
  }

  pub(super) fn into_columns(self) -> Vec<Column> {
    self.columns
  }
}
