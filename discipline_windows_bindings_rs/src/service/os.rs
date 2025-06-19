use windows_service::{
  service_control_handler::{
    ServiceStatusHandle, 
    ServiceControlHandlerResult,
  }, 
  service::ServiceControl,
};
use crate::utility::DynErrOr;

pub fn start_service_controller_dispatcher(
  service_name: &str, 
  service_main: extern "system" fn(u32, *mut *mut u16),
) -> DynErrOr<()> {
  windows_service
    ::service_dispatcher
    ::start(service_name, service_main)?;

  Ok(())
}

pub fn register_service_control_handler<T>(
  service_name: &str,
  service_control_handler: T
) -> DynErrOr<ServiceStatusHandle>
where
  T: FnMut(ServiceControl) -> ServiceControlHandlerResult + 'static + Send
{
  Ok(
    windows_service
      ::service_control_handler
      ::register(service_name, service_control_handler)?
  )
}
