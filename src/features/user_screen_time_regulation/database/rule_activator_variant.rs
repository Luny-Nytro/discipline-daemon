use crate::{database::{ColumnValue, DeserializableScalarValue, SerializableScalarValue, SerializeScalarValueContext}, GenericError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleActivatorVariant {
  AllTheTime,
  OnWeekday,
  InTimeRange,
  InWeekdayRange,
}

impl SerializableScalarValue for RuleActivatorVariant {
  fn serialize_into(&self, context: SerializeScalarValueContext) {
    match self {
      RuleActivatorVariant::AllTheTime => context.as_u8(0),
      RuleActivatorVariant::OnWeekday => context.as_u8(1),
      RuleActivatorVariant::InTimeRange => context.as_u8(2),
      RuleActivatorVariant::InWeekdayRange => context.as_u8(3),
    } 
  }
}

impl DeserializableScalarValue for RuleActivatorVariant {
  fn deserialize(value: ColumnValue) -> Result<Self, GenericError> {
    let number = value.as_u8().map_err(|error|
      error.change_context("Failed to deserialize RuleActivatorVariant: Failed to cast value to u8")
    )?;

    match number {
      0 => Ok(RuleActivatorVariant::AllTheTime), 
      1 => Ok(RuleActivatorVariant::OnWeekday), 
      2 => Ok(RuleActivatorVariant::InTimeRange), 
      3 => Ok(RuleActivatorVariant::InWeekdayRange), 
      _ => {
        Err(
          GenericError::new("Failed to deserialize RuleActivatorVariant: Unknown variant")
            .add_attachment("variant", number.to_string())
        )
      }
    }  
  }
}