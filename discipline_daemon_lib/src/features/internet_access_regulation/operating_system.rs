use std::process::Command;
use crate::{OperatingSystemUserId, OperatingSystemUsername};

#[derive(Debug)]
pub(super) struct OperatingSystemCalls;

impl OperatingSystemCalls {
  pub(super) fn new() -> Self {
    Self {}
  }

  pub(super) fn get_user_id(username: &OperatingSystemUsername) -> Result<OperatingSystemUserId, ()> {
    let output = Command::new("id")
      .arg("-u")
      .arg(username.as_ref()) 
      .output();
  
    let output = match output {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.GetUserId: \nError: {error}");
        return Err(());
      }
    };

    if output.status.success() {
      let user_id = match String::from_utf8(output.stdout) {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.GetUserId.StdoutToString: \n{error}.");
          return Err(());
        }
      };

      let user_id = match user_id.trim().to_string().parse::<u32>() {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.GetUserId.ParseUserId: \nError: {error}.");
          return Err(());
        }
      };

      return Ok(OperatingSystemUserId::new(user_id))
    } 
    
    match String::from_utf8(output.stderr) {
      Ok(stderr) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.GetUserId: \nUser: {username}. \nStderr: {stderr}");
        Err(())
      }
      Err(_) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.GetUserId: \nUser: {username}.");
        Err(())
      }
    }
  }

  pub(super) fn block_networking_access_for_user(&mut self, user_id: &OperatingSystemUserId, username: &OperatingSystemUsername) -> Result<(), ()> {
    let output = Command::new("sudo")
      .arg("iptables")
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
      Ok(output) => {
        output
      }
      Err(error) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.BlockNetworkAccessForUser: \nUser: {username}. \nError: {error}");
        return Err(());
      }
    };

    if output.status.success() {
      return Ok(());
    }

    match String::from_utf8(output.stderr) {
      Ok(stderr) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.BlockNetworkAccessForUser: \nUser: {username}. \nStderr: {stderr}");
        Err(())
      }
      Err(_) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.BlockNetworkAccessForUser. \nUser: {username}.");
        Err(())
      }
    }
  }

  pub(super) fn allow_networking_access_for_user(&mut self, user_id: &OperatingSystemUserId, username: &OperatingSystemUsername) -> Result<(), ()> {
    let output = Command::new("sudo")
      .arg("iptables")
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
      Ok(output) => {
        output
      }
      Err(error) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.AllowNetworkAccessForUser: \nUser: {username}. \nError: {error}");
        return Err(());
      }
    };

    if output.status.success() {
      return Ok(());
    }

    match String::from_utf8(output.stderr) {
      Ok(stderr) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.AllowNetworkAccessForUser: \nUser: {username}. \nStderr: {stderr}");
        Err(())
      }
      Err(_) => {
        eprintln!("Discipline.NetworkingAccess.OperatingSystemCalls.AllowNetworkAccessForUser. \nUser: {username}.");
        Err(())
      }
    }
  }
}