use std::collections::HashSet;
use super::*;

pub trait IsTopLevelCompoundValueSchema: Sized {
  type CompoundValue;

  fn new(definer: &mut TopLevelCompoundValueSchemaDefiner) -> Result<Self, GenericError>;
  fn display_name(&self) -> &str;
  fn create_initial_instance(&self) -> Self::CompoundValue;
}

pub struct TopLevelCompoundValueSchemaDefiner {
  columns: Vec<Column>,
  defined_column_identifiers: HashSet<Identifier>,
  
  collections: Vec<Collection>,
  defined_module_and_collection_identifiers: HashSet<Identifier>,
}

impl TopLevelCompoundValueSchemaDefiner {
  pub(super) fn new() -> Self {
    Self {
      columns: Vec::new(),
      defined_column_identifiers: HashSet::new(),
      collections: Vec::new(),
      defined_module_and_collection_identifiers: HashSet::new(),
    }
  }

  pub fn readonly_required_field(
    &mut self, 
    identifier_as_str: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier_as_str).map_err(|error|
      error.change_context("definning a readonly required field for the top level compound type schema")
    )?;

    if self.defined_column_identifiers.contains(&identifier) {
      return Err(
        GenericError::new("definning a readonly required field for the top level compound type schema")
          .add_error("a field with the same identifier already exists")
          .add_attachment("identifier", identifier_as_str)
      );
    }

    let field = Field::new(
      identifier.to_path(),
      FieldSemantics::ReadonlyRequired,
      identifier.clone(),
    );

    self.columns.push(Column::required(field.path().clone()));
    self.defined_column_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn readonly_optional_field(
    &mut self, 
    identifier_as_str: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier_as_str).map_err(|error|
      error.change_context("definning a readonly optional field for the top level compound type schema")
    )?;

    if self.defined_column_identifiers.contains(&identifier) {
      return Err(
        GenericError::new("definning a readonly optional field for the top level compound type schema")
          .add_error("a field with the same identifier already exists")
          .add_attachment("identifier", identifier_as_str)
      );
    }

    let field = Field::new(
      identifier.to_path(),
      FieldSemantics::ReadonlyOptional,
      identifier.clone(),
    );

    self.columns.push(Column::optional(field.path().clone()));
    self.defined_column_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn writable_required_field(
    &mut self, 
    identifier_as_str: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier_as_str).map_err(|error|
      error.change_context("definning a writable required field for the top level compound type schema")
    )?;

    if self.defined_column_identifiers.contains(&identifier) {
      return Err(
        GenericError::new("definning a writable required field for the top level compound type schema")
          .add_error("a field with the same identifier already exists")
          .add_attachment("identifier", identifier_as_str)
      );
    }

    let field = Field::new(
      identifier.to_path(),
      FieldSemantics::WritableRequired,
      identifier.clone(),
    );

    self.columns.push(Column::required(field.path().clone()));
    self.defined_column_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn writable_optional_field(
    &mut self, 
    identifier_as_str: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier_as_str).map_err(|error|
      error.change_context("definning a writable optional field for the top level compound type schema")
    )?;

    if self.defined_column_identifiers.contains(&identifier) {
      return Err(
        GenericError::new("definning a writable optional field for the top level compound type schema")
          .add_error("a field with the same identifier already exists")
          .add_attachment("identifier", identifier_as_str)
      );
    }

    let field = Field::new(
      identifier.to_path(),
      FieldSemantics::WrirableOptional,
      identifier.clone(),
    );

    self.columns.push(Column::optional(field.path().clone()));
    self.defined_column_identifiers.insert(identifier);

    Ok(field)
  }

  pub fn compound_field<T>(&mut self, identifier_as_str: &str) -> Result<T, GenericError> 
    where 
      T: IsCompoundType
  {
    let identifier = Identifier::new(identifier_as_str).map_err(|error|
      error.change_context("definning a compound field for the top level compound type schema")
    )?;

    if self.defined_column_identifiers.contains(&identifier) {
      return Err(
        GenericError::new("definning a compound field for the top level compound type schema")
          .add_error("a field with the same identifier already exists")
          .add_attachment("identifier", identifier_as_str)
      );
    }

    let mut builder = CompoundTypeDefiner::new(
      identifier.to_path()
    );

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.into_columns().into_iter());
    self.defined_column_identifiers.insert(identifier);
    Ok(compound_field)
  }

  pub fn optional_compound_field<T>(&mut self, identifier_as_str: &str) -> Result<T, GenericError> 
    where 
      T: IsCompoundType
  {
    let identifier = Identifier::new(identifier_as_str).map_err(|error|
      error.change_context("definning an optional compound field for the top level compound type schema")
    )?;

    if self.defined_column_identifiers.contains(&identifier) {
      return Err(
        GenericError::new("definning an optional compound field for the top level compound type schema")
          .add_error("a field with the same identifier already exists")
          .add_attachment("identifier", identifier_as_str)
      );
    }

    let mut builder = CompoundTypeDefiner::new(identifier.to_path());

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.into_columns().into_iter());
    self.defined_column_identifiers.insert(identifier);
    Ok(compound_field)
  }

  pub fn module<T>(
    &mut self, 
    identifier_as_str: &str,
  ) -> 
    Result<T, GenericError>
  where 
    T: IsModule
  {
    let identifier = Identifier::new(identifier_as_str).map_err(|error|
      error.change_context("definning a module under the top level compound type schema")
    )?;

    if self.defined_column_identifiers.contains(&identifier) {
      return Err(
        GenericError::new("definning a module under the top level compound type schema")
          .add_error("a module or collection with the same identifier already exists")
          .add_attachment("identifier", identifier_as_str)
      );
    }

    let mut builder = CompoundTypeDefiner::new(identifier.to_path());

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.into_columns().into_iter());
    self.defined_column_identifiers.insert(identifier);
    Ok(compound_field)
    // let identifier = Identifier::new(identifier_as_str)?;

    // if self.defined_identifiers.contains(&identifier) {
    //   return Err(GenericError::new(""));
    // }

    // let mut definer = ModuleDefiner { 
    //   path: identifier.to_path(),
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
    identifier_as_str: &str,
  ) -> 
    Result<(Collection, CollectionItem), GenericError>
  where 
    CollectionItem: IsCollectionItem
  {
    todo!()
    // let identifier = Identifier::new(identifier_as_str)?;

    // if self.defined_identifiers.contains(&identifier) {
    //   return Err(GenericError::new(""));
    // }

    // let mut collection_item_definer = CollectionItemDefiner::new();

    // let collection_item = CollectionItem::new(
    //   &mut collection_item_definer,
    // )?;

    // let collection = Collection::new(
    //   identifier.to_path(),
    //   identifier.clone(),
    //   collection_item_definer,
    // );

    // self.defined_identifiers.insert(identifier.clone());

    // Ok((collection, collection_item))
  }

  pub(super) fn take_columns(self) -> Vec<Column> {
    self.columns
  }
}
