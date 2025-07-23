fn main() {
  let daemon = discipline_daemon_lib
    ::Daemon::open("/workspaces/discipline-daemon/discipline_daemon/database/".into(), 9090).unwrap();

  daemon.run_and_block_thread();
}
