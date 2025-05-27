use crate::Daemon;

pub trait IsOperation {
  type Outcome;

  fn execute(self, app: &mut Daemon) -> Self::Outcome;
}