use super::*;

pub fn generate_code_define_collection(
  code: &mut String,
  collection_specification: &CollectionSpecification,
) ->
  Result<(), GenericError>
{
  code.push_str("CREATE TABLE IF NOT EXISTS ");
  code.push_str(&collection_specification.fully_qualified_identifier);
  code.push_str(" (");

  let mut multi_column_primary_key_constraint = MlutiColumnPrimaryKeyConstraint::new();
  let mut did_write_a_column_definition = false;

  for column_specification in &collection_specification.column_specifications {
    if did_write_a_column_definition {
      code.push_str(", ");
    }

    code.push_str(&column_specification.fully_qualified_name);
    
    match column_specification.column_type {
      ColumnType::Primary => {
        if collection_specification.has_multiple_primary_key_columns {
          multi_column_primary_key_constraint.write(&column_specification);
        } else {
          code.push_str(" PRIMARY KEY");
        }
      }
      ColumnType::Optional => {
        // noop
      }
      ColumnType::Required => {
        code.push_str(" NOT NULL");
      }
      ColumnType::UniqueOptional => {
        code.push_str(" UNIQUE");
      }
      ColumnType::UniqueRequired => {
        code.push_str(" UNIQUE NOT NULL");
      }
    }

    did_write_a_column_definition = true;
  }

  if collection_specification.has_multiple_primary_key_columns {
    code.push_str(", ");
    code.push_str(&multi_column_primary_key_constraint.finish());
  }
  
  code.push_str(");");

  Ok(())
}


pub(super) fn generate_code_add_collection_item<Serializer>(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_serializer: &Serializer,
  new_collection_item: &Serializer::CompoundValue,
) ->
  Result<(), GenericError>
where 
  Serializer: CompoundValueSerializer
{
  let mut values_clause = String::new();
  serialize_compound_value_into(
    collection_item_serializer, 
    new_collection_item, 
    &mut values_clause
  )?; // TODO: do proper error handling

  code.push_str("INSERT INTO ");
  code.push_str(&collection_specification.fully_qualified_identifier);
  code.push_str(" ");
  code.push_str(&values_clause);
  code.push_str(";");

  Ok(())
}

pub(super) fn generate_code_delete_collection_item(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_matcher: &CollectionItemMatcher,
) ->
  Result<(), GenericError>
{
  code.push_str("DELETE FROM ");
  code.push_str(&collection_specification.fully_qualified_identifier);
  match &collection_item_matcher.inner {
    CollectionItemMatcherInner::NoWhereClause => {
      code.push_str(";");
    }
    CollectionItemMatcherInner::WhereClause(where_clause) => {
      code.push_str(" ");
      code.push_str(&where_clause);
      code.push_str(";");
    }
  }

  Ok(())
}

pub(super) fn generate_code_update_collection_item(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_matcher: &CollectionItemMatcher,
  collection_item_modifications: &CollectionItemModificationsDraft,
) -> 
  Result<(), GenericError>
{
  let Some(set_clause) = collection_item_modifications.finish() else {
    return Ok(());
  };

  code.push_str("UPDATE ");
  code.push_str(&collection_specification.fully_qualified_identifier);
  code.push_str(" ");
  code.push_str(&set_clause);
 
  match &collection_item_matcher.inner {
    CollectionItemMatcherInner::NoWhereClause => {
      code.push_str(";");
    }
    CollectionItemMatcherInner::WhereClause(where_clause) => {
      code.push_str(" ");
      code.push_str(&where_clause);
      code.push_str(";");
    }
  }

  Ok(())
}

pub(super) fn generate_code_find_all_collection_items(
  code: &mut String,
  collection_specification: &CollectionSpecification,
) -> 
  Result<(), GenericError>
{
  code.push_str("SELECT * FROM ");
  code.push_str(&collection_specification.fully_qualified_identifier);
  code.push_str(";");
  Ok(())
}

pub(super) fn generate_code_find_one_collection_item(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_matcher: &CollectionItemMatcher,
) -> 
  Result<(), GenericError>
{
  code.push_str("SELECT * FROM ");
  code.push_str(&collection_specification.fully_qualified_identifier);

  match &collection_item_matcher.inner {
    CollectionItemMatcherInner::NoWhereClause => {
      code.push_str(";");
    }
    CollectionItemMatcherInner::WhereClause(where_clause) => {
      code.push_str(" ");
      code.push_str(where_clause);
      code.push_str(";");
    }
  }

  Ok(())
}
