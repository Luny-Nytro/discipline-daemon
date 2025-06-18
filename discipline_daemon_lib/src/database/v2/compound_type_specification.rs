use std::collections::HashSet;
use super::*;

pub trait IsCompoundType {
  fn define(&mut self, definer: &mut impl CompoundTypeDefiner);
}

pub trait CompoundTypeDefiner {

}

#[derive(Debug)]
pub struct Field {
  pub(super) path: DatabaseEntityPath,
  optional: bool,
  writable: bool,
  primary: bool,
}

enum FieldInner {
  Undefined(UndefinedField),
  Defined(DefinedField),
}

struct DefinedField {
  name: String,
  primary: bool,
  writable: bool,
  optional: bool,
}

struct UndefinedField {
  name: String,
  primary: bool,
  writable: bool,
  optional: bool,
}

impl Field {
  pub(super) fn is_primary(&self) -> bool {
    self.primary
  }
  pub(super) fn is_optional(&self) -> bool {
    self.optional
  }
  pub(super) fn is_writable(&self) -> bool {
    self.writable
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

pub(super) struct ColumnSpecification {
  pub(super) path: DatabaseEntityPath,
  pub(super) column_type: ColumnType,
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

pub struct CompoundTypeNamespace {
  pub(super) columns: Vec<ColumnSpecification>,
  pub(super) primary_columns_number: usize,
  defined_fields: HashSet<String>,
}

impl CompoundTypeNamespace {
  pub fn new() -> Self {
    Self {
      columns: Vec::new(),
      primary_columns_number: 0,
      defined_fields: HashSet::new(),
    }
  }

  pub(super) fn did_define_field(&self, identifier: &str) -> bool {
    self.defined_fields.contains(identifier)
  }

  pub(super) fn define_primary_scalar_field(
    &mut self, 
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining a primary scalar field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining a primary scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    self.columns.push(ColumnSpecification {
      column_type: ColumnType::Primary,
      path: path.clone(),
    });

    Ok(Field {
      path,
      optional: false,
      writable: false,
    })
  }
  
  pub(super) fn define_required_readonly_scalar_field(
    &mut self,
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining a required readonly scalar field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining a required readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    self.columns.push(ColumnSpecification {
      column_type: ColumnType::Required,
      path: scalar_field_path.clone(),
    });

    Ok(Field {
      path: scalar_field_path,
      optional: false,
      writable: false,
    })
  }

  pub(super) fn define_required_writable_scalar_field(
    &mut self,
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining a required writable scalar field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining a required writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    self.columns.push(ColumnSpecification {
      column_type: ColumnType::Required,
      path: scalar_field_path.clone(),
    });

    Ok(Field {
      path: scalar_field_path,
      optional: false,
      writable: true,
    })
  }

  pub(super) fn define_optional_readonly_scalar_field(
    &mut self,
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional readonly scalar field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining an optional readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    self.columns.push(ColumnSpecification {
      column_type: ColumnType::Optional,
      path: scalar_field_path.clone(),
    });

    Ok(Field {
      path: scalar_field_path,
      optional: true,
      writable: false,
    })
  }

  pub(super) fn define_optional_writable_scalar_field(
    &mut self,
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional writable scalar field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining an optional writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    self.columns.push(ColumnSpecification {
      column_type: ColumnType::Optional,
      path: scalar_field_path.clone(),
    });

    Ok(Field {
      path: scalar_field_path,
      optional: true,
      writable: true,
    })
  }

  pub(super) fn define_optional_readonly_compound_field(
    &mut self, 
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional readonly compound field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining an optional readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: true,
      writable: false,
    })
  }
  
  pub(super) fn define_optional_writable_compound_field(
    &mut self, 
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining an optional writable compound field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining an optional writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: true,
      writable: true,
    })
  }

  pub(super) fn define_required_readonly_compound_field(
    &mut self, 
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining a required readonly compound field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining a required readonly scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: false,
      writable: false,
    })
  }
  
  pub(super) fn define_required_writable_compound_field(
    &mut self, 
    super_type_path: &DatabaseEntityPath,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    let scalar_field_path = super_type_path.then(scalar_field_identifier)
      .map_err(|error|
        error.change_context("defining a required writable compound field")
      )?;

    if self.did_define_field(scalar_field_identifier) {
      return Err(
        GenericError::new("defining a required writable scalar field")
          .add_error("a field with the provided identifier already exists")
          .add_attachment("identifier", scalar_field_identifier)
      );
    }

    Ok(CompoundTypeDefiner {
      path: scalar_field_path,
      optional: false,
      writable: true,
    })
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
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    namespace.define_required_readonly_scalar_field(
      &self.path,
      scalar_field_identifier,
    )
  }
  
  pub fn define_required_writable_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    namespace.define_required_writable_scalar_field(
      &self.path,
      scalar_field_identifier,
    )
  }
  
  pub fn define_optional_readonly_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    namespace.define_optional_readonly_scalar_field(
      &self.path,
      scalar_field_identifier,
    )
  }

  pub fn define_optional_writable_scalar_field(
    &mut self,
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    namespace.define_optional_writable_scalar_field(
      &self.path,
      scalar_field_identifier,
    )
  }

  pub fn define_optional_readonly_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    namespace.define_optional_readonly_compound_field(
      &self.path,
      scalar_field_identifier,
    )
  }
  pub fn define_optional_writable_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    namespace.define_optional_writable_compound_field(
      &self.path,
      scalar_field_identifier,
    )
  }
  pub fn define_required_readonly_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    namespace.define_required_readonly_compound_field(
      &self.path,
      scalar_field_identifier,
    )
  }
  pub fn define_required_writable_compound_field(
    &mut self, 
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<CompoundTypeDefiner, GenericError> 
  {
    namespace.define_required_writable_compound_field(
      &self.path,
      scalar_field_identifier,
    )
  }
}