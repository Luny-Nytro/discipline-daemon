use super::*;


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


pub struct CompoundTypeDefiner<'a> {
  collection_item_definer: &'a mut CollectionItemDefiner
}

impl<'a> CompoundTypeDefiner<'a> {
  pub fn define_scalar_field(&mut self, field: &mut ScalarField) -> Tried<(), GenericError> {
    todo!()
  }

  pub fn define_compound_field(&mut self, compound_type: &mut impl IsCompoundType) -> Tried<(), GenericError> {
    todo!()
  }
}

pub trait IsCompoundType {
  fn define(&mut self, definer: &mut CompoundTypeDefiner) -> Tried<(), GenericError>;
}
