use windows::Win32::System::Services::{
  SERVICE_DISABLED,
  SERVICE_AUTO_START,
  SERVICE_START_TYPE,
  SERVICE_DEMAND_START,
};

pub enum StartMode {
  /// A service started by the service control manager when a process calls the StartService 
  /// function. For more information, see Starting Services on Demand.
  Manual,
  /// A service that cannot be started. Attempts to start the service result in the error 
  /// code ERROR_SERVICE_DISABLED.
  Disabled,  
  /// A service started automatically by the service control manager during system startup. 
  /// For more information, see Automatically Starting Services.
  Automatic,
  
  // AutomaticDelayed,

  // /// A device driver started by the IoInitSystem function. This value is valid 
  // /// only for driver services.
  // SystemStart,

  // /// A device driver started by the system loader. This value is valid only for 
  // /// driver services.
  // BootStart,
}

impl StartMode {
  pub fn intoParam(&self) -> SERVICE_START_TYPE {
    match self {
      StartMode::Manual => SERVICE_DEMAND_START,
      StartMode::Disabled => SERVICE_DISABLED,
      StartMode::Automatic => SERVICE_AUTO_START,
    }
  }
}