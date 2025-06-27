use super::*;

#[derive(Debug, Clone)]
pub struct Collection {
  path: Path,
  columns: Vec<Column>,
  identifier: Identifier,
  primary_columns_number: usize,
}

impl Collection {
  pub(super) fn new(
    path: Path, 
    identifier: Identifier,
    item_definer: CollectionItemDefiner,
  ) -> Self {
    Self {
      path,
      identifier,
      columns: item_definer.columns,
      primary_columns_number: item_definer.primary_columns_number,
    }
  }

  fn path(&self) -> &Path {
    &self.path
  }
}


struct MlutiColumnPrimaryKeyConstraint {
  code: String,
}

impl MlutiColumnPrimaryKeyConstraint {
  fn new() -> Self {
    Self {
      code: String::new()
    }
  }

  fn write(&mut self, column: &Column) {
    if self.code.is_empty() {
      self.code.push_str("PRIMARY KEY(");
    } else {
      self.code.push_str(", ");
    }

    self.code.push_str(column.path().to_sql_identifier_string());
  }

  fn finish(mut self) -> String {
    self.code.push_str(")");
    self.code
  }
}

fn generate_code_define_collection(
  code: &mut String,
  collection: &Collection,
) ->
  Result<(), GenericError>
{
  code.push_str("CREATE TABLE IF NOT EXISTS ");
  code.push_str(collection.path().to_sql_identifier_str());
  code.push_str(" (");

  let mut multi_column_primary_key_constraint = MlutiColumnPrimaryKeyConstraint::new();
  let mut did_write_a_column_definition = false;

  for column in &collection.columns {
    if did_write_a_column_definition {
      code.push_str(", ");
    }

    code.push_str(column.path().to_sql_identifier_str());
    
    match column.semantics() {
      ColumnSemantics::Primary => {
        if collection.primary_columns_number > 1 {
          multi_column_primary_key_constraint.write(&column);
        } else {
          code.push_str(" PRIMARY KEY");
        }
      }
      ColumnSemantics::Optional => {
        // noop
      }
      ColumnSemantics::Required => {
        code.push_str(" NOT NULL");
      }
      ColumnSemantics::UniqueOptional => {
        code.push_str(" UNIQUE");
      }
      ColumnSemantics::UniqueRequired => {
        code.push_str(" UNIQUE NOT NULL");
      }
    }

    did_write_a_column_definition = true;
  }

  if collection.primary_columns_number > 0 {
    code.push_str(", ");
    code.push_str(&multi_column_primary_key_constraint.finish());
  }
  
  code.push_str(");");

  Ok(())
}

fn generate_code_add_collection_item<Serializer>(
  code: &mut String,
  collection: &Collection,
  item_serializer: &Serializer,
  item: &Serializer::CompoundType,
) ->
  Result<(), GenericError>
where 
  Serializer: CompoundTypeSerializer
{
  let mut values_clause = String::new();

  serialize_compound_value_into(
    item_serializer, 
    item, 
    &mut values_clause
  )?; // TODO: do proper error handling

  code.push_str("INSERT INTO ");
  code.push_str(collection.path().to_sql_identifier_str());
  code.push_str(" ");
  code.push_str(&values_clause);
  code.push_str(";");

  Ok(())
}

