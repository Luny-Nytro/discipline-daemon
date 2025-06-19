use windows::Win32::System::Services::{
  SERVICE_ERROR,
  SERVICE_ERROR_NORMAL,
  SERVICE_ERROR_IGNORE,
  SERVICE_ERROR_SEVERE,
  SERVICE_ERROR_CRITICAL,
};

pub enum ErrorCtrl {
  /// The startup program ignores the error and continues the startup operation.
  Ignore,
  /// The startup program logs the error in the event log but continues the startup 
  /// operation.
  Normal,
  /// The startup program logs the error in the event log. If the last-known-good 
  /// configuration is being started, the startup operation continues. Otherwise, 
  /// the system is restarted with the last-known-good configuration.
  Severe,
  /// The startup program logs the error in the event log, if possible. If the 
  /// last-known-good configuration is being started, the startup operation fails. 
  /// Otherwise, the system is restarted with the last-known good configuration.
  Critical,
}

impl ErrorCtrl {
  pub fn intoParam(&self) -> SERVICE_ERROR {
    match self {
      ErrorCtrl::Ignore => SERVICE_ERROR_IGNORE,
      ErrorCtrl::Severe => SERVICE_ERROR_SEVERE,
      ErrorCtrl::Normal => SERVICE_ERROR_NORMAL,
      ErrorCtrl::Critical => SERVICE_ERROR_CRITICAL,
    }
  }
}