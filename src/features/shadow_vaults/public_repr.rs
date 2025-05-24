use serde::{Deserialize, Serialize};

use crate::{
  ToPublicRepr, CountdownTimer, Uuid
};

use super::{
  ShadowVault, ShadowVaultDatum, ShadowVaultName, Feature
};

// SECTION: Protector.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum ProtectorPublicRepr {
//   ForDuration(CountdownTimer),
//   ByPassword { is_protected: bool },
// }

// impl ToPublicRepr for Protector {
//   type PublicRepr = ProtectorPublicRepr;

//   fn to_public_repr(&mut self) -> Self::PublicRepr {
//     match self {
//       Protector::ForDuration(inner) => {
//         ProtectorPublicRepr::ForDuration(inner.clone())
//       } 
//       Protector::ByPassword { is_protected, .. } => {
//         ProtectorPublicRepr::ByPassword { is_protected: *is_protected }
//       }
//     }
//   }
// }

// SECTION: ShadowVault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowVaultPublicRepr {
  id: Uuid,
  name: ShadowVaultName,
  datum: Option<ShadowVaultDatum>,
  protector: CountdownTimer,
}

impl ToPublicRepr for ShadowVault {
  type PublicRepr = ShadowVaultPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    ShadowVaultPublicRepr {
      id: self.id.clone(),
      name: self.name.clone(),
      datum: if self.is_protected() {
        None
      } else {
        Some(self.datum.clone())
      },
      protector: self.protector.clone(),
    }
  }
}

// SECTION: Feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturePublicRepr {
  shadow_vaults: Vec<ShadowVaultPublicRepr>
}

impl ToPublicRepr for Feature {
  type PublicRepr = FeaturePublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    FeaturePublicRepr {
      shadow_vaults: self
        .shadow_vaults
        .iter_mut()
        .map(|shadow_vault| shadow_vault.to_public_repr())
        .collect(),
    }
  }
}