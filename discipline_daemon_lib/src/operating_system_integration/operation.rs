use std::sync::{Arc, Mutex};
use super::*;

// pub enum ScheduledTask {
//   ScreenAccessRegulationApplicationCheck,
//   ScreenAccessRegulationApplicationAllow(OperatingSystemUserId),
//   ScreenAccessRegulationApplicationBlock(OperatingSystemUserId),
// }


pub enum ScheduledTask {
  ScreenAccessRegulationApplication(ScreenAccessRegulationScheduledTask)
}

impl ScheduledTask {
  pub fn execute(
    self, 
    operatin_system_integration_data: Arc<Mutex<IntegrationData>>,
    scheduler: Arc<OperationScheduler>
  ) {}
}
