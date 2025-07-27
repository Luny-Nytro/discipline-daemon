use rusqlite::Connection;
use crate::{database::deserialize, Duration, Uuid};
use super::{database_code, ShadowVault, ShadowVaultName, Feature};

pub fn create_shadow_vault(
  connection: &Connection, 
  shadow_vault: &ShadowVault,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::create_shadow_vault(&mut code, shadow_vault);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.Feature.DatabaseProcedures.CreateShadowVault: \nCode: {code}. \nError: {error}.");
  })
}

pub fn delete_shadow_vault(
  connection: &Connection, 
  shadow_vault_id: &Uuid,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::delete_shadow_vault(&mut code, shadow_vault_id);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.Feature.DatabaseProcedures.DeleteShadowVault: \nCode: {code}. \nError: {error}.");
  })
}

pub fn change_shadow_vault_name(
  connection: &Connection, 
  shadow_vault_id: &Uuid,
  new_name: &ShadowVaultName,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::change_shadow_vault_name(&mut code, shadow_vault_id, new_name);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.Feature.DatabaseProcedures.ChangeShadowVaultName: \nCode: {code}. \nError: {error}.");
  })
}

pub fn find_all_shadow_vaults(db: &Connection) -> Result<Vec<ShadowVault>, ()> {
  let mut rules = Vec::new();

  let mut code = String::new();
  database_code::find_all_shadow_vaults(&mut code);
  
  let mut statement = match db.prepare(&code) {
    Ok(value) => {
      value
    }
    Err(error) => {
      eprintln!("Discipline.Feature.DatabaseProcedures.FindAllEnforcers.PrepareStatement: \nCode: {code}. \nError: {error}.");
      return Err(());
    }
  };

  let mut iterator = match statement.query(()) {
    Ok(value) => {
      value
    }
    Err(error) => {
      eprintln!("Discipline.Feature.DatabaseProcedures.FindAllEnforcers.Query: \nCode: {code}. \nError: {error}.");
      return Err(())
    }
  };

  loop {
    let item = match iterator.next() {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.Feature.DatabaseProcedures.FindAllEnforcers.Query.Iterator.Next: \nCode: {code}. \nError: {error}.");
        return Err(());
      }
    };

    let Some(item) = item else {
      break;
    };

    let item = match deserialize(item) {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.Feature.DatabaseProcedures.FindAllEnforcers.Deserialize: \nCode: {code}. \nError: {:?}.", error);
        return Err(());
      }
    };

    rules.push(item);
  }

  Ok(rules)
}

pub fn load_shadow_vaults_feature(db: &Connection) -> Result<Feature, ()> {
  Ok(Feature {
    shadow_vaults: find_all_shadow_vaults(db)?
  })
}

pub fn enabler_for_duration_change_remaining_duration(
  connection: &Connection,
  shadow_vault_id: &Uuid,
  new_remaining_duration: &Duration,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::change_protector_remaining_duration(&mut code, shadow_vault_id, new_remaining_duration);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.Feature.DatabaseProcedures.Protector.ForDuration.ChangeRemainingDuration: \nCode: {code}. \nError: {error}.");
  })
}