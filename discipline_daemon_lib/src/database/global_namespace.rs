use super::*;

pub struct GlobalNamespace {
  pub(super) identifier: String,
}

impl GlobalNamespace {
  pub fn namespace(&mut self, identifier: &str) -> Result<Namespace, GenericError> {
    // TODO: check whether there is already a namespace with the given identifier
    verify_identifier(identifier)
      .map(|_| 
        Namespace { 
          path: identifier.into(), 
          fully_qualified_identifier: format!("{}_{}", self.identifier, identifier),
        }
      )
      .map_err(|error|
        error
          .change_context("creating a new namespace within the global namespace")
          .add_error("invalid namespace identifier")
      )
  }
}
