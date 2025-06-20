mod utility;
use utility::*;

mod proxy_server;
mod daemon;
mod database;
use daemon::*;
mod api_server;

fn main() {
  let daemon = Daemon::open("database_path", 90, 91).unwrap();
  daemon.run();
}