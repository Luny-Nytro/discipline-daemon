use super::*;

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