use std::{fmt::Debug, marker::{PhantomData, PhantomPinned}};

use crate::DateTime;

#[derive(Debug, Clone, Copy)]
pub struct InternalErrorLogger {
  _moon: PhantomData<()>
}

impl InternalErrorLogger {
  pub fn new() -> Self {
    Self {
      _moon: PhantomData,
    }
  }

  pub fn log_error<T: Debug>(&self, error: T) {
    let timestamp = DateTime::now().to_iso_8601_like();
    eprintln!("[{}] Discipline Error: {:?}", timestamp, error);
  }
}

// SAFETY: It's ok to send InternalErrorLogger across threads because
// it's just an empty struct.
unsafe impl Send for InternalErrorLogger {

}