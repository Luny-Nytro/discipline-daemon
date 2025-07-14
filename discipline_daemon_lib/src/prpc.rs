use crate::{Daemon, GenericError};

pub trait IsPRPC {
  type Outcome;

  fn execute(self, daemon: &mut Daemon) -> Self::Outcome;
}

// enum InternalOperationOutcomeInner<T> {
//   InternalError(GenericError),
//   PublicOutcome(T),
// }

// pub struct InternalOperationOutcome<T>(InternalOperationOutcomeInner<T>);

// impl<T> InternalOperationOutcome<T> {
//   pub fn internal_error(generic_error: GenericError) -> Self {
//     Self(InternalOperationOutcomeInner::InternalError(generic_error))
//   }

//   pub fn public_outcome(outcome: T) -> Self {
//     Self(InternalOperationOutcomeInner::PublicOutcome(outcome))
//   }
// }