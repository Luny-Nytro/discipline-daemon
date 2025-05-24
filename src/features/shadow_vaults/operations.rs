use serde::{Serialize, Deserialize};

use crate::{
  App, Duration, IsOperation, ToPublicRepr, DateTime, Uuid,
};

use super::{
  database_procedures, ShadowVaultCreator, ShadowVaultName, 
  ShadowVaultPublicRepr, 
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShadowVault {
  shadow_vault_creator: ShadowVaultCreator
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CreateShadowVaultError {
  ShadowVaultCreationLimitReached,
  IdUnavailable,
  InternalError,
}

impl IsOperation for CreateShadowVault {
  type Outcome = Result<ShadowVaultPublicRepr, CreateShadowVaultError>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let shadow_vaults = &mut app.app_data.shadow_vaults;

    if shadow_vaults.shadow_vaults.len() >= 30 {
      return Err(CreateShadowVaultError::ShadowVaultCreationLimitReached);
    }

    if let Some(rule_id) = self.shadow_vault_creator.id {
      if shadow_vaults.shadow_vaults.iter().any(|rule| rule.id == rule_id) {
        return Err(CreateShadowVaultError::IdUnavailable);
      }
    }

    let now = DateTime::now();
    let mut new_shadow_vault = self.shadow_vault_creator.create(now);
    if let Err(_) = database_procedures::create_shadow_vault(
      &app.database_connection, 
      &new_shadow_vault
    ) {
      return Err(CreateShadowVaultError::InternalError);
    }

    let shadow_vault_public_repr = new_shadow_vault.to_public_repr();
    shadow_vaults.shadow_vaults.push(new_shadow_vault);
    Ok(shadow_vault_public_repr)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteShadowVault {
  shadow_vault_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeleteShadowVaultError {
  NoSuchShadowVault,
  InternalError,
}

impl IsOperation for DeleteShadowVault {
  type Outcome = Result<(), DeleteShadowVaultError> ;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let shadow_vaults = &mut app.app_data.shadow_vaults;

    let Some(position) = shadow_vaults.shadow_vaults.iter().position(|shadow_vault| shadow_vault.id == self.shadow_vault_id) else {
      return Err(DeleteShadowVaultError::NoSuchShadowVault);
    };

    if let Err(_) = database_procedures::delete_shadow_vault(
      &app.database_connection, 
      &self.shadow_vault_id,
    ) {
      return Err(DeleteShadowVaultError::InternalError);
    }

    shadow_vaults.shadow_vaults.remove(position);
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeShadowVaultName {
  shadow_vault_id: Uuid,
  new_name: ShadowVaultName,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeShadowVaultNameError {
  NoSuchShadowVault,
  InternalError,
}

impl IsOperation for ChangeShadowVaultName {
  type Outcome = Result<(), ChangeShadowVaultNameError>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let shadow_vaults = &mut app.app_data.shadow_vaults;

    let Some(shadow_vault) = shadow_vaults
      .shadow_vaults
      .iter_mut()
      .find(|shadow_vault| shadow_vault.id == self.shadow_vault_id) else 
    {
      return Err(ChangeShadowVaultNameError::NoSuchShadowVault);
    };

    if let Err(_) = database_procedures::change_shadow_vault_name(
      &app.database_connection, 
      &self.shadow_vault_id, 
      &self.new_name,
    ) {
      return Err(ChangeShadowVaultNameError::InternalError);
    }

    shadow_vault.name = self.new_name;
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProtectorForDurationIncrementError {
  NoSuchShadowVault,
  WouldBeEffectiveForTooLong,
  // WrongType,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectorForDurationIncrement {
  shadow_vault_id: Uuid,
  increment: Duration,
}

impl IsOperation for ProtectorForDurationIncrement {
  type Outcome = Result<(), ProtectorForDurationIncrementError>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.app_data.shadow_vaults;

    let Some(shadow_vault) = feature
      .shadow_vaults
      .iter_mut()
      .find(|shadow_vault| shadow_vault.id == self.shadow_vault_id) else 
    {
      return Err(ProtectorForDurationIncrementError::NoSuchShadowVault);
    };

    // let Protector::ForDuration(countdown_timer) = &mut shadow_vault
    //   .protector else 
    // {
    //   return Err(ProtectorForDurationIncrementError::WrongType);
    // };
    
    let Some(new_remaining_duration) = shadow_vault.protector
      .remaining_duration()
      .checked_add(&self.increment) else
    {
      return Err(ProtectorForDurationIncrementError::WouldBeEffectiveForTooLong);
    };

    if new_remaining_duration.total_weeks() > 3 {
      return Err(ProtectorForDurationIncrementError::WouldBeEffectiveForTooLong);
    }

    if let Err(_) = database_procedures::enabler_for_duration_change_remaining_duration(
      &app.database_connection, 
      &self.shadow_vault_id, 
      &new_remaining_duration
    ) {
      return Err(ProtectorForDurationIncrementError::InternalError);
    }
    
    shadow_vault.protector.change_remaining_duration(new_remaining_duration);
    Ok(())
  }
}