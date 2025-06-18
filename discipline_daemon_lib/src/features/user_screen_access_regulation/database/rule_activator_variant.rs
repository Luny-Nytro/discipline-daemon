use super::{
  FromScalarValue, IntoScalarValue, GenericError,
  IsScalarValue, ScalarValue,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleActivatorVariant {
  AllTheTime,
  OnWeekday,
  InTimeRange,
  InWeekdayRange,
}

impl IntoScalarValue for RuleActivatorVariant {
  fn into_scalar_value(&self) -> impl IsScalarValue {
    match self {
      RuleActivatorVariant::AllTheTime => 0,
      RuleActivatorVariant::OnWeekday => 1,
      RuleActivatorVariant::InTimeRange => 2,
      RuleActivatorVariant::InWeekdayRange => 3,
    }
  }
}

impl FromScalarValue for RuleActivatorVariant {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    let number = value.as_u8().map_err(|error|
      error.change_context("deserializing RuleActivatorVariant")
    )?;

    match number {
      0 => Ok(RuleActivatorVariant::AllTheTime), 
      1 => Ok(RuleActivatorVariant::OnWeekday), 
      2 => Ok(RuleActivatorVariant::InTimeRange), 
      3 => Ok(RuleActivatorVariant::InWeekdayRange), 
      _ => {
        Err(
          GenericError::new("deserializing RuleActivatorVariant")
            .add_error("unknown variant number")
            .add_attachment("variant", number.to_string())
            .add_attachment("known variant numbers", "0, 1, 2, and 3")
        )
      }
    }  
  }
}