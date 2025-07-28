use crate::operating_system_integration;

pub struct OperationSpec {
  
}
pub static OPERATING_SYSTEM_INTEGRATION_MANAGE_USER: &'static str =
  "OperatingSystemIntegrationManageUser";

pub type OperatingSystemIntegrationManageUser = operating_system_integration::api::ManageUser;

pub static OPERATING_SYSTEM_INTEGRATION_UNMANAGE_USER: &'static str =
  "OperatingSystemIntegrationUnmanageUser";

pub type OperatingSystemIntegrationUnmanageUser = operating_system_integration::api::UnmanageUser;

pub static OPERATING_SYSTEM_INTEGRATION_SCREEN_ACCESS_REGULUATION_APPLICATION_ENABLE: &'static str =
  "OperatingSystemIntegrationScreenAccessRegulationApplicationEnable";

pub type OperatingSystemIntegrationScreenAccessRegulationApplicationEnable =
  operating_system_integration::api::EnableScreenAccessRegulationApplication;

pub static OPERATING_SYSTEM_INTEGRATION_SCREEN_ACCESS_REGULATION_APPLICATION_DISABLE: &'static str =
  "OperatingSystemIntegrationScreenAccessRegulationApplicationDisable";

pub type OperatingSystemIntegrationScreenAccessRegulationApplicationDisable =
  operating_system_integration::api::DisableScreenAccessRegulationApplication;

#[macro_use]
#[macro_export]
macro_rules! find_operation_type {
  ($operation_id:expr, |$operation_type:ident| $code:block else $else:block) => {
    match $operation_id {
      id if id == crate::api::operations::OPERATING_SYSTEM_INTEGRATION_MANAGE_USER => {
        type $operation_type = crate::api::operations::OperatingSystemIntegrationManageUser;
        $code
      }
      id if id == crate::api::operations::OPERATING_SYSTEM_INTEGRATION_UNMANAGE_USER  => {
        type $operation_type = crate::api::operations::OperatingSystemIntegrationUnmanageUser;
        $code
      }
      id if id == crate::api::operations::OPERATING_SYSTEM_INTEGRATION_SCREEN_ACCESS_REGULUATION_APPLICATION_ENABLE  => {
        type $operation_type = crate::api::operations::OperatingSystemIntegrationScreenAccessRegulationApplicationEnable;
        $code
      }
      id if id == crate::api::operations::OPERATING_SYSTEM_INTEGRATION_SCREEN_ACCESS_REGULATION_APPLICATION_DISABLE  => {
        type $operation_type = crate::api::operations::OperatingSystemIntegrationScreenAccessRegulationApplicationDisable;
        $code
      }
      _ => {
        $else
      },
    }
  };
}
