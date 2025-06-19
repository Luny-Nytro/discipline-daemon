use std::time::Duration;
use crate::utility::DynErrOr;
use windows_service::{
  service_control_handler::ServiceStatusHandle, 
  service::{
    ServiceStatus, 
    ServiceType, 
    ServiceState, 
    ServiceControlAccept, 
    ServiceExitCode,
  },
};

static mut SERVICE_STATUS_HANDLE: Option<ServiceStatusHandle> = None;

pub fn initialize(handle: ServiceStatusHandle) {
  unsafe {
    SERVICE_STATUS_HANDLE = Some(handle);
  }
}

/// Tells the operating system that the service has successfully completed 
/// initialization and is now running.
/// 
/// *SAFETY*: May only be called after `initialize` has been called.
pub fn signal_running() -> DynErrOr<()> {
  unsafe {
    SERVICE_STATUS_HANDLE.unwrap().set_service_status(create_running_status())?;
  }

  Ok(())
}

/// Tells the operating system that the service is stopped and is ready to exit.
///  
/// *SAFETY*: May only be called after `initialize` has been called.
pub fn signal_stopped() -> DynErrOr<()> {
  unsafe {
    SERVICE_STATUS_HANDLE.unwrap().set_service_status(create_stopped_status())?;
  }

  Ok(())
}


fn create_running_status() -> ServiceStatus {
  ServiceStatus {
    // Should match the one from system service registry
    service_type: ServiceType::OWN_PROCESS,
    // The new state
    current_state: ServiceState::Running,
    // Accept stop events when running
    controls_accepted: ServiceControlAccept::STOP,
    // Used to report an error when starting or stopping only, otherwise must be zero
    exit_code: ServiceExitCode::Win32(0),
    // Only used for pending states, otherwise must be zero
    checkpoint: 0,
    // Only used for pending states, otherwise must be zero
    wait_hint: Duration::default(),
    process_id: None,
  }
}

fn create_stopped_status() -> ServiceStatus {
  ServiceStatus {
    wait_hint: Duration::default(),
    exit_code: ServiceExitCode::Win32(0),
    checkpoint: 0,
    process_id: None,
    service_type: ServiceType::OWN_PROCESS,
    current_state: ServiceState::Stopped,
    controls_accepted: ServiceControlAccept::all(),
  }
}
