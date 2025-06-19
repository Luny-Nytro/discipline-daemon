use std::ffi::OsString;
use super::os;
use windows_service::{service::ServiceControl, service_control_handler::ServiceControlHandlerResult};
use crate::{
  service_name::{
    initialize_service_name, 
    service_name,
  }, 
  body, 
  utility::{
    DynErrOr, 
    parse_service_arguments,
  }, 
  status_handle,
};

pub fn launch(service_name: &'static str, service_body: fn(Vec<OsString>)) -> DynErrOr<()> {
  initialize_service_name(service_name);  
  body::initialize(service_body);
  os::start_service_controller_dispatcher(service_name, service_main)?;
  Ok(())
}

extern "system" fn service_main(
  arguments_length: u32, 
  arguments: *mut *mut u16,
) {
  let arguments = unsafe {
    parse_service_arguments(arguments_length, arguments)
  };

  if let Err(_) = service_main_inner(arguments) {
    // TODO: handle error
  }
}


fn service_main_inner(arguments: Vec<OsString>) -> DynErrOr<()> {
  let handle = os::register_service_control_handler(service_name(), on_service_control)?;

  status_handle::initialize(handle);
  // TODO: Handle possible error.
  _ = status_handle::signal_running();
  body::run(arguments);
  // TODO: Handle possible error.
  _ = status_handle::signal_stopped();
  
  Ok(())
}

fn on_service_control(service_control: ServiceControl) -> ServiceControlHandlerResult {
  match service_control {
    // When we are told to stop,
    ServiceControl::Stop => {
      // we stop.
      // TODO: Handle the possible error gracefully.
      _ = status_handle::signal_stopped();

      ServiceControlHandlerResult::NoError
    }
    
    ServiceControl::Interrogate => {
      ServiceControlHandlerResult::NoError
    }

    _ => {
      ServiceControlHandlerResult::NotImplemented
    }
  }
}