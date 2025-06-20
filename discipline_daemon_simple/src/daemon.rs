use std::{collections::HashSet, net::{Ipv4Addr, SocketAddrV4}, path::Path, sync::{Arc, Mutex, MutexGuard}, thread::spawn};
use crate::{api_server, database::Database, proxy_server, GenericError};

pub struct Daemon {
  database: Database,
  blocklist: HashSet<String>,
  api_server_address: SocketAddrV4,
  proxy_server_address: SocketAddrV4,
}

impl Daemon {
  pub fn open(
    database_path: impl AsRef<Path>,
    api_server_port: u16,
    proxy_server_port: u16,
  ) ->
    Result<DaemonMutex, GenericError>
  {
    let database = Database::open(database_path).map_err(|error|
      error.change_context("openning daemon")
    )?;

    let blocklist = database.retrieve_blocklist().map_err(|error| 
      error.change_context("openning daemon")
    )?;

    Ok(DaemonMutex(Arc::new(Mutex::new(Daemon {
      database,
      blocklist,
      api_server_address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, api_server_port),
      proxy_server_address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, proxy_server_port),
    }))))
  }

  pub fn api_server_address(&self) -> SocketAddrV4 {
    self.api_server_address
  }
  
  pub fn proxy_server_address(&self) -> SocketAddrV4 {
    self.proxy_server_address
  }

  pub fn is_hostname_in_block_list(&self, hostname: &String) -> bool {
    self.blocklist.contains(hostname)
  }
}

pub struct DaemonMutex(Arc<Mutex<Daemon>>);

impl DaemonMutex {
  pub fn clone(&self) -> DaemonMutex {
    Self(Arc::clone(&self.0))
  }

  pub fn lock(&self) -> Result<MutexGuard<'_, Daemon>, GenericError> {
    self.0.lock().map_err(|error|
      GenericError::new("locking daemon mutex")
        .add_attachment("error", error.to_string())
    )
  }

  pub fn add_blocklist_item(&self, domain: String) -> Result<(), AddBlocklistItemError> {
    let mut daemon = self.lock().map_err(|error| 
      AddBlocklistItemError::InternalError(
        error.change_context("adding a new domain to the blocklist")
      )
    )?;

    daemon.database.add_domain_to_blocklist(&domain).map_err(|error|
      AddBlocklistItemError::InternalError(
        error.change_context("adding a new domain to the blocklist")
      )
    )?;

    daemon.blocklist.insert(domain);
    Ok(())
  }
}

pub enum AddBlocklistItemError {
  InternalError(GenericError),
  TooManyDomains,
  DomainIsTooLong,
}

impl DaemonMutex {
  pub fn run(&self) {
    let moon = self.clone();
    let thread1 = spawn(move || {
      api_server::run(moon);
    });

    let moon = self.clone();
    let thread2 = spawn(move || {
      proxy_server::run(moon);
    });

    _ = thread1.join();
    _ = thread2.join();
  }
}