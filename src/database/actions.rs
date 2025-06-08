use crate::GenericError;
use super::{
  ScalarFieldSpecification, CollectionSpecification, SerializableScalarValue,
  serialize_scalar_value_into, ColumnType, ColumnSpecification,
  CompoundValueSerializer, serialize_compound_value_into,
};

pub struct CollectionItemModifications {
  code: String,
}

impl CollectionItemModifications {
  pub fn new() -> Self {
    Self {
      code: String::new(),
    }
  }

  fn did_write_a_modification(&self) -> bool {
    self.code.len() > 0
  }

  pub fn modify_scalar_field(
    &mut self, 
    scalar_field_specification: &ScalarFieldSpecification, 
    new_scalar_field_value: &impl SerializableScalarValue,
  ) ->
    Result<(), GenericError>
  {
    let mut temp = String::new();

    if let Err(error) = serialize_scalar_value_into(
      new_scalar_field_value, 
      &mut temp,
    ) {
      return Err(
        error
          .change_context("adding a new scalar field modification to CollectionItemModifications")
          .add_error("failed to serialize the new scalar field value")
          .add_attachment("scalar field specification", format!("{scalar_field_specification:?}"))
      )
    }

    if self.did_write_a_modification() {
      self.code.push_str(", ");
    } else {
      self.code.push_str("SET ");
    }

    self.code.push_str(&scalar_field_specification.fully_qualified_identifier);
    self.code.push_str(" = ");
    self.code.push_str(&temp);

    Ok(())
  }

  fn finish(&self) -> Option<&String> {
    Some(&self.code)
  }
}

// CollectionItemMatcher::match_all()
// CollectionItemMatcher::match_by_scalar_field()
// CollectionItemMatcher::match_by_multiple_scalar_fields()
// CollectionItemMatcher::match_by_either_scalar_field()

enum CollectionItemMatcherInner {
  NoWhereClause,
  WhereClause(String),
}

// enum CollectionItemMatchOperation {
//   All,
//   Or(Vec<CollectionItemMatchOperation>),
//   And(Vec<CollectionItemMatchOperation>),
// }

struct CollectionItemAndMatchWriter {
  code: String,
}

impl CollectionItemAndMatchWriter {
  fn new() -> Self {
    Self { 
      code: String::new()
    }
  }

  fn did_write_a_match(&self) -> bool {
    self.code.len() > 0
  }

  pub fn and_scalar_field_is(
    &mut self, 
    scalar_field_specification: &ScalarFieldSpecification,
    scalar_field_value: &impl SerializableScalarValue,
  ) -> 
    Result<(), GenericError>
  {
    let mut temp = String::new();
    if let Err(error) = serialize_scalar_value_into(scalar_field_value, &mut temp) {
      return Err(
        // TODO: Use proper error messages
        error
          .change_context("creating a collection item matcher that matches based a single scalar field value")
          .add_error("failed to serialize scalar field value")
          .add_attachment("scalar field specification", format!("{scalar_field_specification:?}"))
      );
    }

    if self.did_write_a_match() {
      self.code.push_str(" AND ");
    } else {
      self.code.push_str("WHERE ");
    }

    self.code.push_str(&scalar_field_specification.fully_qualified_identifier);
    self.code.push_str(" = ");
    self.code.push_str(&temp);

    Ok(())
  }

  fn finish(self) -> Result<CollectionItemMatcher, GenericError> {
    Ok(CollectionItemMatcher {
      inner: if self.did_write_a_match() {
        CollectionItemMatcherInner::WhereClause(self.code)
      } else {
        CollectionItemMatcherInner::NoWhereClause
      }
    })
  }
}

pub struct CollectionItemMatcher {
  inner: CollectionItemMatcherInner
}

impl CollectionItemMatcher {
  pub fn match_all() -> CollectionItemMatcher {
    CollectionItemMatcher {
      inner: CollectionItemMatcherInner::NoWhereClause
    }
  }

  pub fn match_by_scalar_field(
    scalar_field_specification: &ScalarFieldSpecification,
    scalar_field_value: &impl SerializableScalarValue,
  ) -> 
    Result<CollectionItemMatcher, GenericError>
  {
    let mut code = String::new();
    code.push_str("WHERE ");
    code.push_str(&scalar_field_specification.fully_qualified_identifier);
    code.push_str(" = ");
    serialize_scalar_value_into(scalar_field_value, &mut code)
      .map_err(|error|
        error
          .change_context("creating a collection item matcher that matches based a single scalar field value")
          .add_error("failed to serialize scalar field value")
          .add_attachment("scalar field specification", format!("{scalar_field_specification:?}"))
      )?;

    Ok(CollectionItemMatcher {
      inner: CollectionItemMatcherInner::WhereClause(code)
    })
  }

