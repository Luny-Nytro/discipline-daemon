
use crate::operating_system_integration;

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
