
pub mod operations;
pub use operations::*;

mod from_cli_arguments;

use crate::{GenericError, IsOperation, State, StateSchema, SynchronizeSource};
use crate::database::Connection;

pub struct Daemon {
  pub state: State,
  pub schema: StateSchema,
  pub database_connection: Connection,
  pub http_server_address: String,
  pub is_running: bool,
}

// pub struct AppMutex(Arc<Mutex<App>>);

impl Daemon {
  pub fn open(
    database_file_path: &str,
    http_server_port: u32,
  ) -> 
    Result<Self, GenericError> 
  {
    let database_connection = Connection::new(database_file_path).map_err(|error|
      error.change_context("Failed to create App: Failed to open a connection to the database")
    )?;

    let state_database_adapter = StateSchema::new(database_connection.namespace()).map_err(|error|
      error.change_context("Failed to create App: Failed to create state database adapter")
    )?;

    state_database_adapter.initialize(&database_connection).map_err(|error|
      error.change_context("Failed to create App: Failed to initialize state database schema")
    )?;

    let state = state_database_adapter.load(&database_connection).map_err(|error|
      error.change_context("Failed to create App: Failed to load app state")
    )?;

    Ok(Daemon {
      state,
      is_running: false,
      database_connection,
      schema: state_database_adapter,
      http_server_address: format!("127.0.0.1:{http_server_port}"),
    })
  }

  pub fn open_and_run(
    database_file_path: &str,
    http_server_port: u32,
  ) -> Result<Self, ()> {
    let app = Self::open(database_file_path, http_server_port)?;
    app.run();
    Ok(app)
  }

  fn http_server_address(&self) -> &String {
    &self.http_server_address
  }

  pub fn execute<T>(&mut self, operation: T) -> T::Outcome 
  where 
    T: IsOperation
  {
    operation.execute(self)
  }

  pub fn run(&self) {
    {
      // Don't respawn below threads if they are already spawned.
      let mut app = self.0.lock().unwrap();
      if app.is_running {
        return;
      } else {
        app.is_running = true;
      }
    }

    let user_access_app = Arc::clone(&self.0);
    let user_access_thread = spawn(move || {
      loop {
        let interval = {
          let now = DateTime::now();
          let mut app = user_access_app.lock().unwrap();
          app.state.user_access.apply_actions(now);
          app.state.user_access.enforcing_interval()
        };

        sleep(interval.to_std());
      }
    });

    let network_access_enforcer_app = Arc::clone(&self.0);
    let network_access_enforcer_thread = spawn(move || {
      loop {
        let interval = {
          let now = DateTime::now();
          let mut app = network_access_enforcer_app.lock().unwrap();
          app.state.networking_access.apply_enforcers(now);
          app.state.networking_access.enforcing_interval()
        };

        sleep(interval.to_std());
      }
    });

    let synchronization_app = Arc::clone(&self.0);
    let synchronization_thread = spawn(move || {
      let interval = Duration::from_minutes(5).unwrap().to_std();

      loop {
        {
          let mut app = synchronization_app.lock().unwrap();
          synchronize::synchronize(&mut app);
        }

        sleep(interval);
      }
    });

    let http_server_app = self.clone();
    let http_server_thread = spawn(move || {
      http_server::run(http_server_app);
    });

    user_access_thread.join().unwrap();
    network_access_enforcer_thread.join().unwrap();
    synchronization_thread.join().unwrap();
    http_server_thread.join().unwrap();
  }
}