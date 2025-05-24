#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(u32);

impl UserId {
  pub fn new(user_id: u32) -> Self {
    Self(user_id)
  }

  pub fn as_raw(&self) -> u32 {
    self.0
  }
}