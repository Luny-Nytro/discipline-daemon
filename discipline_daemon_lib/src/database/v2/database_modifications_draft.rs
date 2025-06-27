use super::*;

pub struct DatabaseModificationsDraft {
  code: String,
}

impl DatabaseModificationsDraft {
  pub fn delete_items(
    &mut self, 
    collection: &Collection,
    item_matcher: &CollectionItemMatcher,
  ) -> 
    Result<(), GenericError>
  {
    todo!()
  }

  pub fn commit(&self, database: &Database) -> Result<(), GenericError> {
    todo!()
  }
}