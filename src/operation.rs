use crate::App;

pub trait IsOperation {
  type Outcome;

  fn execute(self, app: &mut App) -> Self::Outcome;
}