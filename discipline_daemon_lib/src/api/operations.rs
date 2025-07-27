use crate::operating_system_integration;

static OPERATING_SYSTEM_INTEGRATION_MANAGE_USER: &'static str =
  "OperatingSystemIntegrationManageUser";

type OperatingSystemIntegrationManageUser = 
  operating_system_integration::api::ManageUser;

static OPERATING_SYSTEM_INTEGRATION_UNMANAGE_USER: &'static str = 
  "OperatingSystemIntegrationUnmanageUser";

type OperatingSystemIntegrationUnmanageUser = 
  operating_system_integration::api::UnmanageUser;

static OPERATING_SYSTEM_INTEGRATION_SCREEN_ACCESS_REGULUATION_APPLICATION_ENABLE: &'static str = 
  "OperatingSystemIntegrationScreenAccessRegulationApplicationEnable";

type OperatingSystemIntegrationRegulationApplicationEnable = 
  operating_system_integration::api::EnableScreenAccessRegulationApplication;

static OPERATING_SYSTEM_INTEGRATION_SCREEN_ACCESS_REGULATION_APPLICATION_DISABLE: &'static str = 
  "OperatingSystemIntegrationScreenAccessRegulationApplicationDisable";



#[macro_use]
#[macro_export]
macro_rules! find_operation_type {
  ($operation_id:expr, |$op_type:ident| $code:block else $else:block) => {
    match $operation_id {
      // id if id == OPERATING_SYSTEM_INTEGRATION_MANAGE_USER => {
      //   type $op_type = OperatingSystemIntegrationManageUser;
      //   $code
      // }
      // add the rest here
      _ => {
        $else
      },
    }
  };
}
