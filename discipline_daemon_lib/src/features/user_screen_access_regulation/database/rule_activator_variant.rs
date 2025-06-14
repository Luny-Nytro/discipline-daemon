use super::{
  DeserializableScalarValue, SerializableScalarValue, GenericError,
  SerializeScalarValueContext, ScalarValue,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleActivatorVariant {
  AllTheTime,
  OnWeekday,
  InTimeRange,
  InWeekdayRange,
}

impl SerializableScalarValue for RuleActivatorVariant {
  fn write_into(&self, context: &mut SerializeScalarValueContext) -> Result<(), GenericError> {
    match self {
      RuleActivatorVariant::AllTheTime => context.write_u8(0),
      RuleActivatorVariant::OnWeekday => context.write_u8(1),
      RuleActivatorVariant::InTimeRange => context.write_u8(2),
      RuleActivatorVariant::InWeekdayRange => context.write_u8(3),
    }
  }
}

impl DeserializableScalarValue for RuleActivatorVariant {
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