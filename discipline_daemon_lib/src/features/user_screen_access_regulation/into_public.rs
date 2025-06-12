use serde::{Deserialize, Serialize};
use crate::{Duration, IntoPublic};
use super::{CommonInfo, Policy, PolicyEnabler, Regulator, Rule, RuleActivator};

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

pub type PolicyEnablerPublicRepr = PolicyEnabler;

impl IntoPublic for PolicyEnabler {
  type Output = PolicyEnablerPublicRepr;

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
pub struct RegulatorPublicRepr {
  policies: Vec<PolicyPublicRepr>,
  is_applying_enabled: bool,
  is_user_screen_acceess_blocked: bool,
}

impl IntoPublic for Regulator {
  type Output = RegulatorPublicRepr;

  fn into_public(self) -> Self::Output {
    RegulatorPublicRepr {
      policies: self.policies.into_public(),
      is_applying_enabled: self.is_applying_enabled,
      is_user_screen_acceess_blocked: self.is_user_screen_access_blocked,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonInfoPublicRepr {
  applying_interval: Duration,
}

impl IntoPublic for CommonInfo {
  type Output = CommonInfoPublicRepr;

  fn into_public(self) -> Self::Output {
    CommonInfoPublicRepr {
      applying_interval: self.applying_interval,
    }
  }
}