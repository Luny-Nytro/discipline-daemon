use serde::{Deserialize, Serialize};
use crate::IntoPublic;
use super::{Policy, Regulation, Rule, RuleActivator};

pub type RuleActivatorPublicRepr = RuleActivator;

impl IntoPublic for RuleActivator {
  type Output = RuleActivatorPublicRepr;

  fn into_public(self) -> Self::Output {
    self.clone()
  }
}

pub type RulePublicRepr = Rule;

impl IntoPublic for Rule {
  type Output = RulePublicRepr;

  fn into_public(self) -> Self::Output {
    self.clone()
  }
}

pub type PolicyPublicRepr = Policy;

impl IntoPublic for PolicyPublicRepr {
  type Output = PolicyPublicRepr;

  fn into_public(self) -> Self::Output {
    self.clone()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationPublicRepr {
  policies: Vec<PolicyPublicRepr>,
}

impl IntoPublic for Regulation {
  type Output = RegulationPublicRepr;

  fn into_public(self) -> Self::Output {
    RegulationPublicRepr {
      policies: self.policies.into_public(),
    }
  }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct CommonInfoPublicRepr {
//   applying_interval: Duration,
// }

// impl IntoPublic for CommonInfo {
//   type Output = CommonInfoPublicRepr;

//   fn into_public(self) -> Self::Output {
//     CommonInfoPublicRepr {
//       applying_interval: self.applying_interval,
//     }
//   }
// }