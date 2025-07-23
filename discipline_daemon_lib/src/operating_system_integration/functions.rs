use std::ffi::CStr;
use std::mem::MaybeUninit;
use super::*;

pub enum RetrieveUserInfoGivenIdReturn {
  Success { 
    user_name: OperatingSystemUserName, 
    user_password: OperatingSystemUserPassword
  },
  NoSuchUser,
  Error,
}

pub fn retrieve_user_info_given_user_id(user_id: OperatingSystemUserId) -> RetrieveUserInfoGivenIdReturn {
  unsafe {
    // The user information (including name and other stuff) will eventually
    // be stored here.
    let mut user_information = MaybeUninit::<libc::passwd>::uninit();

    let buffer_length = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
      -1 => {
        1024 // Default buffer size if sysconf fails
      }
      value => {
        value as usize
      }
    };
    
    // The contents of the strings in "user_information" will be stored
    // here because "getpwuid_r" doesn't allocate memory.
    let mut buffer = vec![0u8; buffer_length];
    
    // If "getpwuid_r" succeeds, this pointer will point to the 
    // "user_information" binding above.
    //
    // However, if "getpwuid_r" fails or if there is no user with the provided id,
    // this will be the null pointer.
    //
    // This is weird. Why would we need this? We will access the user information through
    // the "user_information" variable and will check for errors through the "status"
    // variable, so why is this needed? You are right. This is NOT needed. It's just a 
    // hostrical baggage we have to deal with.
    let mut historical_baggage = std::ptr::null_mut();
    
    // Whether "getpwuid_r" succeeded or failed.
    let status = libc::getpwuid_r(
      user_id.as_raw(),
      user_information.as_mut_ptr(),
      buffer.as_mut_ptr() as *mut libc::c_char,
      buffer_length,
      &mut historical_baggage,
    );

    match status {
      // The operation succeeded
      0 => {
        // noop
      }
      // The user with the specified UID does not exist.
      libc::ENOENT => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Permission Denied: This process lacks permission to read /etc/passwd or a 
      // user database (e.g., LDAP/NIS).
      libc::EACCES => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Interrupted by Signal: The call was interrupted by a signal (e.g., SIGINT).
      // We should try again in this case.
      libc::EINTR => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Buffer Too Small: The buffer we provided is too small.
      libc::ERANGE => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Rare/System-Level Errors (less common but possible):
      // Low-level I/O error while reading /etc/passwd or a user database.
      libc::EIO => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // EMFILE / ENFILE (Too Many Open Files): Process or system file descriptor 
      // limit reached.
      libc::EMFILE | libc::ENFILE => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Bad Memory Address: Invalid buffer pointer passed to "getpwuid_r".
      libc::EFAULT => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Invalid Argument: Invalid parameters (e.g., NULL output pointer)
      libc::EINVAL => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Out of Memory: Kernel or libc failed to allocate memory.
      libc::ENOMEM => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
      // Error not specified in the man page for "getpwuid_r"
      _ => {
        return RetrieveUserInfoGivenIdReturn::Error;
      }
    }

    if historical_baggage.is_null() {
      return RetrieveUserInfoGivenIdReturn::NoSuchUser;
    }

    let user_information = user_information.assume_init();

    let Some(user_name) = OperatingSystemUserName::new(
      CStr::from_ptr(user_information.pw_name)
        .to_string_lossy().into_owned()
    ) else {
      return RetrieveUserInfoGivenIdReturn::Error;
    };

    let Some(user_password) = OperatingSystemUserPassword::new(
      CStr::from_ptr(user_information.pw_passwd)
        .to_string_lossy().into_owned()
    ) else {
      return RetrieveUserInfoGivenIdReturn::Error;
    };

    RetrieveUserInfoGivenIdReturn::Success { 
      user_name, 
      user_password,
    }
  }
}

pub enum RetrieveUserInfoGivenNameReturn {
  Success { user_id: OperatingSystemUserId, user_password: OperatingSystemUserPassword },
  NoSuchUser,
  Error,
}

pub fn retrieve_user_info_given_user_name(user_name: &OperatingSystemUserName) -> RetrieveUserInfoGivenNameReturn {
  unsafe {
    // The user information (including name and other stuff) will eventually
    // be stored here.
    let mut user_information = MaybeUninit::<libc::passwd>::uninit();

    let buffer_length = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
      -1 => {
        1024 // Default buffer size if sysconf fails
      }
      value => {
        value as usize
      }
    };
    
    // The contents of the strings in "user_information" will be stored
    // here because "getpwnam_r" doesn't allocate memory.
    let mut buffer = vec![0u8; buffer_length];
    
    // If "getpwnam_r" succeeds, this pointer will point to the 
    // "user_information" binding above.
    //
    // However, if "getpwnam_r" fails or if there is no user with the provided id,
    // this will be the null pointer.
    //
    // This is weird. Why would we need this? We will access the user information through
    // the "user_information" variable and will check for errors through the "status"
    // variable, so why is this needed? You are right. This is NOT needed. It's just a 
    // hostrical baggage we have to deal with.
    let mut historical_baggage = std::ptr::null_mut();
    
    // Whether "getpwnam_r" succeeded or failed.
    let status = libc::getpwnam_r(
      user_name.as_ref().as_ptr() as *const libc::c_char,
      user_information.as_mut_ptr(),
      buffer.as_mut_ptr() as *mut libc::c_char,
      buffer_length,
      &mut historical_baggage,
    );

    match status {
      // The operation succeeded
      0 => {
        // noop
      }
      // The user with the specified UID does not exist.
      libc::ENOENT => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Permission Denied: This process lacks permission to read /etc/passwd or a 
      // user database (e.g., LDAP/NIS).
      libc::EACCES => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Interrupted by Signal: The call was interrupted by a signal (e.g., SIGINT).
      // We should try again in this case.
      libc::EINTR => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Buffer Too Small: The buffer we provided is too small.
      libc::ERANGE => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Rare/System-Level Errors (less common but possible):
      // Low-level I/O error while reading /etc/passwd or a user database.
      libc::EIO => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // EMFILE / ENFILE (Too Many Open Files): Process or system file descriptor 
      // limit reached.
      libc::EMFILE | libc::ENFILE => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Bad Memory Address: Invalid buffer pointer passed to "getpwnam_r".
      libc::EFAULT => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Invalid Argument: Invalid parameters (e.g., NULL output pointer)
      libc::EINVAL => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Out of Memory: Kernel or libc failed to allocate memory.
      libc::ENOMEM => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
      // Error not specified in the man page for "getpwnam_r"
      _ => {
        return RetrieveUserInfoGivenNameReturn::Error;
      }
    }

    if historical_baggage.is_null() {
      return RetrieveUserInfoGivenNameReturn::NoSuchUser;
    }
    
    let user_information = user_information.assume_init();
    let user_id = OperatingSystemUserId::new(user_information.pw_uid);

    let Some(user_password) = OperatingSystemUserPassword::new(
      CStr::from_ptr(user_information.pw_passwd)
        .to_string_lossy().into_owned()
    ) else {
      return RetrieveUserInfoGivenNameReturn::Error;
    };

    RetrieveUserInfoGivenNameReturn::Success { 
      user_id, 
      user_password,
    }
  }
}

// TODO: Use pam
pub fn change_user_password(
  username: &OperatingSystemUserName,
  new_password: &OperatingSystemUserPassword,
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

// TODO: Use kernel interfaces and systemd loginctl dbus api
pub fn terminate_user_session(
  username: &OperatingSystemUserName,
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
