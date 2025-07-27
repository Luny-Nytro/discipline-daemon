mod state;
use std::ops::{Deref, DerefMut};

pub use state::*;

mod user;
mod user_screen_access_regulation;


pub struct Auto<T> {
  pointer: *mut T
}

impl<T> Deref for Auto<T> {
  type Target = T;

  fn deref(&self) -> &T {
    unsafe {
      &*self.pointer
    }
  }
}

impl<T> DerefMut for Auto<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe {
      &mut *self.pointer
    }
  }
}

pub fn allocate<T>(value: T) -> Auto<T> {
  Auto(Box::into_raw(Box::new(value)))
}