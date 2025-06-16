use crate::GenericError;

enum IdentifierInner {
  Global(String),
  Scoped(String),
}

pub struct DatabaseEntityPath(String);

impl DatabaseEntityPath {
  pub fn new(identifier: &str) -> Result<(), GenericError> {
    verify_identifier(identifier)
      .map(|_| 
        DatabaseEntityPath(identifier.into())
      )
      .map_err(|error| 
        error.change_context("creating a path")
      )
  }

  pub fn then(&self, identifier: &str) -> Result<(), GenericError> {
    verify_identifier(identifier)
      .map(|_| 
        DatabaseEntityPath(format!("{}_{}", self.value(), identifier))
      )
      .map_err(|error| 
        error
          .change_context("creating a new scoped identifier")
          .add_attachment("super identifier", self.value())
      )
  }

  pub fn as_str(&self) -> &str {
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
