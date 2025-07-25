use std::sync::{Arc, Mutex};
use super::*;

// pub enum ScheduledTask {
//   ScreenAccessRegulationApplicationCheck,
//   ScreenAccessRegulationApplicationAllow(OperatingSystemUserId),
//   ScreenAccessRegulationApplicationBlock(OperatingSystemUserId),
// }


pub enum AsyncOperation {
  ScreenAccessRegulationApplication(screen_access_regulation_application::ScreenAccessRegulationAsyncOperation)
}

impl AsyncOperation {
  pub fn execute(
    self, 
    operatin_system_integration_data: Arc<Mutex<Data>>,
    scheduler: Arc<OperationScheduler>
  ) {}
}
