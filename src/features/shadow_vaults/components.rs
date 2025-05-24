use serde::{Deserialize, Serialize};
use crate::{CountdownTimer, DateTime, Duration, Uuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateShadowVaultNameError {
  NameTooLong,
  NameTooShort,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowVaultName(String);

impl ShadowVaultName {
  pub const MIN_LENGTH: usize = 0;
  pub const MAX_LENGTH: usize = 20;

  pub fn new(name: String) -> Result<ShadowVaultName, CreateShadowVaultNameError> {
    if name.len() < Self::MIN_LENGTH {
      Err(CreateShadowVaultNameError::NameTooShort)
    } else if name.len() > Self::MAX_LENGTH {
      Err(CreateShadowVaultNameError::NameTooLong)
    } else {
      Ok(Self(name))
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateShadowVaultDatumError {
  DatumTooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowVaultDatum(String);

impl ShadowVaultDatum {
  // const MIN_LENGTH: usize = 0;
  const MAX_LENGTH: usize = 40;

  pub fn new(name: String) -> Result<ShadowVaultDatum, CreateShadowVaultDatumError> {
    if name.len() > Self::MAX_LENGTH {
      Err(CreateShadowVaultDatumError::DatumTooLong)
    } else {
      Ok(Self(name))
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}

// SECTION: ShadowVaultProtector.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ProtectorCreator {
//   ForDuration(Duration),
//   ByPassword(Password),
// }

// impl ProtectorCreator {
//   pub fn create(self, now: DateTime) -> Protector {
//     match self {
//       Self::ForDuration(duration) => {
//         Protector::ForDuration(CountdownTimer::new(duration, now))
//       }
//       Self::ByPassword(password) => {
//         Protector::ByPassword { password, is_protected: true }
//       }
//     }
//   }
// }

// #[derive(Debug, Clone)]
// pub enum Protector {
//   ForDuration(CountdownTimer),
//   ByPassword { is_protected: bool, password: Password },
// }

// impl Protector {
//   pub fn is_effective(&self) -> bool {
//     match self {
//       Protector::ForDuration(countdown_timer) => {
//         countdown_timer.is_running()
//       }
//       Protector::ByPassword { is_protected, .. } => {
//         *is_protected
//       }
//     }
//   }
// }

// SECTION: ShadowVault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowVaultCreator {
  pub(super) id: Option<Uuid>,
  name: ShadowVaultName,
  datum: ShadowVaultDatum,
  protector: Duration,
}

impl ShadowVaultCreator {
  pub fn create(self, now: DateTime) -> ShadowVault {
    ShadowVault {
      id: self.id.unwrap_or_else(Uuid::new_v4),
      name: self.name,
      datum: self.datum,
      protector: CountdownTimer::new(self.protector, now),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ShadowVault {
  pub(super) id: Uuid,
  pub(super) name: ShadowVaultName,
  pub(super) datum: ShadowVaultDatum,
  pub(super) protector: CountdownTimer
}

impl ShadowVault {
  pub fn is_protected(&mut self) -> bool {
    self.protector.is_running()
  }  
}

// SECTION: Feature.
#[derive(Debug, Clone)]
pub struct Feature {
  pub(super) shadow_vaults: Vec<ShadowVault>
}