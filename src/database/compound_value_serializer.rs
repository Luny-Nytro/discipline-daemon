use super::SerializeContext;

pub trait CompoundValueSerializer {
  type Input;

  fn serialize_into(
    &self, 
    value: &Self::Input,
    context: &mut SerializeContext, 
  );
}