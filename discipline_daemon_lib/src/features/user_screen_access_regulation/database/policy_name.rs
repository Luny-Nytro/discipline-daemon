use super::{
  PolicyName, IntoScalarValue, FromScalarValue,
  IsScalarValue, ScalarValue,
};

impl IntoScalarValue for PolicyName {
  fn into_scalar_value(&self) -> impl IsScalarValue {
    self.as_ref()
  }
}

impl FromScalarValue for PolicyName {
  fn deserialize(value: ScalarValue) -> Result<Self, crate::GenericError> {
    value
      .as_string()
      .and_then(PolicyName::new)
      .map_err(|error| error.change_context("deserializing PolicyName"))
  }
}