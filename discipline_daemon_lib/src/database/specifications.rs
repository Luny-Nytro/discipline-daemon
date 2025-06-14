

pub struct CompoundTypeFieldsScope {
  fully_qualified_name: String,
  optional: bool,
}

impl CompoundTypeFieldsScope {
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

#[derive(Debug, PartialEq, Eq)]
pub enum ColumnType {
  Primary, 
  UniqueRequired,
  UniqueOptional,
  Optional,
  Required,
}

pub struct ColumnSpecification {
  pub(super) fully_qualified_name: String,
  pub(super) column_type: ColumnType,
}

pub trait CompoundTypeSpecificationProvider {
  fn add_fields(&self, context: &mut CompoundTypeFieldsSpecification) -> Result<(), GenericError>;
}

pub struct CompoundTypeFieldsSpecification {
  column_specifications: Vec<ColumnSpecification>,
}

impl CompoundTypeFieldsSpecification {
  pub fn new() -> Self {
    Self {
      column_specifications: Vec::new()
    }
  }
  
  pub fn add_scalar_field(&mut self, scalar_field_specification: &ScalarFieldSpecification) -> Result<(), GenericError> {
    if self
      .column_specifications
      .iter()
      .any(|spec| spec.fully_qualified_name == scalar_field_specification.fully_qualified_identifier)
    {
      return Err(todo!());
    }

    self.column_specifications.push(ColumnSpecification {
      column_type: ColumnType::Required,
      fully_qualified_name: scalar_field_specification.fully_qualified_identifier.clone(),
    });

    Ok(())
  }

  pub fn add_compound_field(
    &mut self, 
    compound_field_specification_provider: &impl CompoundTypeSpecificationProvider,
  ) -> 
    Result<(), GenericError>
  {
    compound_field_specification_provider.add_fields(self)
  }
}
