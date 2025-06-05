use crate::GenericError;

fn verify_identifier(identifier: &str) -> Result<(), GenericError> {
  // // Check if it contains any underscores — disallowed in your case
  // if identifier.contains('_') {
  //   return Err(
  //     GenericError::new("verify identifier")
  //       .add_error("identifier contains an underscore: underscores are reserved by this program for namespacing")
  //       .add_attachment("identifier", identifier)
  //   );
  // }

  // Check if the first character is a valid starting character
  let mut characters = identifier.chars();
  let beginning = match characters.next() {
    Some(character) => {
      character
    }
    None => {
      return Err(
        GenericError::new("verify identifier")
          .add_error("identifier is the empty string")
      );
    }
  };

  if !beginning.is_ascii_alphabetic() {
    return Err(
      GenericError::new("verify identifier")
        .add_error("identifier begins with a character that is not a ascii alphabetic character, which is U+0041 'A' ..= U+005A 'Z' or U+0061 'a' ..= U+007A 'z'")
        .add_attachment("identifier", identifier)
    );
  }

  // Check the rest of the characters are alphanumeric only
  if !characters.all(|character| character.is_ascii_alphanumeric()) {
    return Err(
      GenericError::new("verify identifier")
        .add_error("identifier contains non-alphanumeric characters")
        .add_attachment("identifier", identifier)
    );
  }

  Ok(())
}

#[derive(Debug)]
pub struct ScalarFieldSpecification {
  pub fully_qualified_identifier: String,
  pub optional: bool,
  pub writeable: bool,
}

pub struct ScalarFieldSpecificationBuilder {
  fully_qualified_name: String,
  optional: bool,
  writeable: bool,
}

impl ScalarFieldSpecificationBuilder {
  fn new(name: String, optional: bool) -> Self {
    Self {
      fully_qualified_name: name,
      optional,
      writeable: false,
    }
  }

  pub fn optional(mut self) -> Self {
    self.optional = true;
    self
  }

  pub fn writeable(mut self) -> Self {
    self.writeable = true;
    self
  }

  pub fn build(self) -> Result<ScalarFieldSpecification, GenericError> {
    verify_identifier(&self.fully_qualified_name)
      .map_err(|error| 
        error
          .change_context("verify field identifier")
          .change_context("create scalar field specification from builder")
      )?;

    Ok(ScalarFieldSpecification {
      fully_qualified_identifier: self.fully_qualified_name,
      optional: self.optional,
      writeable: self.writeable,
    })
  }
}

pub struct CompoundTypeSpecificationCreator {
  fully_qualified_name: String,
  optional: bool,
}

impl CompoundTypeSpecificationCreator {
  pub fn new() -> Self {
    Self {
      fully_qualified_name: String::new(),
      optional: false,
    }
  }

  pub fn optional(mut self) -> Self {
    self.optional = true;
    self
  }

  pub fn scalar_field_specification(&self, identifier: &str) -> ScalarFieldSpecificationBuilder {
    ScalarFieldSpecificationBuilder::new(
      format!("{}_{}", self.fully_qualified_name, identifier), 
      self.optional,
    )
  }

  pub fn compound_field_specification(&self, identifier: &str) -> Result<CompoundTypeSpecificationCreator, GenericError> {
    verify_identifier(identifier)
      .map(|_| CompoundTypeSpecificationCreator { 
        fully_qualified_name: format!("{}_{}", self.fully_qualified_name, identifier), 
        optional: self.optional,
      })
      .map_err(|error|
        error
          // TODO: update these error messages
          .change_context("verify namespace name")
          .change_context("create namespace")
      )
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ColumnType {
  Primary, 
  UniqueRequired,
  UniqueOptional,
  Optional,
  Required,
}

pub struct ColumnSpecification {
  pub(super) fully_qualified_name: String,
  pub(super) column_type: ColumnType,
}

pub trait CompoundTypeSpecificationProvider {
  fn add_fields(&self, context: &mut CompoundTypeFieldsSpecification) -> Result<(), GenericError>;
}

pub struct CompoundTypeFieldsSpecification {
  column_specifications: Vec<ColumnSpecification>,
}

impl CompoundTypeFieldsSpecification {
  pub fn add_scalar_field(&mut self, scalar_field_specification: &ScalarFieldSpecification) -> Result<(), GenericError> {
    if self
      .column_specifications
      .iter()
      .any(|spec| spec.fully_qualified_name == scalar_field_specification.fully_qualified_identifier)
    {
      return Err(todo!());
    }

    self.column_specifications.push(ColumnSpecification {
      column_type: ColumnType::Required,
      fully_qualified_name: scalar_field_specification.fully_qualified_identifier.clone(),
    });

    Ok(())
  }

  pub fn add_compound_field(
    &mut self, 
    compound_field_specification_provider: &impl CompoundTypeSpecificationProvider,
  ) -> 
    Result<(), GenericError>
  {
    compound_field_specification_provider.add_fields(self)
  }
}

pub struct CollectionSpecfication {
  pub(super) identifier: String,
  pub(super) fully_qualified_identifier: String,
  pub(super) column_specifications: Vec<ColumnSpecification>,
  pub(super) has_multiple_primary_key_columns: bool,
}

impl CollectionSpecfication {
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

pub struct Namespace {
  pub(super) identifier: String,
  pub(super) fully_qualified_identifier: String,
}

impl Namespace {
  pub fn namespace(&self, identifier: &str) -> Result<Namespace, GenericError> {
    verify_identifier(identifier)
      .map(|_| 
        Namespace { 
          identifier: identifier.into(), 
          fully_qualified_identifier: format!("{}_{}", self.fully_qualified_identifier, identifier),
        }
      )
      // TODO: Do proper error handling
  }

  pub fn collection(
    &self, 
    collection_identifier: &str,
    collection_item_fields_specification: CompoundTypeFieldsSpecification,
  ) -> 
    Result<CollectionSpecfication, GenericError> 
  {
    // TODO: do proper error handling

    if let Err(error) = verify_identifier(collection_identifier) {
      return Err(error);
    }

    if collection_item_fields_specification.column_specifications.is_empty() {
      return Err(todo!());
    }

    Ok(CollectionSpecfication::new(
      collection_identifier.into(), 
      format!("{}_{}", self.fully_qualified_identifier, collection_identifier), 
      collection_item_fields_specification.column_specifications,
    ))
  }
}

pub struct Database {}

impl Database {
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn namespace(&self, identifier: &str) -> Result<Namespace, GenericError> {
    verify_identifier(identifier)
      .map(|_|
        Namespace {
          identifier: identifier.into(),
          fully_qualified_identifier: identifier.into(),
        }
      )
    // TODO: do proper error handling
  }
}