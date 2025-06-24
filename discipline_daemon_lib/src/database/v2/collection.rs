use super::*;

enum FieldSemantics {
  Primary,
  ReadonlyRequired,
  ReadonlyOptional,
  WritableRequired,
  WrirableOptional,
}

pub struct Field {
  path: Path,
  semantics: FieldSemantics,
  identifier: Identifier,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ColumnType {
  Primary, 
  UniqueRequired,
  UniqueOptional,
  Optional,
  Required,
}

pub(super) struct Column {
  pub(super) path: Path,
  pub(super) column_type: ColumnType,
}

impl Column {
  pub(super) fn primary(path: Path) -> Self {
    Self {
      path,
      column_type: ColumnType::Primary,
    }
  }

  pub(super) fn unique_required(path: Path) -> Self {
    Self {
      path,
      column_type: ColumnType::UniqueRequired,
    }
  }

  pub(super) fn unique_optional(path: Path) -> Self {
    Self {
      path,
      column_type: ColumnType::UniqueOptional,
    }
  }

  pub(super) fn required(path: Path) -> Self {
    Self {
      path,
      column_type: ColumnType::Required,
    }
  }

  pub(super) fn optional(path: Path) -> Self {
    Self {
      path,
      column_type: ColumnType::Optional,
    }
  }
}

pub trait IsCompoundType: Sized {
  fn define(definer: &mut CompoundTypeBuilder) -> Tried<Self, GenericError>;
}

pub struct CompoundTypeBuilder {
  path: Path,
  columns: Vec<Column>,
  column_identifiers: HashSet<Identifier>,
}

impl CompoundTypeBuilder {
  pub fn add_readonly_required_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.column_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field {
      path: self.path.append_identifier(&identifier),
      semantics: FieldSemantics::ReadonlyRequired,
      identifier: identifier,
    };

    self.columns.push(Column::required(field.path.clone()));

    Ok(field)
  }

  pub fn add_readonly_optional_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.column_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field {
      path: self.path.append_identifier(&identifier),
      semantics: FieldSemantics::ReadonlyOptional,
      identifier,
    };

    self.columns.push(Column::optional(field.path.clone()));

    Ok(field)
  }

  pub fn add_writable_required_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.column_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field {
      path: self.path.append_identifier(&identifier),
      semantics: FieldSemantics::WritableRequired,
      identifier,
    };

    self.columns.push(Column::required(field.path.clone()));

    Ok(field)
  }

  pub fn add_writable_optional_field(
    &mut self, 
    identifier: &str,
  ) -> 
    Tried<Field, GenericError> 
  {
    let identifier = Identifier::new(identifier)?;

    if self.column_identifiers.contains(&identifier) {
      return Err(GenericError::new(""));
    }

    let field = Field {
      path: self.path.append_identifier(&identifier),
      semantics: FieldSemantics::WrirableOptional,
      identifier,
    };

    self.columns.push(Column::optional(field.path.clone()));

    Ok(field)
  }

  pub fn add_compound_field<T>(&mut self, identifier: &str) -> Tried<T, GenericError> 
    where 
      T: IsCompoundType
  {
    let identifier = Identifier::new(identifier)?;

    let mut builder = CompoundTypeBuilder {
      path: self.path.append_identifier(&identifier),
      columns: Vec::new(),
      column_identifiers: HashSet::new(),
    };

    let compound_field = T::define(&mut builder)?;
    self.columns.extend(builder.columns.into_iter());
    self.column_identifiers.insert(identifier.clone());
    Ok(compound_field)
  }
}

pub struct Collection {
  identifier: Identifier,
  path: Path,
}

pub trait IsCollectionDefiner {
  fn add_collection<T>(&mut self, identifier: &str) -> Collection
    where 
      T: IsCollectionItem;
}

pub trait IsCompoundTypeDefiner {
  fn add_readonly_required(&mut self, identifier: &str) -> Result<Field, GenericError>;
  fn add_readonly_optional(&mut self, identifier: &str) -> Result<Field, GenericError>;
  fn add_writable_required(&mut self, identifier: &str) -> Result<Field, GenericError>;
  fn add_writable_optional(&mut self, identifier: &str) -> Result<Field, GenericError>;
  fn add_compound_field<T>(&self, identifier: &str) -> Result<T, GenericError>
    where 
      T: IsCompoundType;
}

pub trait IsCollectionItemDefiner {
  fn add_primary();
}

pub struct CollectionItemDefiner {
  columns: Vec<Column>
}

// pub trait IsCompoundType {
//   fn new() -> Self;
//   fn serializer() -> Self;
//   fn deserializer() -> Self;
// }

pub trait IsCollectionItem: Sized {
  fn new() -> Self;
}

pub(super) enum CollectionInner {
  
}

impl Collection {
  pub(super) fn new(
    // path: DatabaseEntityPath,
    // collection_item_namespace: CompoundTypeNamespace,
  ) -> 
    Result<Self, GenericError>
  {
    if collection_item_namespace.columns.is_empty() {
      return Err(
        GenericError::new("creating a collection")
          .add_error("you didn't define any fields for the collection item! define one or more by calling the 'add_*_field' methods of the CollectionItemDefiner")
          .add_attachment("collection path", path.as_str())
      );
    }
    if collection_item_namespace.primary_columns_number == 0 {
      return Err(
        GenericError::new("creating a collection")
          .add_error("you didn't define any primary fields for the collection item! define one or more by calling the 'add_primary_scalar_field' method of the CollectionItemDefiner")
          .add_attachment("collection path", path.as_str())
      );
    }

    Ok(Self {
      path,
      collection_item_namespace,
    })
  }
}

use std::collections::HashSet;
use super::*;


pub struct DatabaseNamespace {
  pub(super) path: DatabaseEntityPath,
  defined_entities: HashSet<String>,
}

impl DatabaseNamespace {
  pub(super) fn new(path: DatabaseEntityPath) -> Self {
    Self {
      path,
      defined_entities: HashSet::new(),
    }
  }

  pub fn add_namespace(
    &mut self, 
    database: &mut Database,
    new_namespace_identifier: &str,
  ) -> 
    Result<DatabaseNamespace, GenericError> 
  {
    // TODO: check whether there is a namespace with the given identifier

    self.path.then(new_namespace_identifier)
      .map(|path| 
        DatabaseNamespace { 
          path,
          defined_entities: HashSet::new(),
        }
      )
      .map_err(|error|
        error
          .change_context("creating a new namespace within a non-global namespace")
          .add_error("invalid namespace identifier")
          .add_attachment("super namespace fully qualified identifier", self.path.as_str())
      )
  }

  pub fn add_collection(
    &mut self, 
    database: &mut Database,
    collection_identifier: &str,
    collection_item_namespace: CompoundTypeNamespace,
  ) -> 
    Result<Collection, GenericError> 
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
        Collection::new(
          collection_path,
          collection_item_namespace,
        )
      )
  } 
}