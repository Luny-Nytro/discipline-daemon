use super::*;

pub struct CollectionSpecification {
  pub(super) identifier: String,
  pub(super) fully_qualified_identifier: String,
  pub(super) column_specifications: Vec<ColumnSpecification>,
  pub(super) has_multiple_primary_key_columns: bool,
}

impl CollectionSpecification {
  fn new(
    identifier: String,
    fully_qualified_identifier: String,
    column_specifications: Vec<ColumnSpecification>,
  ) -> Self {
    let mut primary_key_columns_count = 0;
    for column_specification in &column_specifications {
      if column_specification.column_type == ColumnType::Primary {
        primary_key_columns_count += 1;
      }
      if primary_key_columns_count > 1 {
        break;
      }
    }

    Self {
      identifier,
      fully_qualified_identifier,
      column_specifications,
      has_multiple_primary_key_columns: primary_key_columns_count > 1,
    }
  }
}
