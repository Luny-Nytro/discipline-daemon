use std::{collections::HashSet, sync::{Arc, Mutex, MutexGuard}};
use crate::GenericError;

pub struct Daemon {
  blocked_hostnames: HashSet<String>
}

impl Daemon {
  fn new(blocked_hostnames: HashSet<String>) -> Self {
    Self { 
      blocked_hostnames
    }
  }

  pub fn is_hostname_in_block_list(&self, hostname: &String) -> bool {
    self.blocked_hostnames.contains(hostname)
  }
}

pub struct DaemonMutex {
  inner: Arc<Mutex<Daemon>>
}

impl DaemonMutex {
  pub fn clone(&self) -> DaemonMutex {
    Self {
      inner: Arc::clone(&self.inner)
    }
  }

  pub fn lock(&self) -> Result<MutexGuard<'_, Daemon>, GenericError> {
    self.inner.lock().map_err(|error|
      GenericError::new("locking daemon mutex")
        .add_attachment("error", error.to_string())
    )
  }
}