  pub fn match_by_multiple_scalar_fields() -> CollectionItemAndMatchWriter {
    CollectionItemAndMatchWriter::new()
  }
}

pub struct CompoundTypeSpecifcationProviderContext {
  column_specifications: Vec<ColumnSpecification>
}

impl CompoundTypeSpecifcationProviderContext {
  fn new() -> Self {
    Self {
      column_specifications: Vec::new(),
    }
  }

  pub fn write_field(&mut self, scalar_field_description: &ScalarFieldSpecification) -> Result<(), GenericError> {
    if self
      .column_specifications
      .iter()
      .any(|column_specification| column_specification.fully_qualified_name == scalar_field_description.fully_qualified_identifier)
    {
      // TODO: return a proper GenericError. I AM TOOOO TIRED.
      return Err(todo!());
    }
    
    self.column_specifications.push(ColumnSpecification {
      column_type: ColumnType::Required,
      fully_qualified_name: scalar_field_description.fully_qualified_identifier.clone(),
    });

    Ok(())
  }
}

pub trait ProvidesCollectionItemSpecification {
  fn write_item_specification(&self, context: &mut WriteCompoundTypeSpecificationContext) -> Result<(), GenericError>;
  fn do_special_thingies();
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

  fn write(&mut self, column_specification: &ColumnSpecification) {
    if self.code.is_empty() {
      self.code.push_str("PRIMARY KEY(");
    } else {
      self.code.push_str(", ");
    }

    self.code.push_str(&column_specification.fully_qualified_name);
  }

  fn finish(mut self) -> String {
    self.code.push_str(")");
    self.code
  }
}

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

pub struct WriteCompoundTypeSpecificationContext {
  column_specifications: Vec<ColumnSpecification>
}

impl WriteCompoundTypeSpecificationContext {
  fn new() -> Self {
    Self {
      column_specifications: Vec::new(),
    }
  }

  pub fn write_field(&mut self, scalar_field_description: &ScalarFieldSpecification) -> Result<(), GenericError> {
    self.column_specifications.push(ColumnSpecification {
      column_type: ColumnType::Required,
      fully_qualified_name: scalar_field_description.fully_qualified_identifier.clone(),
    });

    Ok(())
  }

  pub fn mark_field_as_primary(&mut self, scalar_field_description: &ScalarFieldSpecification) -> Result<(), GenericError> {
    let Some(column_specification) = self
      .column_specifications
      .iter_mut()
      .find(|column_specification| 
        column_specification.fully_qualified_name 
        == 
        scalar_field_description.fully_qualified_identifier
      ) else 
    {
      return Err(
        GenericError::new("marking a filed as a primary field")
          .add_error("unknown field")
          .add_attachment("fully qualified field name", &scalar_field_description.fully_qualified_identifier)
      )
    };
   
    column_specification.column_type = ColumnType::Primary;
    Ok(())
  }
}

pub trait IsCompoundTypeSpecifcation {
  fn write_fields_into(&self, writer: &mut CompoundTypeSpecifcationProviderContext) -> Result<(), GenericError>;
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
  collection_item_modifications: &CollectionItemModifications,
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

pub struct DatabaseSpecificationsProviderContext<'a> {
  code: &'a mut String,
}

impl<'a> DatabaseSpecificationsProviderContext<'a> {
  fn new(code: &'a mut String) -> Self {
    Self { code }
  }

  pub fn add_collection_specification(
    &mut self, 
    collection_specification: &CollectionSpecification,
  ) -> 
    Result<(), GenericError>
  {
    // if, in the future, this actually fails, handle the error and change the context
    generate_code_define_collection(&mut self.code, collection_specification)
  }
}

pub trait DatabaseSpecificationsProvider {
  fn add_specifications(&self, context: &mut DatabaseSpecificationsProviderContext) -> Result<(), GenericError>;
}

pub(super) fn generate_code_define_database_schema(
  code: &mut String,
  database_specifications_provider: &impl DatabaseSpecificationsProvider,
) ->
  Result<(), GenericError>
{
  // TODO: Retrun an error if the providers adds zero collection specifications
  let mut context = DatabaseSpecificationsProviderContext::new(code);
  database_specifications_provider
    .add_specifications(&mut context)
    .map_err(|error| error.change_context("generate sql code that initializes the database schema, which are tables, triggers and views"))
}