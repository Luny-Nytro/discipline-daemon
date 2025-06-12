use std::fmt::Write;
use crate::{database::{ScalarTypeSerde, Serialize}, Duration, Uuid};
use super::{ShadowVault, ShadowVaultName};

pub fn initialize(into: &mut impl Write) {
  writeln!(into, "
    CREATE TABLE IF NOT EXISTS ShadowVaults (
      Id TEXT NOT NULL PRIMARY KEY,
      Name TEXT NOT NULL,
      Datum TEXT NOT NULL,
      ProtectorDuration INTEGER NOT NULL,
      ProtectorRemainingDuration INTEGER NOT NULL,
      ProtectorPreviousSynchronizationTime INTEGER NOT NULL
    );
  ").unwrap();
}

pub fn create_shadow_vault(
  into: &mut impl Write, 
  shadow_vault: &ShadowVault, 
) {
  writeln!(
    into, 
    "INSERT INTO ShadowVaults VALUES ({});", 
    shadow_vault.serialize(),
  ).unwrap();
}

pub fn delete_shadow_vault(
  into: &mut impl Write, 
  shadow_vault_id: &Uuid,
) {
  writeln!(
    into, 
    "DELETE FROM ShadowVaults WHERE Id = {};", 
    shadow_vault_id.serialize(),
  ).unwrap();
}

pub fn change_shadow_vault_name(
  into: &mut impl Write, 
  shadow_vault_id: &Uuid,
  new_name: &ShadowVaultName,
) {
  let new_name = new_name.serialize();
  let shadow_vault_id = shadow_vault_id.serialize();

  writeln!(into, "UPDATE ShadowVaults SET Name = {new_name} WHERE Id = {shadow_vault_id};").unwrap();
}

pub fn find_all_shadow_vaults(into: &mut impl Write) {
  writeln!(into, "SELECT * FROM ShadowVaults;").unwrap();
}

pub fn change_protector_remaining_duration(
  into: &mut impl Write,
  shadow_vault_id: &Uuid,
  new_remaining_duration: &Duration,
) {
  let shadow_vault_id = shadow_vault_id.serialize();
  let new_remaining_duration = new_remaining_duration.serialize();
  writeln!(into, "UPDATE ShadowVaults SET ProtectorRemainingDuration = {new_remaining_duration} WHERE Id = {shadow_vault_id};").unwrap();
}