use std::collections::BTreeMap;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{spawn, JoinHandle};
use std::time::Instant;
use crate::chronic::Duration;
use crate::Daemon;

pub enum TopAsyncTask {
  ScreenAccessRegulation(
    super
      ::integration
      ::screen_access_regulation
      ::AsyncTask,
  ),
  InternetAccessRegulation(
    super
      ::integration
      ::internet_access_regulation
      ::AsyncTask,
  ),
}

impl TopAsyncTask {
  pub fn execute(self, daemon: Arc<Daemon>) {
    match self {
      TopAsyncTask::InternetAccessRegulation(task) => {
        task.execute(daemon);
      }
      TopAsyncTask::ScreenAccessRegulation(task) => {
        task.execute(daemon);
      }
    }
  }
}

struct OperationSchedulerCrossThreadMutexedData {
  dropped: bool,
  operations: BTreeMap<Instant, TopAsyncTask>,
  worker_thread: Option<JoinHandle<()>>,
}

pub struct AsyncScheduler {
  data: Mutex<OperationSchedulerCrossThreadMutexedData>,
  condvar: Condvar,
}

impl AsyncScheduler {
  pub fn new() -> Arc<Self> {
    Arc::new(AsyncScheduler {
      condvar: Condvar::new(),
      data: Mutex::new(OperationSchedulerCrossThreadMutexedData {
        dropped: false,
        worker_thread: None,
        operations: BTreeMap::new(),
      }),
    })
  }

  pub fn run_if_idle(self: Arc<Self>, daemon: Arc<Daemon>) {
    let scheduler = Arc::clone(&self);

    let handle = spawn(move || loop {
      let operation = {
        let data = scheduler.data.lock().unwrap();
        if data.dropped {
          break;
        }

        pick(&scheduler.condvar, data)
      };

      operation.execute(Arc::clone(&daemon));
    });

    self.data.lock().unwrap().worker_thread = Some(handle);
  }
}

fn pick(
  condvar: &Condvar,
  mut data: MutexGuard<OperationSchedulerCrossThreadMutexedData>,
) -> TopAsyncTask {
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

impl AsyncScheduler {
  pub fn add_immediate_operation(&self, operation: impl Into<TopAsyncTask>) {
    self
      .data
      .lock()
      .unwrap()
      .operations
      .insert(Instant::now(), operation.into());
    self.condvar.notify_one();
  }

  pub fn add_delayed_operation(&self, operation: impl Into<TopAsyncTask>, delay: Duration) {
    let time = Instant::now().checked_add(delay.as_standard_duration()).unwrap();
    self
      .data
      .lock()
      .unwrap()
      .operations
      .insert(time, operation.into());
    self.condvar.notify_one();
  }

  fn stop(&self) {
    let mut data = self.data.lock().unwrap();
    data.dropped = true;
    self.condvar.notify_one();
    data
      .worker_thread
      .take()
      .map(|thread| thread.join().unwrap());
  }
}

impl Drop for AsyncScheduler {
  fn drop(&mut self) {
    self.stop();
  }
}
