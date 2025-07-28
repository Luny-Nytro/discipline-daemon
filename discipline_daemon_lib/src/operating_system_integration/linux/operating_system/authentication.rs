use std::io::Write;
use std::process::Command;
use crate::GenericError;
use super::*;

// TODO: Use pam

pub fn change_user_password(
  username: &UserName,
  new_password: &UserPassword,
) -> Result<(), GenericError> {
  let mut chpasswd = Command::new("chpasswd").spawn().map_err(|error| {
    GenericError::new("Change operating system user password")
      .add_error("Failed to call the 'chpasswd' linux program")
      .add_attachment("io error", error.to_string())
      .add_attachment("username", username.as_ref())
      .add_attachment("new password", new_password.as_ref())
  })?;

  let Some(mut writer) = chpasswd.stdin.take() else {
    return Err(GenericError::new("Change operating system user password")
      .add_error("Failed to take stdin writer of the 'chpasswd' linux program")
      .add_attachment("username", username.as_ref())
      .add_attachment("new password", new_password.as_ref()));
  };

  let username = username.as_ref();
  let new_password = new_password.as_ref();

  if let Err(error) = writeln!(writer, "{username}:{new_password}") {
    return Err(GenericError::new("Change operating system user password")
      .add_error("Failed to write to the 'chpasswd' linux program")
      .add_attachment("username", username)
      .add_attachment("new password", new_password)
      .add_attachment("io error", error.to_string()));
  }

  let output = chpasswd.wait_with_output().map_err(|error| {
    GenericError::new("Change operating system user password")
      .add_error("The 'chpasswd' linux program failed")
      .add_attachment("username", username)
      .add_attachment("new password", new_password)
      .add_attachment("io error", error.to_string())
  })?;

  if output.status.success() {
    return Ok(());
  }

  match String::from_utf8(output.stderr) {
    Ok(stderr) => Err(GenericError::new("Change operating system user password")
      .add_error("The 'chpasswd' linux program failed")
      .add_attachment("username", username)
      .add_attachment("new password", new_password)
      .add_attachment("'chpasswd' stderr", stderr)),
    Err(error) => Err(GenericError::new("Change operating system user password")
      .add_error("The 'chpasswd' linux program faild and stderr isn't valid utf8")
      .add_attachment("username", username)
      .add_attachment("new password", new_password)
      .add_attachment("utf8 parse error", error.to_string())),
  }
}
