use super::*;


pub struct CollectionItemDefiner {
  
}

impl CollectionItemDefiner {
  pub fn define_primary_scalar_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining a primary scalar field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining a primary scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Primary,
      path: identifier.into(),
    });

    Ok(ScalarFieldSpecification {
      path: identifier.into(),
      optional: false,
      writable: false,
    })
  }

  pub fn define_required_readonly_scalar_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an required readonly scalar field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an required readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Required,
      path: identifier.into(),
    });

    Ok(ScalarFieldSpecification {
      path: identifier.into(),
      optional: false,
      writable: false,
    })
  }

  pub fn define_required_writable_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an required writable scalar field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an required writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Required,
      path: identifier.into(),
    });

    Ok(ScalarFieldSpecification {
      path: identifier.into(),
      optional: false,
      writable: true,
    })
  }

  pub fn define_optional_readonly_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an optional readonly scalar field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an optional readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Optional,
      path: identifier.into(),
    });

    Ok(ScalarFieldSpecification {
      path: identifier.into(),
      optional: true,
      writable: false,
    })
  }

  pub fn define_optional_writable_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an optional writable scalar field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an optional writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Optional,
      path: identifier.into(),
    });

    Ok(ScalarFieldSpecification {
      path: identifier.into(),
      optional: true,
      writable: true,
    })
  }

  pub fn define_optional_readonly_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an optional readonly compound field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an optional readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: identifier.into(),
      optional: true,
      writable: false,
    })
  }
  
  pub fn define_optional_writable_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an optional writable compound field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an optional writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: identifier.into(),
      optional: true,
      writable: true,
    })
  }

  pub fn define_required_readonly_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an required readonly compound field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an required readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: identifier.into(),
      optional: false,
      writable: false,
    })
  }
  
  pub fn define_required_writable_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let path = DatabaseEntityPath::new(identifier)
      .map_err(|error|
        error.change_context("defining an required writable compound field")
      )?;

    if self.did_define_field(identifier) {
      return Err(
        GenericError::new("defining an required writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: identifier.into(),
      optional: false,
      writable: true,
    })
  }
}
