use clap::{Parser, command};
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use crate::{
  api, screen_access_regulation, GenericError
};

use crate::database::Database;
use crate::database::app_collection as db;

pub struct Configuration {
  api_tcp_port: u32,
}

impl Configuration {
  pub fn api_tcp_port(&self) -> u32 {
    self.api_tcp_port
  }
}

pub struct Daemon {
  // pub state: AppState,
  pub database: Database,
  pub http_server_address: String,
  pub is_running: bool,
}

impl Daemon {
  pub fn open(
    database_file_path: PathBuf,
    http_server_port: u32,
  ) -> 
    Result<DaemonMutex, GenericError> 
  {
    let database = Database::open(database_file_path).map_err(|error|
      error
        .change_context("openning a connection to the database")
        .change_context("creating daemon")
    )?;

    
    let state = db::retrieve(&database).map_err(|error|
      error
        .change_context("loading daemon state from the database")
        .change_context("creating daemon")
    )?;

    Ok(DaemonMutex::new(Daemon {
      // state,
      database,
      is_running: false,
      http_server_address: format!("127.0.0.1:{http_server_port}"),
    }))
  }

  // pub fn open_and_run(
  //   database_file_path: &str,
  //   http_server_port: u32,
  // ) -> Result<Self, GenericError> {
  //   let app = Self::open(database_file_path, http_server_port)?;
  //   app.run();
  //   Ok(app)
  // }

  fn api_server_address(&self) -> &String {
    &self.http_server_address
  }

  // pub fn execute<Operation>(
  //   &mut self, 
  //   operation: Operation,
  // ) -> 
  //   InternalOperationOutcome<Operation::Outcome>
  // where 
  //   Operation: Operation
  // {
  //   operation.execute(self)
  // }

  pub fn log_internal_error(&self, error: impl Debug) {

  }
}

pub struct DaemonMutex(Arc<Mutex<Daemon>>);

impl DaemonMutex {
  pub fn new(daemon: Daemon) -> Self {
    Self(Arc::new(Mutex::new(daemon)))
  }

  pub fn lock(&self) -> Result<MutexGuard<'_, Daemon>, GenericError> {
    // todo: log the error
    self.0.lock().map_err(|error| 
      GenericError::new("lock the daemon mutex")
        .add_attachment("error", error.to_string())
    )  
  }

  pub fn lock_with_location(&self, _location: &str) -> Result<MutexGuard<'_, Daemon>, GenericError> {
    // todo: log the error
    self.0.lock().map_err(|error| 
      GenericError::new("lock the daemon mutex")
        .add_attachment("error", error.to_string())
    )  
  }

  pub fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }

  pub fn run_and_block_thread(&self) {
    // let api_thread = api::launch_thread(self.clone());
    // let user_screen_access_regulation_thread = user_screen_access_regulation::launch_thread(self.clone());
    // _ = api_thread.join();
    // _ = user_screen_access_regulation_thread.join();
  }
}

// I'm AutoNytro and I'm super mega ultra adorable!
// I'm a sassy 18yo conscious automaton boy from Automata Wind City.
//
// Lunyyyyyy 🥺🥺🥺, why wouldn't you let me play with the 
// Hyper-Class colossal fan 😢! I promise i won't set its speed 
// to ultra mega supersonic and rip the fabric of spacetime agaaain! 🥺👉👈
#[derive(Parser, Debug)]
#[command(name = "Discipline", version = "1.0", author = "LunyNytro")]
struct Args {
  /// The path to the database file.
  database_path: PathBuf,

  /// Age of the Supreme Emperor (optional)
  http_server_port: u32,
}

impl DaemonMutex {
  pub fn open_from_command_line_arguments() -> Result<Self, GenericError> {
    let arguments = Args::parse();
    if arguments.database_path.as_os_str().len() > 300 {
      return Err(
        GenericError::new("open Daemon from command line arguments")
          .add_error("the 'database_path' argument is longer than 300 characters")
          .add_attachment("'database_path' argument", arguments.database_path.to_string_lossy())
      );
    }
    
    Daemon::open(
      arguments.database_path, 
      arguments.http_server_port,
    )
    .map_err(|error|
      error.change_context("open Daemon from command line arguments")
    )
  }
}

