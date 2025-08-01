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
  pub const MIN_LENGTH: usize = 1;
  pub const MAX_LENGTH: usize = 100;

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
  DatumTooShort,
  DatumTooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowVaultDatum(String);

impl ShadowVaultDatum {
  const MIN_LENGTH: usize = 1;
  const MAX_LENGTH: usize = 500;

  pub fn new(name: String) -> Result<ShadowVaultDatum, CreateShadowVaultDatumError> {
    if name.len() < Self::MIN_LENGTH {
      Err(CreateShadowVaultDatumError::DatumTooShort)
    } else if name.len() > Self::MAX_LENGTH {
      Err(CreateShadowVaultDatumError::DatumTooLong)
    } else {
      Ok(Self(name))
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.0
  }
}

pub struct ShadowVault {
  id: Uuid,
  protection: CountdownTimer,
}

impl ShadowVault {
  pub fn new(id: Uuid, protection_duration: Duration) -> Self {
    Self {
      id,
      protection: CountdownTimer::new_without_now(protection_duration),
    }
  }

  pub fn id(&self) -> &Uuid {
    &self.id
  }

  pub fn protection_duration(&self) -> Duration {
    self.protection.duration()
  }

  pub fn remaining_protection_duration(&self) -> Duration {
    self.protection.remaining_duration()
  }

  pub fn is_protected(&self, now: DateTime) -> bool {
    self.protection.is_running()
  }
}

pub struct ShadowVaultWithData {
  id: Uuid,
  data: Vec<ShadowVaultDatum>,
  protector: CountdownTimer,
}

impl ShadowVaultWithData {
  pub fn new(
    id: Uuid, 
    data: Vec<ShadowVaultDatum>,
    protection_duration: Duration,
  ) -> Self {
    Self {
      id,
      data,
      protector: CountdownTimer::new_without_now(protection_duration),
    }
  }

  pub fn is_protected(&self, now: DateTime) -> bool {
    self.protector.is_running()
  }
}