use serde::{Deserialize, Serialize};
use crate::{
  user_access, 
  DateTime, IsOperation, ToPublicRepr
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllDataOutcome {
  now: DateTime,
  user_access: user_access::FeaturePublicRepr,
  // shadow_vaults: shadow_vaults::FeaturePublicRepr,
  // networking_access: networking_access::FeaturePublicRepr,
}

impl IsOperation for GetData {
  type Outcome = GetAllDataOutcome;

  fn execute(self, app: &mut super::App) -> Self::Outcome {
    Self::Outcome {
      now: DateTime::now(),
      user_access: app.state.user_access.to_public_repr(),
      // shadow_vaults: app.state.shadow_vaults.to_public_repr(),
      // networking_access: app.state.networking_access.to_public_repr(),
    }
  }
}