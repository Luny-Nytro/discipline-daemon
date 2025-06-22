#[derive(Debug)]
pub struct ScalarField {
  identifier: String,
  optional: bool,
  writable: bool,
  primary: bool,
  is_defined: bool,
}

impl ScalarField {
  pub fn primary(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: false,
      primary: true,
      writable: false,
      is_defined: false,
    }
  }
  
  pub fn readonly_required(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: false,
      primary: false,
      writable: false,
      is_defined: false,
    }
  }

  pub fn writable_required(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: false,
      primary: false,
      writable: true,
      is_defined: false,
    }
  }

  pub fn readonly_optional(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: true,
      primary: false,
      writable: false,
      is_defined: false,
    }
  }

  pub fn writable_optional(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: true,
      primary: false,
      writable: true,
      is_defined: false,
    }
  }
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
  pub(super) path: String,
  pub(super) column_type: ColumnType,
}