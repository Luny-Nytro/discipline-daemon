use std::process::Command;
use crate::GenericError;
use super::*;

pub fn terminate_user_sessions_using_loginctl(user_id: UserId) -> Result<(), GenericError> {
  let output = Command::new("loginctl")
    .arg("terminate-user")
    .arg(user_id.as_raw().to_string())
    .output()
    .map_err(|error| {
      GenericError::new("Gracefully logout operating system user")
        .add_error("Failed to execute the 'loginctl terminate-user' linux command")
        .add_attachment("user id", user_id.as_raw().to_string())
        .add_attachment("io error", error.to_string())
    })?;

  if output.status.success() {
    return Ok(());
  }

  match String::from_utf8(output.stderr) {
    Ok(stderr) => Err(GenericError::new("Gracefully logout operating system user")
      .add_error("The 'loginctl terminate-user' linux command failed")
      .add_attachment("user id", user_id.as_raw().to_string())
      .add_attachment("'loginctl terminate-user' stderr", stderr)),
    Err(error) => Err(GenericError::new("Gracefully logout operating system user")
      .add_error("The 'loginctl terminate-user' linux command failed and stderr isn't valid utf8")
      .add_attachment("user id", user_id.as_raw().to_string())
      .add_attachment("utf8 parse error", error.to_string())),
  }
}


// TODO: Use kernel interfaces and systemd loginctl dbus api
pub fn terminate_user_session(
  username: &UserName,
) -> Result<(), GenericError> {
  let username = username.as_ref();

  let output = Command::new("pkill")
    .arg("-TERM")
    .arg("-u")
    .arg(username)
    .output()
    .map_err(|error| {
      GenericError::new("Gracefully logout operating system user")
        .add_error("Failed to execute the 'pkill' linux command")
        .add_attachment("username", username)
        .add_attachment("io error", error.to_string())
    })?;

  if output.status.success() {
    return Ok(());
  }

  match String::from_utf8(output.stderr) {
    Ok(stderr) => Err(GenericError::new("Gracefully logout operating system user")
      .add_error("The 'pkill' linux command failed")
      .add_attachment("username", username)
      .add_attachment("'pkill' stderr", stderr)),
    Err(error) => Err(GenericError::new("Gracefully logout operating system user")
      .add_error("The 'pkill' linux command failed and stderr isn't valid utf8")
      .add_attachment("username", username)
      .add_attachment("utf8 parse error", error.to_string())),
  }
}
