use crate::GenericError;


pub struct Column {
  pub fully_qualified_name: String,
  pub unique: bool,
  pub primary: bool,
  pub optional: bool,
}

pub struct ColumnBuilder {
  name: String,
  unique: bool,
  primary: bool,
  optional: bool,
}

impl ColumnBuilder {
  fn new(name: String, optional: bool) -> Self {
    Self {
      name,
      unique: false,
      primary: false,
      optional,
    }
  }

  pub fn unique(mut self) -> Self {
    self.unique = true;
    self
  }

  pub fn primary(mut self) -> Self {
    self.primary = true;
    self
  }

  pub fn optional(mut self) -> Self {
    self.optional = true;
    self
  }

  pub fn build(self) -> Result<Column, GenericError> {
    // TODO: Retuen an error if some of the info violates SQLite's column 
    // constraint rules
    Ok(Column {
      fully_qualified_name: self.name,
      unique: self.unique,
      primary: self.primary,
      optional: self.optional,
    })
  }
}

pub struct ColumnNamespace {
  path: String,
  optional: bool,
}

impl ColumnNamespace {
  pub fn new() -> Self {
    Self {
      path: String::new(),
      optional: false,
    }
  }

  pub fn optional(mut self) -> Self {
    self.optional = true;
    self
  }

  pub fn create_column_builder(&self, name: &str) -> ColumnBuilder {
    ColumnBuilder::new(
      format!("{}_{}", self.path, name), 
      self.optional,
    )
  }

  // TODO: This should return a Result with the Err variant representing
  // the 'name' being invalid.
  pub fn create_namespace(&self, name: &str) -> ColumnNamespace {
    ColumnNamespace { 
      path: format!("{}_{}", self.path, name), 
      optional: self.optional,
    }
  }
}