use crate::{Duration, GenericError, OperatingSystemUserId};

pub struct OsUser {
  regulation_enforcing_interval: Duration,
  is_screen_access_blocked: Duration,
  // pub(super) private_password: OperatingSystemPassword,

}

pub struct OperatingSystemIntegration {

}

impl OperatingSystemIntegration {
  pub fn block_user_access(&self, user_id: &OperatingSystemUserId) {
    todo!()
  }
  pub fn allow_user_access(&self, user_id: &OperatingSystemUserId) {
    todo!()
  }
}


  // fn allow_user_access(
  //   &mut self,
  //   username: &OperatingSystemUsername,
  //   password: &OperatingSystemPassword,
  // ) -> 
  //   Result<(), GenericError> 
  // {
  //   if !self.is_user_screen_access_blocked {
  //     return Ok(());
  //   }

  //   match self
  //     .operating_system_calls
  //     .change_user_password(username, password) 
  //   {
  //     Ok(_) => {
  //       self.is_user_screen_access_blocked = false;
  //       Ok(())
  //     }
  //     Err(error) => {
  //       Err(
  //         error.change_context("Allow user screen access")
  //       )
  //     }
  //   }
  // }

  // fn block_user_access(
  //   &mut self, 
  //   username: &OperatingSystemUsername,
  //   private_password: &OperatingSystemPassword,
  // ) -> 
  //   Result<(), GenericError> 
  // {
  //   if self.is_user_screen_access_blocked {
  //     return Ok(());
  //   }

  //   match self
  //     .operating_system_calls
  //     .change_user_password(username, private_password) 
  //   {
  //     Ok(_) => {
  //       self.is_user_screen_access_blocked = false;
  //     }
  //     Err(error) => {
  //       return Err(
  //         error.change_context("Block user screen access")
  //       )
  //     }
  //   }

  //   self
  //     .operating_system_calls
  //     .gracefully_logout_user(username)
  //     .map_err(|error| error.change_context("Block user screen access"))
  // }


use std::io::Write;
use std::process::Command;
use crate::{OperatingSystemPassword, OperatingSystemUsername};

#[derive(Debug, Clone)]
pub struct OperatingSystemCalls {}

impl OperatingSystemCalls {
  pub fn new() -> Self {
    Self {}
  }

  pub fn change_user_password(&mut self, 
    username: &OperatingSystemUsername, 
    new_password: &OperatingSystemPassword,
  ) -> Result<(), GenericError> {
    let mut chpasswd = Command::new("chpasswd")
      .spawn()
      .map_err(|error| 
        GenericError::new("Change operating system user password")
          .add_error("Failed to call the 'chpasswd' linux program")
          .add_attachment("io error", error.to_string())
          .add_attachment("username", username.as_ref())
          .add_attachment("new password", new_password.as_ref())
      )?;
  
    let Some(mut writer) = chpasswd.stdin.take() else {
      return Err(
        GenericError::new("Change operating system user password")
          .add_error("Failed to take stdin writer of the 'chpasswd' linux program")
          .add_attachment("username", username.as_ref())
          .add_attachment("new password", new_password.as_ref())
      )
    };
    
    let username = username.as_ref();
    let new_password = new_password.as_ref();

    if let Err(error) = writeln!(writer, "{username}:{new_password}") {
      return Err(
        GenericError::new("Change operating system user password")
          .add_error("Failed to write to the 'chpasswd' linux program")
          .add_attachment("username", username)
          .add_attachment("new password", new_password)
          .add_attachment("io error", error.to_string())
      );
    }

    let output = chpasswd
      .wait_with_output()
      .map_err(|error|
        GenericError::new("Change operating system user password")
        .add_error("The 'chpasswd' linux program failed")
        .add_attachment("username", username)
        .add_attachment("new password", new_password)
        .add_attachment("io error", error.to_string())
      )?;

    if output.status.success() {
      return Ok(());
    }

    match String::from_utf8(output.stderr) {
      Ok(stderr) => {
        Err(
          GenericError::new("Change operating system user password")
          .add_error("The 'chpasswd' linux program failed")
          .add_attachment("username", username)
          .add_attachment("new password", new_password)
          .add_attachment("'chpasswd' stderr", stderr)
        )
      }
      Err(error) => {
        Err(
          GenericError::new("Change operating system user password")
          .add_error("The 'chpasswd' linux program faild and stderr isn't valid utf8")
          .add_attachment("username", username)
          .add_attachment("new password", new_password)
          .add_attachment("utf8 parse error", error.to_string())
        )
      }
    }
  }

  pub fn gracefully_logout_user(
    &self, 
    username: &OperatingSystemUsername,
  ) -> Result<(), GenericError> {
    let username = username.as_ref();

    let output = Command::new("pkill")
      .arg("-TERM")
      .arg("-u")
      .arg(username)
      .output()
      .map_err(|error|
        GenericError::new("Gracefully logout operating system user")
          .add_error("Failed to execute the 'pkill' linux command")
          .add_attachment("username", username)
          .add_attachment("io error", error.to_string())
      )?;

    if output.status.success() {
      return Ok(());
    }

    match String::from_utf8(output.stderr) {
      Ok(stderr) => {
        Err(
          GenericError::new("Gracefully logout operating system user")
          .add_error("The 'pkill' linux command failed")
          .add_attachment("username", username)
          .add_attachment("'pkill' stderr", stderr)
        )
      }
      Err(error) => {
        Err(
          GenericError::new("Gracefully logout operating system user")
          .add_error("The 'pkill' linux command failed and stderr isn't valid utf8")
          .add_attachment("username", username)
          .add_attachment("utf8 parse error", error.to_string())
        )
      }
    }
  }
}
