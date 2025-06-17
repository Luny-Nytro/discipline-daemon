use clap::{Parser, command};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{sleep, spawn};
use crate::{
  GenericError, IsOperation, State, 
  Specification, DateTime, Duration,
  InternalOperationOutcome,
};
use crate::database::Database;

pub struct Daemon {
  pub state: State,
  pub state_database_specification: Specification,
  pub database_connection: Database,
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
    let mut database = Database::open(database_file_path).map_err(|error|
      error
        .change_context("openning a connection to the database")
        .change_context("creating daemon")
    )?;

    let state_database_specification = Specification::new(database.define_namespace()).map_err(|error|
      error
        .change_context("creating state database specification")
        .change_context("creating daemon")
    )?;

    let state = state_database_specification.load(&database).map_err(|error|
      error
        .change_context("loading daemon state from the database")
        .change_context("creating daemon")
    )?;

    Ok(DaemonMutex::new(Daemon {
      state,
      state_database_specification,
      is_running: false,
      database_connection: database,
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

  pub fn execute<Operation>(
    &mut self, 
    operation: Operation,
  ) -> 
    InternalOperationOutcome<Operation::Outcome>
  where 
    Operation: IsOperation
  {
    operation.execute(self)
  }
}

pub struct DaemonMutex(Arc<Mutex<Daemon>>);

impl DaemonMutex {
  pub fn new(daemon: Daemon) -> Self {
    Self(Arc::new(Mutex::new(daemon)))
  }

  pub fn lock(&self) -> Result<MutexGuard<'_, Daemon>, GenericError> {
    self.0.lock().map_err(|error| 
      GenericError::new("lock the daemon mutex")
        .add_attachment("error", error.to_string())
    )  
  }

  pub fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

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

pub struct ServerThread {
  daemon: DaemonMutex,
}

impl ServerThread {
  pub fn spawn(daemon: DaemonMutex) {
    spawn(move || {
      // http_server::run(http_server_app);
    })
    .join();
  }
}

pub struct SynchronizationThread {
  daemon: DaemonMutex,
}

impl SynchronizationThread {
  pub fn spawn(daemon: DaemonMutex) {
    let default_interval = Duration::from_minutes(5).unwrap();

    spawn(move || {
      loop {
        match Self::job(daemon.clone()) {
          Ok(interval) => {
            sleep(interval.to_std());
          }
          Err(error) => {
            sleep(default_interval.to_std());

            let now = DateTime::now().to_iso_8601_like();
            eprintln!("{now}: {error:?}\n---------------------------------------------")
          }
        }
      }
    });
  }

  pub fn job(daemon: DaemonMutex) -> Result<Duration, GenericError> {
    let now = DateTime::now();
    
    let mut daemon = daemon.lock().map_err(|error| 
      error.change_context("perform synchronization job")
    )?;

    let private_password = daemon
      .state
      .user_screen_access_regulation_common_info
      .private_password()
      .clone();

    let mut errors = Vec::new();
    
    for user in &mut daemon.state.users {
      if let Err(error) = user.screen_access_regulator.apply(
        now, 
        &user.operating_system_username, 
        &user.operating_system_password, 
        &private_password,
      ) {
        errors.push(error);
      }
    }
    
    todo!()
  }
}