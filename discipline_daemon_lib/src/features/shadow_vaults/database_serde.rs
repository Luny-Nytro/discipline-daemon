use crate::database::{
  Deserialize, DeserializeContext, ScalarTypeError, 
  ScalarTypeSerde, SerializationContext, 
  Serialize, ValueRef, 
};

use super::{
  ShadowVault, ShadowVaultName, 
  ShadowVaultDatum, 
};

use crate::{
  countdown_timer, CountdownTimer
};

// SECTION: ShdaowVaultName
impl ScalarTypeSerde for ShadowVaultName {
  fn serialize(&self) -> String {
    self.as_ref().serialize()
  }

  fn deserialize(value: ValueRef) -> Option<Self> {
    let Some(name) = String::deserialize(value) else {
      return None;
    };

    ShadowVaultName::new(name).ok()
  }
}

// SECTION: ShadowVaultDatum
impl ScalarTypeSerde for ShadowVaultDatum {
  fn serialize(&self) -> String {
    self.as_ref().serialize()
  }

  fn deserialize(value: ValueRef) -> Option<Self> {
    let Some(name) = String::deserialize(value) else {
      return None;
    };

    ShadowVaultDatum::new(name).ok()
  }
}

// SECTION: Protector
// pub enum ProtectorType {
//   ForDuration,
//   ByPassword,
// }

// impl ScalarTypeSerde for ProtectorType {
//   fn serialize(&self) -> String {
//     match self {
//       Self::ForDuration => 0.serialize(),
//       Self::ByPassword => 1.serialize(),        
//     }
//   }

//   fn deserialize(value: ValueRef) -> Option<Self> {
//     match value {
//       ValueRef::Integer(0) => Some(Self::ForDuration), 
//       ValueRef::Integer(1) => Some(Self::ByPassword), 
//       _ => None,
//     }
//   }
// }

// impl Serialize for Protector {
//   fn serialize_into(&self, ctx: &mut SerializationContext) {
//     match self {
//       Self::ForDuration(countdown_timer) => {
//         ctx.scalar(&ProtectorType::ForDuration);
//         ctx.compound(countdown_timer);
//       } 
//       Self::ByPassword { is_protected, password } => {
//         ctx.scalar(&ProtectorType::ByPassword);
//         ctx.scalar(is_protected);
//         ctx.scalar(password);
//         ctx.null();
//       }
//     }
//   }
// }

// #[derive(Debug)]
// pub enum ProtectorDeserializeError {
//   Type(ScalarTypeError),
//   ForDuration(countdown_timer::database_serde::DeserializeError),
//   ByPasswordIsProtected(ScalarTypeError),  
//   ByPasswordPassword(ScalarTypeError),  
// }

// impl Deserialize for Protector {
//   type Error = ProtectorDeserializeError;

//   fn columns_number() -> usize {
//     // Type
//     1 
//     +
//     CountdownTimer::columns_number()
//       .max(2)
//   }

//   fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
//     let r#type = match ctx.scalar_type() {
//       Ok(value) => value,
//       Err(error) => return Err(ProtectorDeserializeError::Type(error))
//     };

//     match r#type {
//       ProtectorType::ForDuration => {
//         match ctx.compund_type() {
//           Ok(countdown_timer) => Ok(Protector::ForDuration(countdown_timer)),
//           Err(error) => Err(ProtectorDeserializeError::ForDuration(error)),
//         }
//       }
//       ProtectorType::ByPassword => {
//         Ok(Protector::ByPassword { 
//           is_protected: match ctx.scalar_type() {
//             Ok(value) => value,
//             Err(error) => return Err(ProtectorDeserializeError::ByPasswordIsProtected(error)),
//           }, 
//           password: match ctx.scalar_type() {
//             Ok(value) => value,
//             Err(error) => return Err(ProtectorDeserializeError::ByPasswordPassword(error)),
//           }
//         })
//       }
//     }
//   }
// }

// SECTION: ShadowVault
impl Serialize for ShadowVault {
  fn serialize_into(&self, ctx: &mut SerializationContext) {
    ctx.scalar(&self.id);
    ctx.scalar(&self.name);
    ctx.scalar(&self.datum);
    ctx.compound(&self.protector);
  }
}

#[derive(Debug)]
pub enum ShadowVaultDeserializeError {
  Id(ScalarTypeError),
  Name(ScalarTypeError),
  Datum(ScalarTypeError),
  Protector(countdown_timer::database_serde::DeserializeError),
}

impl Deserialize for ShadowVault {
  type Error = ShadowVaultDeserializeError;

  fn columns_number() -> usize {
    // id, name and datum
    3 
    +
    CountdownTimer::columns_number()
  }

  fn deserialize(ctx: &mut DeserializeContext) -> Result<Self, Self::Error> {
    Ok(ShadowVault {
      id: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(ShadowVaultDeserializeError::Id(error)),
      },
      name: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(ShadowVaultDeserializeError::Name(error)),
      },
      datum: match ctx.scalar_type() {
        Ok(value) => value,
        Err(error) => return Err(ShadowVaultDeserializeError::Datum(error)),
      },
      protector: match ctx.compund_type() {
        Ok(value) => value,
        Err(error) => return Err(ShadowVaultDeserializeError::Protector(error)),
      },
    })
  }
}