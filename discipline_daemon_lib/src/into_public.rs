pub trait IntoPublic {
  type Output;
  
  fn into_public(self) -> Self::Output;
}