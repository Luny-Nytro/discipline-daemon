use super::*;

// CollectionItemMatcher::match_all()
// CollectionItemMatcher::match_by_scalar_field()
// CollectionItemMatcher::match_by_multiple_scalar_fields()
// CollectionItemMatcher::match_by_either_scalar_field()

pub(super) enum CollectionItemMatcherInner {
  NoWhereClause,
  WhereClause(String),
}

// enum CollectionItemMatchOperation {
//   All,
//   Or(Vec<CollectionItemMatchOperation>),
//   And(Vec<CollectionItemMatchOperation>),
// }

pub struct CollectionItemAndMatchWriter {
  code: String,
}

impl CollectionItemAndMatchWriter {
  fn new() -> Self {
    Self { 
      code: String::new()
    }
  }

  fn did_write_a_match(&self) -> bool {
    self.code.len() > 0
  }

  pub fn and_scalar_field_is(
    mut self, 
    scalar_field_specification: &Field,
    scalar_field_value: &impl IntoScalarValue,
  ) -> 
    Result<Self, GenericError>
  {
    // TODO: Err if the scalar field is already added

    let mut serialized_scalar_field_value = String::new();
    // if let Err(error) = 
      serialize_scalar_value_into(scalar_field_value, &mut serialized_scalar_field_value);
    // {
    //   return Err(
    //     // TODO: Use proper error messages
    //     error
    //       .change_context("adding a scalar field match condition to a CollectionItemAndMatcher")
    //       .add_error("failed to serialize scalar field value")
    //       .add_attachment("scalar field specification", format!("{scalar_field_specification:?}"))
    //   );
    // }

    if self.did_write_a_match() {
      self.code.push_str(" AND ");
    } else {
      self.code.push_str("WHERE ");
    }

    self.code.push_str(scalar_field_specification.path.as_str());
    self.code.push_str(" = ");
    self.code.push_str(&serialized_scalar_field_value);

    Ok(self)
  }

  pub fn finalize(self) -> Result<CollectionItemMatcher, GenericError> {
    if self.did_write_a_match() {
      Ok(CollectionItemMatcher {
        inner: CollectionItemMatcherInner::WhereClause(self.code)
      })
    } else {
      Err(
        GenericError::new("finalizing a collection item matcher that matches based on the values of multiple scalar fields")
          .add_error("no scalar fields were specified. use `.and_scalar_field_is()` to add to specify fields to match against. if this was intentional, use `CollectionItemMatcher::match_all()` to match all items in the collection.")
      )
    }
  }
}

pub struct CollectionItemMatcher {
  pub(super) inner: CollectionItemMatcherInner
}

impl CollectionItemMatcher {
  pub fn match_all() -> CollectionItemMatcher {
    CollectionItemMatcher {
      inner: CollectionItemMatcherInner::NoWhereClause
    }
  }

  pub fn match_by_scalar_field(
    scalar_field_specification: &Field,
    scalar_field_value: &impl IntoScalarValue,
  ) -> 
    Result<CollectionItemMatcher, GenericError>
  {
    let mut code = String::new();
    code.push_str("WHERE ");
    code.push_str(scalar_field_specification.path.as_str());
    code.push_str(" = ");
    serialize_scalar_value_into(scalar_field_value, &mut code);
      // .map_err(|error|
      //   error
      //     .change_context("creating a collection item matcher that matches based a single scalar field value")
      //     .add_error("failed to serialize scalar field value")
      //     .add_attachment("scalar field specification", format!("{scalar_field_specification:?}"))
      // )?;

    Ok(CollectionItemMatcher {
      inner: CollectionItemMatcherInner::WhereClause(code)
    })
  }

  pub fn match_by_multiple_scalar_fields() -> CollectionItemAndMatchWriter {
    CollectionItemAndMatchWriter::new()
  }
}
