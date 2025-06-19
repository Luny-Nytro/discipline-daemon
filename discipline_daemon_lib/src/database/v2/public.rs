use super::*;

#[derive(Debug)]
pub struct Field {
  identifier: String,
  optional: bool,
  writable: bool,
  primary: bool,
}

impl Field {
  pub fn primary(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: false,
      primary: true,
      writable: false,
    }
  }
  
  pub fn readonly_required(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: false,
      primary: false,
      writable: false,
    }
  }

  pub fn writable_required(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: false,
      primary: false,
      writable: true,
    }
  }

  pub fn readonly_optional(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: true,
      primary: false,
      writable: false,
    }
  }

  pub fn writable_optional(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
      optional: true,
      primary: false,
      writable: true,
    }
  }
}

pub struct Collection {
  identifier: String,
}

impl Collection {
  pub fn new(identifier: &str) -> Self {
    Self {
      identifier: identifier.into(),
    }
  }
}

pub struct Definitions {
  colletionless_item_fields: Vec<()>,
  collections
}


pub struct Definer {
  path: String,
}

impl Definer {
  fn define_field(&mut self, definitions: &mut Definitions, field: &mut Field) -> Result<Definer, GenericError> {

  }

  fn define_collection(&mut self, collection: &mut Collection) -> Result<Definer, GenericError>;
  fn define_namespace(&mut self, namespace_identifier: &str) -> Result<Definer, GenericError>;
}