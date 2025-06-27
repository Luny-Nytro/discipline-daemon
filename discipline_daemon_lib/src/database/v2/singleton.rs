use std::collections::HashSet;
use super::*;

pub struct SingletonDefiner {
  path: Path,
  columns: Vec<Column>,
  defined_identifiers: HashSet<Identifier>,
}

pub trait IsSingleton: Sized {
  fn new(definer: &mut SingletonDefiner) -> Result<Self, GenericError>;
  fn display_name(&self) -> &str;
}

impl SingletonDefiner {
  pub(super) fn new() -> Self {
    Self {
      path: Path::new(),
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

    let mut builder = CompoundTypeDefiner::new(self.path.append_identifier(&identifier));

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.take_columns().into_iter());
    self.defined_identifiers.insert(identifier);
    Ok(compound_field)
  }

  pub fn optional_compound_field<T>(&mut self, identifier: &str) -> Result<T, GenericError> 
    where 
      T: IsCompoundType
  {
    let identifier = Identifier::new(identifier)?;

    let mut builder = CompoundTypeDefiner::new(self.path.append_identifier(&identifier));

    let compound_field = T::new(&mut builder)?;
    self.columns.extend(builder.take_columns().into_iter());
    self.defined_identifiers.insert(identifier);
    Ok(compound_field)
  }
}