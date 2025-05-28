use serde::{Deserialize, Serialize};
use crate::{Duration, ToPublicRepr};
use super::{CommonInfo, Policy, PolicyEnabler, Regulator, Rule, RuleActivator};

pub type RuleActivatorPublicRepr = RuleActivator;

impl ToPublicRepr for RuleActivator {
  type PublicRepr = RuleActivatorPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    self.clone()
  }
}

pub type RulePublicRepr = Rule;

impl ToPublicRepr for Rule {
  type PublicRepr = RulePublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    self.clone()
  }
}

pub type PolicyEnablerPublicRepr = PolicyEnabler;

impl ToPublicRepr for PolicyEnabler {
  type PublicRepr = PolicyEnablerPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    self.clone()
  }
}

pub type PolicyPublicRepr = Policy;

impl ToPublicRepr for PolicyPublicRepr {
  type PublicRepr = PolicyPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    self.clone()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatorPublicRepr {
  policies: Vec<PolicyPublicRepr>,
  is_applying_enabled: bool,
  is_user_screen_acceess_blocked: bool,
}

impl ToPublicRepr for Regulator {
  type PublicRepr = RegulatorPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    RegulatorPublicRepr {
      policies: self.policies.to_public_repr(),
      is_applying_enabled: self.is_applying_enabled,
      is_user_screen_acceess_blocked: self.is_user_screen_access_blocked,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonInfoPublicRepr {
  applying_interval: Duration,
}

impl ToPublicRepr for CommonInfo {
  type PublicRepr = CommonInfoPublicRepr;

  fn to_public_repr(&mut self) -> Self::PublicRepr {
    CommonInfoPublicRepr {
      applying_interval: self.applying_interval,
    }
  }
}