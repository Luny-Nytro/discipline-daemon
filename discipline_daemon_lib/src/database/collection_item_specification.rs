use super::*;

pub struct CollectionItemDefiner {
  path: DatabaseEntityPath
}

impl CollectionItemDefiner {
  pub fn new() -> Self {
    Self {
      path: DatabaseEntityPath::new_empty()
    }
  }

  pub fn define_primary_scalar_field(
    &mut self, 
    namespace: &mut CompoundTypeNamespace,
    scalar_field_identifier: &str,
  ) -> 
    Result<Field, GenericError> 
  {
    namespace.define_primary_scalar_field(
      &self.path, 
      scalar_field_identifier,
    )
  }

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
