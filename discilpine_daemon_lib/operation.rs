use crate::{Daemon, GenericError};

pub trait IsOperation {
  type Outcome;

  fn execute(self, daemon: &mut Daemon) -> Result<Self::Outcome, GenericError>;
}