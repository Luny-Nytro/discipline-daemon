pub trait Matches<T = super::Value> {
  fn matches(&self, value: &T) -> bool;
}