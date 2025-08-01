use crate::Uuid;
use crate::operating_system_integration::UserId;
use crate::logic::data_vaults::{ShadowVault};
use crate::chronic::{CountdownTimer, Duration, DateTime};
use super::*;

pub struct ShadowVaultSchema {
  id: String,
  protection_duration: String,
  protection_remaining_duration: String,
  protection_previous_synchronization_time: String,
}

impl ShadowVaultSchema {
  fn new() -> Self {
    Self {
      id: "Id".into(),
      protection_duration: "ProtectionDuration".into(),
      protection_remaining_duration: "ProtectionRemainingDuration".into(),
      protection_previous_synchronization_time: "ProtectionPreviousSynchronizationTime".into(),
    }
  }
}

fn serialize(
  context: &mut SerializeCompoundValueContext,
  schema: &ShadowVaultSchema,
  shadow_vault_id: &Uuid,
) {
  context.write_scalar(&schema.id, value.);
}

pub struct NormalizedShadowVault {
  id: Uuid,
  user_id: UserId,
  protection: CountdownTimer,
}

