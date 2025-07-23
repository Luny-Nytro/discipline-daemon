mod data;
use data::*;

mod operation;
use operation::*;

mod functions;
use functions::*;

mod types;
pub use types::*;

mod user_screen_access;
use user_screen_access::*;

mod api;
use api::*;

use crate::*;
use std::collections::{BTreeMap, VecDeque};
use std::io::Write;
use std::marker::PhantomData;
use std::mem::take;
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

struct Mutexed {
  dropped: bool,
  operations: BTreeMap<Instant, Operation>,
}

pub struct OperationScheduler {
  worker_thread: Option<JoinHandle<()>>,
  condvar_and_mutex: Arc<(Condvar, Mutex<Mutexed>)>,
}

impl OperationScheduler {
  pub fn new() -> Self {
    let mut scheduler = OperationScheduler {
      worker_thread: None,
      condvar_and_mutex: Arc::new((
        Condvar::new(),
        Mutex::new(Mutexed { 
          dropped: false, 
          operations: BTreeMap::new(),
        }),
      )),
    };

    let condvar_and_mutex = Arc::clone(&scheduler.condvar_and_mutex);

    let handle = spawn(move || {
      loop {
        let operation = {
          let mutex_guard = condvar_and_mutex.1.lock().unwrap();
          if mutex_guard.dropped {
            break;
          }

          pick(&condvar_and_mutex.0, mutex_guard)
        };

        operation.execute();
      }
    });

    scheduler.worker_thread = Some(handle);
    scheduler
  }
}

fn pick(
  condvar: &Condvar,
  mut mutex_guard: MutexGuard<Mutexed>,
) -> Operation {
  let operation = loop {
    let now = Instant::now();

    match mutex_guard.operations.first_entry() {
      Some(operation) if operation.key().le(&now) => {
        break operation.remove();
      }

      Some(operation) => {
        let wait = operation.key().saturating_duration_since(now);
        mutex_guard = condvar.wait_timeout(mutex_guard, wait).unwrap().0;
      }

      None => {
        mutex_guard = condvar.wait(mutex_guard).unwrap();
      }
    }
  };

  operation
}

impl OperationScheduler {
  pub fn add_immediate_operation(&self, operation: Operation) {
    self
      .condvar_and_mutex.1
      .lock()
      .unwrap()
      .operations
      .insert(Instant::now(), operation);

    self.condvar_and_mutex.0.notify_one();
  }

  pub fn add_delayed_operation(&self, operation: Operation, delay: Duration) {
    let time = Instant::now().checked_add(delay).unwrap();
    
    self
      .condvar_and_mutex.1
      .lock()
      .unwrap()
      .operations
      .insert(time, operation);

    self
      .condvar_and_mutex.0
      .notify_one();
  }
}

//   fn execute_operation(shared_state: &Arc<(Mutex<SharedState>, Condvar)>, op: Operation) {
//     println!("Executing operation: {:?}", op);

//     // Here you would have your actual operation execution logic

//     // Example of scheduling more operations during execution
//     match op {
//       Operation::TaskA(s) if s == "schedule_more" => {
//         let scheduler = OperationScheduler {
//           shared_state: Arc::clone(shared_state),
//           worker_thread: None,
//         };

//         scheduler.add_operation(Operation::TaskB(42), Priority::Immediate);
//         scheduler.add_operation(
//           Operation::TaskA("followup".to_string()),
//           Priority::LowPriority,
//         );
//       }
//       _ => {}
//     }
//   }

//   fn stop(&mut self) {
//     let (lock, cvar) = &*self.shared_state;
//     let mut state = lock.lock().unwrap();
//     state.should_stop = true;
//     cvar.notify_one();

//     if let Some(thread) = self.worker_thread.take() {
//       thread.join().unwrap();
//     }
//   }
// }

// impl Drop for OperationScheduler {
//   fn drop(&mut self) {
//     self.stop();
//   }
// }

fn execute_operation(operation: Operation, x: &OperationScheduler) {}
