use clap::{Parser, command};
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use crate::{GenericError, InternalErrorLogger};
use crate::database::Database;
use crate::operating_system_integration::OperatingSystemIntegration;

pub struct Configuration {
  database_directory_path: PathBuf,
  api_tcp_port: u16,
}

impl Configuration {
  pub fn new(
    database_directory_path: PathBuf,
    api_tcp_port: u16,
  ) -> Self {
    Self {
      database_directory_path,
      api_tcp_port,
    }
  }

  pub fn api_tcp_port(&self) -> u16 {
    self.api_tcp_port
  }

  pub fn database_directory_path(&self) -> &PathBuf {
    &self.database_directory_path
  }
}

// TODO: Rename to DaemonData
// TODO: Create Daemon struct that is just Daemon(Arc<DaemonData>)
// TODO: Make other code just use the new Daemon, instead.
pub struct Daemon {
  dropped: AtomicBool,
  database: Database,
  configuration: Configuration,
  internal_error_logger: InternalErrorLogger,
  operating_system_integration: OperatingSystemIntegration,
}

impl Daemon {
  pub fn open_with_configuration(configuration: Configuration) -> Result<Daemon, GenericError> {
    let database = Database::open(configuration.database_directory_path()).map_err(|error|
      error
        .change_context("opening a connection to the database")
        .change_context("creating daemon")
    )?;

    let operating_system_integration = OperatingSystemIntegration::open(&database)?;

    // TODO: Run operating_system_integration and api server.

    Ok(Daemon {
      dropped: AtomicBool::new(false),
      database,
      configuration,
      internal_error_logger: InternalErrorLogger::new(),
      operating_system_integration,
    })
  }

  pub fn database(&self) -> &Database {
    &self.database
  }
  
  pub fn configuration(&self) -> &Configuration {
    &self.configuration
  }

  pub fn operating_system_integration(&self) -> &OperatingSystemIntegration {
    &self.operating_system_integration
  }

  pub fn internal_logger(&self) -> InternalErrorLogger {
    self.internal_error_logger
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
struct CommandLineArguments {
  /// The path to the directory of the database.
  database_directory_path: PathBuf,

  /// Age of the Supreme Emperor (optional)
  api_tcp_port: u16,
}

impl Daemon {
  pub fn open_with_command_line_arguments() -> Result<Self, GenericError> {
    let arguments = CommandLineArguments::parse();
    if arguments.database_directory_path.as_os_str().len() > 300 {
      return Err(
        GenericError::new("open Daemon from command line arguments")
          .add_error("the 'database_path' argument is longer than 300 characters")
          .add_attachment("'database_path' argument", arguments.database_directory_path.to_string_lossy())
      );
    }

    let configuration = Configuration::new(
      arguments.database_directory_path, 
      arguments.api_tcp_port,
    );

    Daemon::open_with_configuration(configuration)
    .map_err(|error|
      error.change_context("open Daemon from command line arguments")
    )
  }
}