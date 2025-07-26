use windows::Win32::{
  
  Foundation::{
    HANDLE,
  },

  Security::{
    TOKEN_PRIVILEGES,
    TOKEN_ADJUST_PRIVILEGES,
    TOKEN_QUERY,
  },
  
};

fn poweroff() {
  unsafe {
    let token: HANDLE;
    let token: TOKEN_PRIVILEGES;

    // Get a token for this process.
    if (!OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &token)) {
      return;
    }

    // Get the LUID for the shutdown privilege.
    LookupPrivilegeValue(NULL, SE_SHUTDOWN_NAME, &token.Privileges[0].Luid);

    token.PrivilegeCount = 1;  // one privilege to set
    token.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

    // Get the shutdown privilege for this process.
    AdjustTokenPrivileges(hToken, FALSE, &tkp, 0, (PTOKEN_PRIVILEGES)NULL, 0);

    // Exit Windows.
    ExitWindowsEx(
      EWX_SHUTDOWN 
      | 
      EWX_FORCE,

      SHTDN_REASON_MAJOR_OPERATINGSYSTEM 
      | 
      SHTDN_REASON_MINOR_UPGRADE 
      | 
      SHTDN_REASON_FLAG_PLANNED,
    );
  }
}