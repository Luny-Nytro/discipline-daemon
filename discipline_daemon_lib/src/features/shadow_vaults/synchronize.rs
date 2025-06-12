use std::fmt::Write;
use crate::{database::ScalarTypeSerde, DateTime, CountdownTimer};
use super::{ShadowVault, Feature};

pub fn synchronize_protector(
  into: &mut impl Write, 
  now: DateTime,
  protector: &mut CountdownTimer, 
  serialized_rule_id: &String,
) {
  protector.on_time_synchronize(now);

  let remaining_duration = protector.remaining_duration().serialize();
  let previous_synchronization_time = protector.previous_synchronization_time().serialize();
  writeln!(
    into, 
    "UPDATE ShadowVaults
      SET 
        ProtectorRemainingDuration = {remaining_duration}, 
        ProtectorPreviousSynchronizationTime = {previous_synchronization_time} 
      WHERE 
        Id = {serialized_rule_id};
    "
  ).unwrap();
}

pub fn synchronize_shadow_vault(
  into: &mut impl Write,
  now: DateTime,
  shadow_vault: &mut ShadowVault,
) {
  let serialized_rule_id = shadow_vault.id.serialize();
  synchronize_protector(into, now, &mut shadow_vault.protector, &serialized_rule_id);
}

pub fn synchronize_shadow_vaults_feature(
  into: &mut impl Write,
  now: DateTime,
  shadow_vaults_feature: &mut Feature,
) {
  for shadow_vault in &mut shadow_vaults_feature.shadow_vaults {
    synchronize_shadow_vault(into, now, shadow_vault);
  }
}
