use super::GenericError;


fn verify_identifier(string: &str) -> Result<(), GenericError> {
  // Check if it contains any underscores — disallowed in your case
  if string.contains('_') {
    return Err(
      GenericError::new("checking whether string is a valid database entity identifier")
        .add_error("string contains an underscore: underscores are reserved by this program for namespacing")
        .add_attachment("string", string)
    );
  }

  // Check if the first character is a valid starting character
  let mut characters = string.chars();
  let beginning = match characters.next() {
    Some(character) => {
      character
    }
    None => {
      return Err(
        GenericError::new("checking whether string is a valid database entity identifier")
          .add_error("string is the empty string")
      );
    }
  };

  if !beginning.is_ascii_alphabetic() {
    return Err(
      GenericError::new("checking whether string is a valid database entity identifier")
        .add_error("string begins with a character that is not a ascii alphabetic character, which is U+0041 'A' ..= U+005A 'Z' or U+0061 'a' ..= U+007A 'z'")
        .add_attachment("string", string)
    );
  }

  // Check the rest of the characters are alphanumeric only
  if !characters.all(|character| character.is_ascii_alphanumeric()) {
    return Err(
      GenericError::new("checking whether string is a valid database entity identifier")
        .add_error("string contains non-alphanumeric characters")
        .add_attachment("string", string)
    );
  }

  Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
  pub(super) fn new(identifier: &str) -> Result<Self, GenericError> {
    match verify_identifier(identifier) {
      Ok(_) => {
        Ok(Self(identifier.into()))
      }
      Err(error) => {
        Err(error.change_context("creating a database entity identifier from string"))
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct Path(String);

impl Path {
  pub(super) fn new() -> Self {
    Self(String::new())
  }

  pub(super) fn append_identifier(&self, identifier: &Identifier) -> Self {
    if self.0.len() > 0 {
      Self(format!("{}_{}", self.0, identifier.0))
    } else {
      Self(identifier.0.clone())
    }
  }

  pub(super) fn to_displayable_string(&self) -> &String {
    &self.0
  }

  pub(super) fn to_sql_identifier_string(&self) -> &String {
    &self.0
  }

  pub(super) fn to_sql_identifier_str(&self) -> &str {
    &self.0
  }
}