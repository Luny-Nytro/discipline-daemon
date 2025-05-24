#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
  /// Linux rules for a username are:
  /// It must be 1 to 32 characters long.
  /// It can only contain lowercase letters (a-z), digits (0-9), dashes (-), and underscores (_).
  /// It must start with a letter
  pub fn is_valid_linux_username(username: &String) -> bool {
    if username.len() < 1 || 32 < username.len() {
      return false;
    }
  
    let mut characters = username.chars();
    
    let Some(beginning) = characters.next() else {
      return false; // Empty string case (shouldn't happen due to len check)
    };

    if !beginning.is_ascii_lowercase() {
      return false;
    }

    for character in characters {
      if !character.is_ascii_lowercase() 
      || !character.is_ascii_digit() 
      || character != '-' 
      || character != '_' {
        return false
      }
    }

    true
  }

  pub fn new(username: String) -> Option<Username> {
    if Self::is_valid_linux_username(&username) {
      Some(Self(username))
    } else {
      None
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}