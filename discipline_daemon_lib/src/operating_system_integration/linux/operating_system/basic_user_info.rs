use std::ffi::CStr;
use std::mem::MaybeUninit;
use super::*;

// TODO: Rename to BasicUserInfo
pub struct RetrievedUserInfo {
  pub user_id: UserId,
  pub user_name: UserName, 
  pub user_password: UserPassword
}

// TODO: Rename to RetrieveBasicUserInfoReturn
pub enum RetrieveUserInfoReturn {
  Success(RetrievedUserInfo),
  NoSuchUser,
  Error,
}

pub fn retrieve_user_info_given_user_id(user_id: UserId) -> RetrieveUserInfoReturn {
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
        return RetrieveUserInfoReturn::Error;
      }
      // Permission Denied: This process lacks permission to read /etc/passwd or a 
      // user database (e.g., LDAP/NIS).
      libc::EACCES => {
        return RetrieveUserInfoReturn::Error;
      }
      // Interrupted by Signal: The call was interrupted by a signal (e.g., SIGINT).
      // We should try again in this case.
      libc::EINTR => {
        return RetrieveUserInfoReturn::Error;
      }
      // Buffer Too Small: The buffer we provided is too small.
      libc::ERANGE => {
        return RetrieveUserInfoReturn::Error;
      }
      // Rare/System-Level Errors (less common but possible):
      // Low-level I/O error while reading /etc/passwd or a user database.
      libc::EIO => {
        return RetrieveUserInfoReturn::Error;
      }
      // EMFILE / ENFILE (Too Many Open Files): Process or system file descriptor 
      // limit reached.
      libc::EMFILE | libc::ENFILE => {
        return RetrieveUserInfoReturn::Error;
      }
      // Bad Memory Address: Invalid buffer pointer passed to "getpwuid_r".
      libc::EFAULT => {
        return RetrieveUserInfoReturn::Error;
      }
      // Invalid Argument: Invalid parameters (e.g., NULL output pointer)
      libc::EINVAL => {
        return RetrieveUserInfoReturn::Error;
      }
      // Out of Memory: Kernel or libc failed to allocate memory.
      libc::ENOMEM => {
        return RetrieveUserInfoReturn::Error;
      }
      // Error not specified in the man page for "getpwuid_r"
      _ => {
        return RetrieveUserInfoReturn::Error;
      }
    }

    if historical_baggage.is_null() {
      return RetrieveUserInfoReturn::NoSuchUser;
    }

    let user_information = user_information.assume_init();

    let Some(user_name) = UserName::new(
      CStr::from_ptr(user_information.pw_name)
        .to_string_lossy().into_owned()
    ) else {
      return RetrieveUserInfoReturn::Error;
    };

    let Some(user_password) = UserPassword::new(
      CStr::from_ptr(user_information.pw_passwd)
        .to_string_lossy().into_owned()
    ) else {
      return RetrieveUserInfoReturn::Error;
    };

    RetrieveUserInfoReturn::Success(RetrievedUserInfo { 
      user_id,
      user_name, 
      user_password,
    })
  }
}

pub fn retrieve_user_info_given_user_name(user_name: UserName) -> RetrieveUserInfoReturn {
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
        return RetrieveUserInfoReturn::Error;
      }
      // Permission Denied: This process lacks permission to read /etc/passwd or a 
      // user database (e.g., LDAP/NIS).
      libc::EACCES => {
        return RetrieveUserInfoReturn::Error;
      }
      // Interrupted by Signal: The call was interrupted by a signal (e.g., SIGINT).
      // We should try again in this case.
      libc::EINTR => {
        return RetrieveUserInfoReturn::Error;
      }
      // Buffer Too Small: The buffer we provided is too small.
      libc::ERANGE => {
        return RetrieveUserInfoReturn::Error;
      }
      // Rare/System-Level Errors (less common but possible):
      // Low-level I/O error while reading /etc/passwd or a user database.
      libc::EIO => {
        return RetrieveUserInfoReturn::Error;
      }
      // EMFILE / ENFILE (Too Many Open Files): Process or system file descriptor 
      // limit reached.
      libc::EMFILE | libc::ENFILE => {
        return RetrieveUserInfoReturn::Error;
      }
      // Bad Memory Address: Invalid buffer pointer passed to "getpwnam_r".
      libc::EFAULT => {
        return RetrieveUserInfoReturn::Error;
      }
      // Invalid Argument: Invalid parameters (e.g., NULL output pointer)
      libc::EINVAL => {
        return RetrieveUserInfoReturn::Error;
      }
      // Out of Memory: Kernel or libc failed to allocate memory.
      libc::ENOMEM => {
        return RetrieveUserInfoReturn::Error;
      }
      // Error not specified in the man page for "getpwnam_r"
      _ => {
        return RetrieveUserInfoReturn::Error;
      }
    }

    if historical_baggage.is_null() {
      return RetrieveUserInfoReturn::NoSuchUser;
    }
    
    let user_information = user_information.assume_init();
    let user_id = UserId::new(user_information.pw_uid);

    let Some(user_password) = UserPassword::new(
      CStr::from_ptr(user_information.pw_passwd)
        .to_string_lossy().into_owned()
    ) else {
      return RetrieveUserInfoReturn::Error;
    };

    RetrieveUserInfoReturn::Success(RetrievedUserInfo { 
      user_id, 
      user_name,
      user_password,
    })
  }
}
