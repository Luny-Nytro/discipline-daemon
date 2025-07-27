use std::sync::{Arc, Mutex};
use super::*;

// pub enum ScheduledTask {
//   ScreenAccessRegulationApplicationCheck,
//   ScreenAccessRegulationApplicationAllow(OperatingSystemUserId),
//   ScreenAccessRegulationApplicationBlock(OperatingSystemUserId),
// }


pub enum AsyncOperation {
  ScreenAccessRegulationApplication(screen_access_regulation_application::ScreenAccessRegulationAsyncOperation),
  InternetAccessRegulationApplication(internet_access_regulation_application::InternetAccessRegulationAsyncTask),
}

impl AsyncOperation {
  pub fn execute(
    self, 
    operatin_system_integration_data: Arc<Mutex<OperatingSystemIntegrationData>>,
    scheduler: Arc<OperationScheduler>
  ) {
    
  }
}
