pub trait ToPublicRepr {
  type PublicRepr;
  
  fn to_public_repr(&mut self) -> Self::PublicRepr;
}

impl<T> ToPublicRepr for Vec<T> 
where 
  T: ToPublicRepr
{
  type PublicRepr = Vec<T::PublicRepr>;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    let mut items = Vec::with_capacity(self.len());
    for item in self {
      items.push(item.to_public_repr());
    }
    items
  }
}