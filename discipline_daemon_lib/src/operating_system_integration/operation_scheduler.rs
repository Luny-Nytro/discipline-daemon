use std::collections::BTreeMap;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{spawn, JoinHandle};
use std::time::{Duration, Instant};
use super::*;

struct OperationSchedulerCrossThreadMutexedData {
  dropped: bool,
  operations: BTreeMap<Instant, ScheduledTask>,
  worker_thread: Option<JoinHandle<()>>,
}

pub struct OperationScheduler {
  data: Mutex<OperationSchedulerCrossThreadMutexedData>,
  condvar: Condvar, 
}

impl OperationScheduler {
  pub fn new(
    operating_system_integration_data: Arc<Mutex<IntegrationData>>,
  ) -> Arc<Self> {
    let scheduler = Arc::new(OperationScheduler {
      condvar: Condvar::new(),
      data: Mutex::new(OperationSchedulerCrossThreadMutexedData { 
        dropped: false, 
        worker_thread: None,
        operations: BTreeMap::new(),
      })
    });

    let scheduler_clone = Arc::clone(&scheduler);

    let handle = spawn(move || {
      loop {
        let operation = {
          let data = scheduler.data.lock().unwrap();
          if data.dropped {
            break;
          }

          pick(&scheduler.condvar, data)
        };

        operation.execute(
          Arc::clone(&operating_system_integration_data),
          Arc::clone(&scheduler)
        );
      }
    });

    scheduler_clone.data.lock().unwrap().worker_thread = Some(handle);
    scheduler_clone
  }
}

fn pick(
  condvar: &Condvar,
  mut data: MutexGuard<OperationSchedulerCrossThreadMutexedData>,
) -> ScheduledTask {
  let operation = loop {
    let now = Instant::now();

    match data.operations.first_entry() {
      Some(operation) if operation.key().le(&now) => {
        break operation.remove();
      }

      Some(operation) => {
        let wait = operation.key().saturating_duration_since(now);
        data = condvar.wait_timeout(data, wait).unwrap().0;
      }

      None => {
        data = condvar.wait(data).unwrap();
      }
    }
  };

  operation
}

impl OperationScheduler {
  pub fn add_immediate_operation(&self, operation: impl Into<ScheduledTask>) {
    self.data.lock().unwrap().operations.insert(Instant::now(), operation.into());
    self.condvar.notify_one();
  }

  pub fn add_delayed_operation(&self, operation: impl Into<ScheduledTask>, delay: Duration) {
    let time = Instant::now().checked_add(delay).unwrap();
    self.data.lock().unwrap().operations.insert(time, operation.into());
    self.condvar.notify_one();
  }

  fn stop(&self) {
    let mut data = self.data.lock().unwrap();
    data.dropped = true;
    self.condvar.notify_one();
    data.worker_thread.take().map(|thread| thread.join().unwrap());
  }
}

impl Drop for OperationScheduler {
  fn drop(&mut self) {
    self.stop();
  }
}