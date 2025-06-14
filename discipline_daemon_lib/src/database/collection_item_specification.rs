
pub struct CollectionItemFieldsNamespace {
  fully_qualified_name: String,
  optional: bool,
}

impl CollectionItemFieldsNamespace {
  pub fn new() -> Self {
    Self {
      fully_qualified_name: String::new(),
      optional: false,
    }
  }

  fn optional(mut self) -> Self {
    self.optional = true;
    self
  }

  pub fn primary_scalar_field_specification(&mut self, identifier: &str) -> ScalarFieldSpecificationBuilder {
    ScalarFieldSpecificationBuilder::new(
      format!("{}_{}", self.fully_qualified_name, identifier), 
      self.optional,
    )
  }

  pub fn scalar_field_specification(&mut self, identifier: &str) -> ScalarFieldSpecificationBuilder {
    ScalarFieldSpecificationBuilder::new(
      format!("{}_{}", self.fully_qualified_name, identifier), 
      self.optional,
    )
  }

  pub fn compound_field_specification(&mut self, identifier: &str) -> Result<CompoundTypeFieldsScope, GenericError> {
    verify_identifier(identifier)
      .map(|_| CompoundTypeFieldsScope { 
        fully_qualified_name: format!("{}_{}", self.fully_qualified_name, identifier), 
        optional: self.optional,
      })
      .map_err(|error|
        error
          // TODO: update these error messages
          .change_context("verify namespace name")
          .change_context("create namespace")
      )
  }

  pub fn optional_compound_field_specification(&mut self, identifier: &str) -> Result<CompoundTypeFieldsScope, GenericError> {
    verify_identifier(identifier)
      .map(|_| CompoundTypeFieldsScope { 
        fully_qualified_name: format!("{}_{}", self.fully_qualified_name, identifier), 
        optional: self.optional,
      })
      .map_err(|error|
        error
          // TODO: update these error messages
          .change_context("verify namespace name")
          .change_context("create namespace")
      )
  }
}
