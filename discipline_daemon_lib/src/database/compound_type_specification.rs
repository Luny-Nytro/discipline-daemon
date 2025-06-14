use super::*;

// Maybe rename to "ScalarFieldsDefinition"
#[derive(Debug)]
pub struct ScalarFieldSpecification {
  pub(super) fully_qualified_identifier: String,
  pub(super) optional: bool,
  pub(super) writeable: bool,
}

pub struct ScalarFieldSpecificationBuilder {
  fully_qualified_name: String,
  optional: bool,
  writeable: bool,
}

impl ScalarFieldSpecificationBuilder {
  fn new(name: String, optional: bool) -> Self {
    Self {
      fully_qualified_name: name,
      optional,
      writeable: false,
    }
  }

  pub fn optional(mut self) -> Self {
    self.optional = true;
    self
  }

  pub fn writeable(mut self) -> Self {
    self.writeable = true;
    self
  }

  pub fn build(self) -> Result<ScalarFieldSpecification, GenericError> {
    verify_identifier(&self.fully_qualified_name)
      .map_err(|error| 
        error
          .change_context("verify field identifier")
          .change_context("create scalar field specification from builder")
      )?;

    Ok(ScalarFieldSpecification {
      fully_qualified_identifier: self.fully_qualified_name,
      optional: self.optional,
      writeable: self.writeable,
    })
  }
}
