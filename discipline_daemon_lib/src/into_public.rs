pub trait IntoPublic {
  type Output;
  
  fn into_public(self) -> Self::Output;
}

impl<T> IntoPublic for Vec<T>
where 
  T: IntoPublic
{
  type Output = Vec<T::Output>;

  fn into_public(self) -> Self::Output {
    self.into_iter().map(IntoPublic::into_public).collect()
  }
}