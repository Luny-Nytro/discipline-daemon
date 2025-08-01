mod screen_access_regulation;
mod internet_access_regulation;
mod operating_system_integration_linux;

pub mod operations {
  pub use super::screen_access_regulation::{
    CreatePolicy as ScreenAccessRegulationCreatePolicy,
    CreateRule as ScreenAccessRegulationCreateRule,
    DeletePolicy as ScreenAccessRegulationDeletePolicy,
    DeleteRule as ScreenAccessRegulationDeleteRule,
    IncreasePolicyProtection as ScreenAccessRegulationIncreasePolicyProtection,
    UpdatePolicyName as ScreenAccessRegulationUpdatePolicyName,
    UpdateRuleActivatorTimeRange as ScreenAccessRegulationUpdateRuleActivatorTimeRange,
    UpdateRuleActivatorWeekdayRange as ScreenAccessRegulationUpdateRuleActivatorWeekdayRange,
  };

  pub use super::internet_access_regulation::{
    CreatePolicy as InternetAccessRegulationCreatePolicy,
    CreateRule as InternetAccessRegulationCreateRule,
    DeletePolicy as InternetAccessRegulationDeletePolicy,
    DeleteRule as InternetAccessRegulationDeleteRule,
    IncreasePolicyProtection as InternetAccessRegulationIncreasePolicyProtection,
    UpdatePolicyName as InternetAccessRegulationUpdatePolicyName,
    UpdateRuleActivatorTimeRange as InternetAccessRegulationUpdateRuleActivatorTimeRange,
    UpdateRuleActivatorWeekdayRange as InternetAccessRegulationUpdateRuleActivatorWeekdayRange,
  };

  pub use super
    ::operating_system_integration_linux
    ::screen_access_regulation
    ::
  {
    EnableApplication as OperatingSystemIntegrationScreenAccessRegulationEnableApplication,
    DisableApplication as OperatingSystemIntegrationScreenAccessRegulationDisableApplication,
  };

  pub use super
    ::operating_system_integration_linux
    ::internet_access_regulation
    ::
  {
    EnableApplication as OperatingSystemIntegrationInternetAccessRegulationEnableApplication,
    DisableApplication as OperatingSystemIntegrationInternetAccessRegulationDisableApplication,
  };

  pub use super
    ::operating_system_integration_linux
    ::top
    ::
  {
    ManageUser as OperatingSystemIntegrationManageUser,
    UnmanageUser as OperatingSystemIntegrationUnmanageUser,
  };
}