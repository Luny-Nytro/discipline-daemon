use clap::{Parser, command};
use crate::debug;

use super::AppMutex;

#[derive(Parser, Debug)]
#[command(name = "Discipline", version = "1.0", author = "LunyNytro")]
struct Args {
  /// The path to the database file.
  database_path: String,

  /// Age of the Supreme Emperor (optional)
  http_server_port: u32,
}

impl AppMutex {
  pub fn open_from_command_line_arguments() -> Result<Self, ()> {
    debug!("AppMutex::open_from_command_line_arguments()");

    let arguments = Args::parse();
    if arguments.database_path.len() > 300 {
      eprintln!("Discipline.OpenFromCommandLineArguments.DatabsePathTooLong.");
      return Err(());
    }
    
    AppMutex::open(arguments.database_path, arguments.http_server_port)
  }

  pub fn open_from_command_line_arguments_then_run() -> Result<(), ()> {
    debug!("AppMutex::open_from_command_line_arguments_then_run()");

    let app = Self::open_from_command_line_arguments()?;
    app.run();
    Ok(())
  }
}