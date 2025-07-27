use super::*;
use std::process::Command;

// TODO: Write functions that allow and block netowking by enabling and disabling NetworkManager


pub fn block_inbound_network_traffic_for_user(
  user_id: &UserId,
  user_name: &UserName,
) -> Result<(), ()> {
  let output = Command::new("iptables")
    .arg("-A")
    .arg("INPUT")
    .arg("-m")
    .arg("owner")
    .arg("--uid-owner")
    .arg(user_id.as_raw().to_string())
    .arg("-j")
    .arg("DROP")
    .output();

  let output = match output {
    Ok(output) => output,
    Err(error) => {
      eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.BlockNetworkAccessForUser: \nUser: {user_name}. \nError: {error}");
      return Err(());
    }
  };

  if output.status.success() {
    return Ok(());
  }

  match String::from_utf8(output.stderr) {
    Ok(stderr) => {
      eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.BlockNetworkAccessForUser: \nUser: {user_name}. \nStderr: {stderr}");
      Err(())
    }
    Err(_) => {
      eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.BlockNetworkAccessForUser. \nUser: {user_name}.");
      Err(())
    }
  }
}

pub fn allow_inbound_network_traffic_for_user(
  user_id: &UserId,
  user_name: &UserName,
) -> Result<(), ()> {
  let output = Command::new("iptables")
    .arg("-D")
    .arg("INPUT")
    .arg("-m")
    .arg("owner")
    .arg("--uid-owner")
    .arg(user_id.as_raw().to_string())
    .arg("-j")
    .arg("DROP")
    .output();

  let output = match output {
    Ok(output) => output,
    Err(error) => {
      eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.AllowNetworkAccessForUser: \nUser: {user_name}. \nError: {error}");
      return Err(());
    }
  };

  if output.status.success() {
    return Ok(());
  }

  match String::from_utf8(output.stderr) {
    Ok(stderr) => {
      eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.AllowNetworkAccessForUser: \nUser: {user_name}. \nStderr: {stderr}");
      Err(())
    }
    Err(_) => {
      eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.AllowNetworkAccessForUser. \nUser: {user_name}.");
      Err(())
    }
  }
}
