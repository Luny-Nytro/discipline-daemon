use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FieldSemantics {
  Primary,
  ReadonlyRequired,
  ReadonlyOptional,
  WritableRequired,
  WrirableOptional,
}

// TODO: Later, rename this to Fieldspecification
#[derive(Debug, Clone)]
pub struct Field {
  path: Path,
  semantics: FieldSemantics,
  identifier: Identifier,
}

impl Field {
  pub(super) fn new(
    path: Path, 
    semantics: FieldSemantics,
    identifier: Identifier, 
  ) -> Self {
    Self {
      path, 
      identifier,
      semantics,
    }
  }

  pub(super) fn path(&self) -> &Path {
    &self.path
  }

  pub(super) fn is_readonly(&self) -> bool {
    match self.semantics {
      FieldSemantics::Primary => true,
      FieldSemantics::ReadonlyOptional => true,
      FieldSemantics::ReadonlyRequired => true,
      FieldSemantics::WrirableOptional => false,
      FieldSemantics::WritableRequired => false,
    }
  }
}