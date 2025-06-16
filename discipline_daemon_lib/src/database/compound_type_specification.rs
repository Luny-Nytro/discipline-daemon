use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ColumnType {
  Primary, 
  UniqueRequired,
  UniqueOptional,
  Optional,
  Required,
}

pub(super) struct ColumnSpecification {
  pub(super) path: DatabaseEntityPath,
  pub(super) column_type: ColumnType,
}

#[derive(Debug)]
pub struct ScalarFieldSpecification {
  pub(super) path: DatabaseEntityPath,
  pub(super) optional: bool,
  pub(super) writable: bool,
}

// pub struct ScalarFieldSpecificationBuilder {
//   fully_qualified_name: Identifier,
//   optional: bool,
//   writeable: bool,
// }

// impl ScalarFieldSpecificationBuilder {
//   // pub(super) fn new(name: String, optional: bool, ) -> Self {
//   //   Self {
//   //     fully_qualified_name: name,
//   //     optional,
//   //     writeable: false,
//   //   }
//   // }

//   pub fn optional(mut self) -> Self {
//     self.optional = true;
//     self
//   }

//   pub fn writeable(mut self) -> Self {
//     self.writeable = true;
//     self
//   }

//   pub fn build(self) -> Result<ScalarFieldSpecification, GenericError> {
//     verify_identifier(&self.fully_qualified_name)
//       .map_err(|error| 
//         error
//           .change_context("verifying field identifier")
//           .change_context("creating scalar field specification from builder")
//       )?;

//     Ok(ScalarFieldSpecification {
//       fully_qualified_identifier: self.fully_qualified_name,
//       optional: self.optional,
//       writable: self.writeable,
//     })
//   }
// }

pub struct CompoundTypeNamepace {
  pub(super) columns: Vec<ColumnSpecification>,
  pub(super) primary_columns_number: usize,
}

impl CompoundTypeNamepace {
  pub fn new() -> Self {
    Self {
      columns: Vec::new(),
      primary_columns_number: 0,
    }
  }

  fn did_define_field(&self, identifier: &str) -> bool {
    self.columns.iter().any(|column| column.path == identifier)
  }
}

pub struct CompoundTypeDefiner {
  pub(super) path: DatabaseEntityPath,
  pub(super) optional: bool,
  pub(super) writable: bool,
}

impl CompoundTypeDefiner {
  pub fn define_required_readonly_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an required readonly scalar field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an required readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_path)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Required,
      path: scalar_field_path.into(),
    });

    Ok(ScalarFieldSpecification {
      path: scalar_field_path,
      optional: false,
      writable: false,
    })
  }

  pub fn define_required_writable_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an required writable scalar field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an required writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_path)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Required,
      path: scalar_field_path.clone(),
    });

    Ok(ScalarFieldSpecification {
      path: scalar_field_path,
      optional: false,
      writable: true,
    })
  }

  pub fn define_optional_readonly_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional readonly scalar field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an optional readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", new_scalar_field_identifier)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Optional,
      path: scalar_field_path,
    });

    Ok(ScalarFieldSpecification {
      path: scalar_field_path,
      optional: true,
      writable: false,
    })
  }

  pub fn define_optional_writable_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<ScalarFieldSpecification, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional writable scalar field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an optional writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", new_scalar_field_identifier)
      );
    }

    namespace.columns.push(ColumnSpecification {
      column_type: ColumnType::Optional,
      path: scalar_field_path,
    });

    Ok(ScalarFieldSpecification {
      path: scalar_field_path,
      optional: true,
      writable: true,
    })
  }

  pub fn define_optional_readonly_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional readonly compound field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an optional readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", new_scalar_field_identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: true,
      writable: false,
    })
  }
  
  pub fn define_optional_writable_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional writable compound field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an optional writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", new_scalar_field_identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: true,
      writable: true,
    })
  }

  pub fn define_required_readonly_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an required readonly compound field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an required readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", new_scalar_field_identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: false,
      writable: false,
    })
  }
  
  pub fn define_required_writable_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamepace,
    new_scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = self.path.then(new_scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an required writable compound field")
      )?;

    if self.did_define_field(scalar_field_path) {
      return Err(
        GenericError::new("defining an required writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_path)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: true,
      writable: true,
    })
  }
}