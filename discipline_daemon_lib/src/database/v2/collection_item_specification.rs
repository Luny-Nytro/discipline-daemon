use super::*;

pub trait IsCollectionItem {
  fn define(&mut self, definer: &mut CollectionItemDefiner) -> Tried<(), GenericError>;
}

pub struct CollectionItemDefiner {
  columns: Vec<Column>,
  primary_columns_number: usize,
}

impl CollectionItemDefiner {
  pub fn define_scalar_field(&mut self, field: &mut ScalarField) -> Tried<(), GenericError> {
    todo!()
  }

  pub fn define_compound_field(&mut self, compound_type: &mut impl IsCompoundType) -> Tried<(), GenericError> {
    todo!()
  }
}

//       path: DatabaseEntityPath::new_empty()
//     }
//   }

//   pub fn define_primary_scalar_field(
//     &mut self, 
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<Field, GenericError> 
//   {
//     namespace.define_primary_scalar_field(
//       &self.path, 
//       scalar_field_identifier,
//     )
//   }

//   pub fn define_required_readonly_scalar_field(
//     &mut self,
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<Field, GenericError> 
//   {
//     namespace.define_required_readonly_scalar_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }
  
//   pub fn define_required_writable_scalar_field(
//     &mut self,
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<Field, GenericError> 
//   {
//     namespace.define_required_writable_scalar_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }
  
//   pub fn define_optional_readonly_scalar_field(
//     &mut self,
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<Field, GenericError> 
//   {
//     namespace.define_optional_readonly_scalar_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }

//   pub fn define_optional_writable_scalar_field(
//     &mut self,
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<Field, GenericError> 
//   {
//     namespace.define_optional_writable_scalar_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }

//   pub fn define_optional_readonly_compound_field(
//     &mut self, 
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<CompoundTypeDefiner, GenericError> 
//   {
//     namespace.define_optional_readonly_compound_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }
//   pub fn define_optional_writable_compound_field(
//     &mut self, 
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<CompoundTypeDefiner, GenericError> 
//   {
//     namespace.define_optional_writable_compound_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }
//   pub fn define_required_readonly_compound_field(
//     &mut self, 
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<CompoundTypeDefiner, GenericError> 
//   {
//     namespace.define_required_readonly_compound_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }
//   pub fn define_required_writable_compound_field(
//     &mut self, 
//     namespace: &mut CompoundTypeNamespace,
//     scalar_field_identifier: &str,
//   ) -> 
//     Result<CompoundTypeDefiner, GenericError> 
//   {
//     namespace.define_required_writable_compound_field(
//       &self.path,
//       scalar_field_identifier,
//     )
//   }
// }
