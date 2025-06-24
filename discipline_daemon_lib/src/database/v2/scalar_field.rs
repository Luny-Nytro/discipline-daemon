use super::*;

#[derive(Debug)]
pub struct ScalarField(ScalarFieldInner);

#[derive(Debug)]
pub(super) enum ScalarFieldInner {
  Undefined(UndefinedScalarField),
  Defined(DefinedScalarField),
}

#[derive(Debug)]
pub(super) struct UndefinedScalarField {
  identifier: Identifier,
  semantics: ScalarFieldSemantics,
}

impl UndefinedScalarField {
  pub(super) fn identifier(&self) -> &Identifier {
    &self.identifier
  }

  pub(super) fn semantics(&self) -> &ScalarFieldSemantics {
    &self.semantics
  }

  pub(super) fn define(&self, path: Path) -> ScalarField {
    ScalarField(ScalarFieldInner::Defined(DefinedScalarField { 
      path, 
      semantics: self.semantics 
    }))
  }
}

#[derive(Debug)]
pub(super) struct DefinedScalarField {
  path: Path,
  semantics: ScalarFieldSemantics,
}

impl DefinedScalarField {
  pub(super) fn path(&self) -> &Path {
    &self.path
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScalarFieldSemantics {
  Primary,
  RequiredReadonly,
  RequiredWritable,
  OptionalReadonly,
  OptionalWritable,
}

impl ScalarField {
  pub fn primary(identifier: &str) -> Self {
    Self(ScalarFieldInner::Undefined(UndefinedScalarField{
      identifier: Identifier::new(identifier),
      semantics: ScalarFieldSemantics::Primary,
    }))
  }
  
  pub fn readonly_required(identifier: &str) -> Self {
    Self(ScalarFieldInner::Undefined(UndefinedScalarField{
      identifier: Identifier::new(identifier),
      semantics: ScalarFieldSemantics::RequiredReadonly,
    }))
  }

  pub fn writable_required(identifier: &str) -> Self {
    Self(ScalarFieldInner::Undefined(UndefinedScalarField{
      identifier: Identifier::new(identifier),
      semantics: ScalarFieldSemantics::RequiredWritable,
    }))
  }

  pub fn readonly_optional(identifier: &str) -> Self {
    Self(ScalarFieldInner::Undefined(UndefinedScalarField{
      identifier: Identifier::new(identifier),
      semantics: ScalarFieldSemantics::OptionalReadonly,
    }))
  }

  pub fn writable_optional(identifier: &str) -> Self {
    Self(ScalarFieldInner::Undefined(UndefinedScalarField{
      identifier: Identifier::new(identifier),
      semantics: ScalarFieldSemantics::OptionalWritable,
    }))
  }

  pub(super) fn inner(&self) -> &ScalarFieldInner {
    &self.0
  }

  pub(super) fn inner_mut(&mut self) -> &mut ScalarFieldInner {
    &mut self.0
  }

  pub(super) fn as_defined(&self) -> Result<&DefinedScalarField, GenericError> {
    match &self.0 {
      ScalarFieldInner::Defined(inner) => {
        Ok(inner)
      }
      ScalarFieldInner::Undefined(inner) => {
        Err(
          GenericError::new("attempting to use a ScalarField in an internal database operation")
            .add_error("ScalarField is not defined yet!")
            .add_attachment("ScalarField path", inner.identifier().as_displayable_string())
        )
      }
    }
  }

  // pub(super) fn is_primary(&self) -> bool {
  //   self.primary
  // }
  // pub(super) fn is_readonly(&self) -> bool {
  //   self.is_readonly()
  // }
}


