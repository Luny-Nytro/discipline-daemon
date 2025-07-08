use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleActivatorType {
  AllTheTime,
  OnWeekday,
  InTimeRange,
  InWeekdayRange,
}

impl RuleActivatorType {
  pub fn to_number(&self) -> u8 {
    todo!()
  }

  pub fn from_number(number: u8) -> Result<Self, GenericError> {
    todo!()
  }
}

impl SerializableScalarValue for RuleActivatorType {
  fn serialize(&self, context: &mut SerializeScalarValueContext) {
    match self {
      RuleActivatorType::AllTheTime     => 0.serialize(context),
      RuleActivatorType::OnWeekday      => 1.serialize(context),
      RuleActivatorType::InTimeRange    => 2.serialize(context),
      RuleActivatorType::InWeekdayRange => 3.serialize(context),
    }
  }
}

impl DeserializableScalarValue for RuleActivatorType {
  fn deserialize(value: ScalarValue) -> Result<Self, GenericError> {
    let number = value.as_u8()?;

    match number {
      0 => Ok(RuleActivatorType::AllTheTime), 
      1 => Ok(RuleActivatorType::OnWeekday), 
      2 => Ok(RuleActivatorType::InTimeRange), 
      3 => Ok(RuleActivatorType::InWeekdayRange), 
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
