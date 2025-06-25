use crate::GenericError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
  pub(super) fn new(identifier: &str) -> Result<Self, GenericError> {
    // TODO: Escape the identifier
    // Self(identifier.into())
    todo!()
  }

  pub(super) fn as_displayable_string(&self) -> String {
    todo!()
  }

  pub(super) fn as_path(&self) -> Path {
    todo!()
  }
}

#[derive(Debug, Clone)]
pub struct Path(String);

impl Path {
  pub fn new() -> Self {
    Self(String::new())
  }

  pub fn append_identifier(&self, identifier: &Identifier) -> Self {
    // TODO: Escape identifiers
    Self(format!("{}.{}", self.0, identifier.0))
  }

  pub fn append_identifier_string(&self, identifier: &str) -> Self {
    todo!()
    // // TODO: Escape identifiers
    // Self(format!("{}.{}", self.0, identifier.0))
  }

  pub fn to_displayable_string(&self) -> String {
    todo!()
  }

  pub(super) fn as_string(&self) -> &String {
    &self.0
  }

  pub(super) fn as_str(&self) -> &str {
    &self.0
  }
}

enum IdentifierInner {
  Global(String),
  Scoped(String),
}

#[derive(Debug, Clone)]
pub struct DatabaseEntityPath(String);

impl DatabaseEntityPath {
  pub(super) fn new(identifier: &str) -> Result<Self, GenericError> {
    verify_identifier(identifier)
      .map(|_| 
        DatabaseEntityPath(identifier.into())
      )
      .map_err(|error| 
        error.change_context("creating a path")
      )
  }

  pub(super) fn new_empty() -> Self {
    Self(String::new())
  }
  
  pub(super) fn then(&self, identifier: &str) -> Result<Self, GenericError> {
    verify_identifier(identifier)
      .map(|_| 
        DatabaseEntityPath(format!("{}_{}", self.0, identifier))
      )
      .map_err(|error| 
        error
          .change_context("creating a new scoped identifier")
          .add_attachment("super identifier", &self.0)
      )
  }

  pub(super) fn as_str(&self) -> &str {
    &self.0
  }
}

fn verify_identifier(identifier: &str) -> Result<(), GenericError> {
  // // Check if it contains any underscores — disallowed in your case
  // if identifier.contains('_') {
  //   return Err(
  //     GenericError::new("verify identifier")
  //       .add_error("identifier contains an underscore: underscores are reserved by this program for namespacing")
  //       .add_attachment("identifier", identifier)
  //   );
  // }

  // Check if the first character is a valid starting character
  let mut characters = identifier.chars();
  let beginning = match characters.next() {
    Some(character) => {
      character
    }
    None => {
      return Err(
        GenericError::new("verify identifier")
          .add_error("identifier is the empty string")
      );
    }
  };

  if !beginning.is_ascii_alphabetic() {
    return Err(
      GenericError::new("verify identifier")
        .add_error("identifier begins with a character that is not a ascii alphabetic character, which is U+0041 'A' ..= U+005A 'Z' or U+0061 'a' ..= U+007A 'z'")
        .add_attachment("identifier", identifier)
    );
  }

  // Check the rest of the characters are alphanumeric only
  if !characters.all(|character| character.is_ascii_alphanumeric()) {
    return Err(
      GenericError::new("verify identifier")
        .add_error("identifier contains non-alphanumeric characters")
        .add_attachment("identifier", identifier)
    );
  }

  Ok(())
}
