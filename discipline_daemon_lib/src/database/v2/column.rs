use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ColumnSemantics {
  Primary, 
  UniqueRequired,
  UniqueOptional,
  Optional,
  Required,
}

#[derive(Debug, Clone,)]
pub(super) struct Column {
  path: Path,
  semantics: ColumnSemantics,
}

impl Column {
  pub(super) fn primary(path: Path) -> Self {
    Self {
      path,
      semantics: ColumnSemantics::Primary,
    }
  }

  pub(super) fn unique_required(path: Path) -> Self {
    Self {
      path,
      semantics: ColumnSemantics::UniqueRequired,
    }
  }

  pub(super) fn unique_optional(path: Path) -> Self {
    Self {
      path,
      semantics: ColumnSemantics::UniqueOptional,
    }
  }

  pub(super) fn required(path: Path) -> Self {
    Self {
      path,
      semantics: ColumnSemantics::Required,
    }
  }

  pub(super) fn optional(path: Path) -> Self {
    Self {
      path,
      semantics: ColumnSemantics::Optional,
    }
  }

  pub(super) fn path(&self) -> &Path {
    &self.path
  }

  pub(super) fn semantics(&self) -> ColumnSemantics {
    self.semantics
  }
}
