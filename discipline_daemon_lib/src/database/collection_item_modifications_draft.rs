use super::*;

pub struct CollectionItemChanges {
  code: String,
}

impl CollectionItemChanges {
  pub fn new() -> Self {
    Self {
      code: String::new(),
    }
  }

  fn did_write_a_modification(&self) -> bool {
    self.code.len() > 0
  }

  pub fn set_scalar_field(
    &mut self, 
    scalar_field_specification: &Field, 
    new_scalar_field_value: &impl IntoScalarValue,
  ) ->
    Result<(), GenericError>
  {
    // TODO: Return an error if the scalar field is readonly 
    let mut serialized_scalar_field_value = String::new();

    // if let Err(error) = 
    serialize_scalar_value_into(
      new_scalar_field_value, 
      &mut serialized_scalar_field_value,
    );
    // {
    //   return Err(
    //     error
    //       .change_context("adding a new scalar field modification to CollectionItemModifications")
    //       .add_error("failed to serialize the new scalar field value")
    //       .add_attachment("scalar field specification", format!("{scalar_field_specification:?}"))
    //   )
    // }

    if self.did_write_a_modification() {
      self.code.push_str(", ");
    } else {
      self.code.push_str("SET ");
    }

    self.code.push_str(scalar_field_specification.path.as_str());
    self.code.push_str(" = ");
    self.code.push_str(&serialized_scalar_field_value);

    Ok(())
  }

  pub(super) fn finish(&self) -> Option<&String> {
    Some(&self.code)
  }
}
