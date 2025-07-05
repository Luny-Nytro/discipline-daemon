use std::collections::HashSet;
use super::*;

pub trait IsCollectionItem: Sized {
  fn new(definer: &mut CollectionItemDefiner) -> Result<Self, GenericError>;
  fn display_name(&self) -> &str;
}

pub struct CollectionItemDefiner {
  path: Path,
  pub(super) columns: Vec<Column>,
  defined_identifiers: HashSet<Identifier>,
  pub(super) primary_columns_number: usize,
}

impl CollectionItemDefiner {
  pub(super) fn new() -> Self {
    Self {
      path: Path::new(),
      columns: Vec::new(),
      defined_identifiers: HashSet::new(),
      primary_columns_number: 0,
    }
  }
  
  pub fn primary_scalar_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.defined_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field::new(
      self.path.append_identifier(&identifier),
      FieldSemantics::Primary,
      identifier.clone(),
    );

    self.columns.push(Column::primary(field.path().clone()));
    self.defined_identifiers.insert(identifier.clone());
    self.primary_columns_number += 1;
    Ok(field)
  }

  pub fn readonly_required_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
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
    self.defined_identifiers.insert(identifier.clone());

    Ok(field)
  }

  pub fn readonly_optional_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
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
    self.defined_identifiers.insert(identifier.clone());

    Ok(field)
  }

  pub fn writable_required_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
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
    self.defined_identifiers.insert(identifier.clone());

    Ok(field)
  }

  pub fn writable_optional_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
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
    self.defined_identifiers.insert(identifier.clone());

    Ok(field)
  }

  pub fn compound_field<T>(&mut self, identifier: &str) -> Tried<T, GenericError> 
    where 
      T: IsCompoundType
  {
    let identifier = Identifier::new(identifier)?;

    let mut builder = CompoundTypeDefiner::new(
      self.path.append_identifier(&identifier)
    );

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.into_columns().into_iter());
    self.defined_identifiers.insert(identifier.clone());
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


}