fn generate_code_delete_collection_item(
  code: &mut String,
  collection: &Collection,
  item_matcher: &CollectionItemMatcher,
) ->
  Result<(), GenericError>
{
  code.push_str("DELETE FROM ");
  code.push_str(collection.path().to_sql_identifier_string());

  match &item_matcher.inner {
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

fn generate_code_update_collection_item(
  code: &mut String,
  collection: &Collection,
  item_matcher: &CollectionItemMatcher,
  item_changes: &CollectionItemModificationsDraft,
) -> 
  Result<(), GenericError>
{
  let Some(set_clause) = item_changes.finish() else {
    // TODO: Maybe return an error here
    return Ok(());
  };

  code.push_str("UPDATE ");
  code.push_str(collection.path().to_sql_identifier_string());
  code.push_str(" ");
  code.push_str(&set_clause);
 
  match &item_matcher.inner {
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

fn generate_code_find_all_collection_items(
  code: &mut String,
  collection: &Collection,
) -> 
  Result<(), GenericError>
{
  code.push_str("SELECT * FROM ");
  code.push_str(collection.path().to_sql_identifier_str());
  code.push_str(";");
  Ok(())
}

fn generate_code_find_one_collection_item(
  code: &mut String,
  collection: &Collection,
  collection_item_matcher: &CollectionItemMatcher,
) -> 
  Result<(), GenericError>
{
  code.push_str("SELECT * FROM ");
  code.push_str(collection.path().to_sql_identifier_str());

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

impl Collection {
  pub fn retrieve_one_item<Deserializer>(
    &self,
    database: &Database,
    item_matcher: &CollectionItemMatcher,
    item_deserializer: &Deserializer,
  ) ->
    Result<Option<Deserializer::Output>, GenericError>
  where 
    Deserializer: CompoundValueDeserializer
  {
    let mut code = String::new();
    generate_code_find_one_collection_item(
      &mut code,
      self,
      item_matcher,
    ).map_err(|error|
      error.change_context("retrieving one collection item")
    )?;

    let mut statement = database.connection.prepare(&code).map_err(|error|
      GenericError::new("creating sqlite query statement")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retreiving one collection item")
        .add_attachment("collection", self.path.to_displayable_string())
    )?;
    
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("create sqlite query iterator")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retreiving one collection item")
        .add_attachment("collection", self.path.to_displayable_string())
    )?;

    loop {
      let row = iterator.next().map_err(|error|
        GenericError::new("getting the next item of a sqlite query iterator")
          .add_attachment("error", error.to_string())
          .add_attachment("code", code.clone())
          .change_context("retreiving one collection item")
          .add_attachment("collection", self.path.to_displayable_string())
      )?;

      let Some(row) = row else {
        return Ok(None)
      };

      return deserialize_compound_value(
        row, 
        item_deserializer,
      )
      .map(Some)
      .map_err(|error|
        error
          .change_context("retrieving one collection item")
          .add_attachment("collection", self.path.to_displayable_string())
      );
    }
  }

  pub fn retrieve_all_items<Deserializer>(
    &self,
    database: &Database,
    item_deserializer: &Deserializer,
  ) ->
    Result<Vec<Deserializer::Output>, GenericError>
  where 
    Deserializer: CompoundValueDeserializer
  {
    let mut code = String::new();
    generate_code_find_all_collection_items(
      &mut code, 
      self,
    ).map_err(|error|
      error
        .change_context("retrieving all the items of a collection")
    )?;

    let mut statement = database.connection.prepare(&code).map_err(|error|
      GenericError::new("creating sqlite query statement")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retrieving all the items of a collection")
        .add_attachment("collection", self.path.to_displayable_string())
    )?;
    
    let mut iterator = statement.query(()).map_err(|error|
      GenericError::new("creating sqlite iterator")
        .add_attachment("error", error.to_string())
        .add_attachment("code", code.clone())
        .change_context("retrieving all the items of a collection")
        .add_attachment("collection", self.path.to_displayable_string())
    )?;

    let mut items = Vec::new();
    loop {
      let row = iterator.next().map_err(|error|
        GenericError::new("retrieving the next item of a sqlite row iterator")
          .add_attachment("error", error.to_string())
          .add_attachment("code", code.clone())
          .change_context("retrieving all the items of a collection")
          .add_attachment("collection", self.path.to_displayable_string())
      )?;

      let Some(row) = row else {
        break;
      };

      let item = deserialize_compound_value(
        row, 
        item_deserializer,
      ).map_err(|error|
        error
          .change_context("deserializing a collection item")
          .change_context("retrieving all the items of a collection")
          .add_attachment("collection", self.path.to_displayable_string())
      )?;

      items.push(item);
    }

    Ok(items)
  }

  pub fn update_items(
    &self,
    database: &Database,
    item_matcher: &CollectionItemMatcher,
    item_changes: &CollectionItemModificationsDraft,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
    generate_code_update_collection_item(
      &mut code, 
      self, 
      item_matcher, 
      item_changes,
    ).map_err(|error|
      error.change_context("updating collection items")
    )?;

    database.execute(&code).map_err(|error| 
      error
        .change_context("updating collection items")
        .add_attachment("collection", self.path.to_displayable_string())
    )
  }

  pub fn delete_items(
    &self,
    database: &Database,
    item_matcher: &CollectionItemMatcher,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
    generate_code_delete_collection_item(
      &mut code, 
      self, 
      item_matcher, 
    ).map_err(|error|
      error.change_context("deleting collection items")
    )?;

    database.execute(&code).map_err(|error| 
      error
        .change_context("deleting collection items")
        .add_attachment("collection", self.path.to_displayable_string())
    )
  }

  pub fn add_item<Serializer: CompoundTypeSerializer>(
    &self,
    database: &Database,
    item_serializer: &Serializer,
    item: &Serializer::CompoundType,
  ) -> 
    Result<(), GenericError> 
  {
    let mut code = String::new();
    generate_code_add_collection_item(
      &mut code, 
      self, 
      item_serializer,
      item, 
    ).map_err(|error|
      error.change_context("adding a new item to collection")
    )?;

    database.execute(&code).map_err(|error| 
      error
        .change_context("adding a new item to collection")
        .add_attachment("collection", self.path.to_displayable_string())
    )
  }

  pub fn create_modifications_draft(&self) -> CollectionItemModificationsDraft {
    todo!()
  }
  pub fn commit_modifications_draft(
    &self, 
    database: &Database, 
    modifications_draft: &CollectionItemModificationsDraft,
    item_matcher: &CollectionItemMatcher

  ) -> 
    Result<(), GenericError>
  {
    todo!()
  }
  // pub fn initialize_database_schema(
  //   &self,
  //   database_specifications_provider: &impl DatabaseSpecificationsProvider,
  // ) -> 
  //   Result<(), GenericError>
  // {
  //   let mut code = String::new();
  //   generate_code_define_database_schema(
  //     &mut code, 
  //     database_specifications_provider,
  //   )
  //   .and_then(|_|
  //     self.execute(&code)
  //   )
  //   .map_err(|error|
  //     error.change_context("initializing database schema")
  //   )
  // }
}

// // Examoles
// pub struct CompoundTypeExample {
//   a: Field,
//   b: Field,
//   c: Field,
//   d: Field,
// }

// impl IsCompoundType for CompoundTypeExample {
//   fn new(definer: &mut CompoundTypeDefiner) -> Tried<Self, GenericError> {
//     Ok(Self {
//       a: definer.readonly_required_field("a")?,
//       b: definer.readonly_required_field("b")?,
//       c: definer.readonly_required_field("c")?,
//       d: definer.readonly_required_field("d")?,
//     })
//   }
// }

// pub struct CollectionItemExample {
//   id: Field,
//   name: Field,
//   compound_field: CompoundTypeExample,
// }

// impl IsCollectionItem for CollectionItemExample {
//   fn new(definer: &mut CollectionItemDefiner) -> Result<Self, GenericError> {
//     Ok(Self {
//       id: definer.primary_scalar_field("id")?,
//       name: definer.writable_required_field("name")?,
//       compound_field: definer.compound_field("lunar")?,
//     })
//   }
// }

// pub struct NamespaceExample {
//   rule_collection: Collection,
//   rule_collection_item: CollectionItemExample,
// }

// impl IsNamespace for NamespaceExample {
//   fn new(definer: &mut DatabaseDefiner) -> Result<Self, GenericError> {
//     let (rule_collection, rule_collection_item) = definer.collection("rules")?;

//     Ok(Self {
//       rule_collection,
//       rule_collection_item,
//     })
//   }

//   // update_rule_name
//   // update_rule_description
//   // create_rule
//   // delete_rule
//   // d_rule
// }



