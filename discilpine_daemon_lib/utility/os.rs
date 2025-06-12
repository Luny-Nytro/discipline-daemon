// pub fn is_root_user() -> bool {
//   unsafe {
//     libc::geteuid() == 0
//   }
// }

// pub fn my_shutdown() {
//   unsafe {
//     if is_root_user()  {
//       libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF);
//     }
//   }
// }

// pub fn logout() -> Result<(), ()> {
//   match Command::new("logout").output() {
//     Ok(_) => Ok(()),
//     Err(_) => Err(()),
//   }
// }

// pub fn shutdown() -> Result<(), ()> {
//   let output = Command::new("shutdown")
//     .arg("-h")
//     .arg("now")
//     .output();

//   match output {
//     Ok(_) => Ok(()),
//     Err(_) => Err(()),
//   }
// }

// pub fn allow_network() -> Result<(), ()> {
//   match Command::new("systemctl").arg("start").arg("NetworkFeature").output() {
//     Ok(_) => Ok(()),
//     Err(_) => Err(())
//   }
// }

// pub fn block_network() -> Result<(), ()> {
//   match Command::new("systemctl").arg("stop").arg("NetworkFeature").output() {
//     Ok(_) => Ok(()),
//     Err(_) => Err(())
//   }
// }

// pub fn change_user_password(username: &str, password: &str) -> Result<(), ()> {
//   let chpasswd = Command::new("chpasswd").spawn();
//   let mut chpasswd = match chpasswd {
//     Ok(chpasswd) => chpasswd,
//     Err(_) => return Err(()),
//   };

  
//   let Some(mut stdin) = chpasswd.stdin.take() else {
//     return Err(())
//   };

//   match writeln!(stdin, "{username}:{password}") {
//     Ok(_) => Ok(()),
//     Err(_) => Err(())
//   }
// }

// pub struct PasswordChanger {
//   chpasswd: Child,
//   writer: ChildStdin,
// }

// impl PasswordChanger {
//   pub fn new() -> Result<Self, ()> {
//     let mut chpasswd = match Command::new("chpasswd").spawn() {
//       Ok(chpasswd) => chpasswd,
//       Err(_) => return Err(()),
//     };
  
    
//     let Some(writer) = chpasswd.stdin.take() else {
//       return Err(())
//     };

//     Ok(Self {
//       chpasswd,
//       writer,
//     })
//   }

//   pub fn change_password(&mut self, username: &str, password: &str) -> Result<(), ()> {
//     match writeln!(self.writer, "{username}:{password}") {
//       Ok(_) => Ok(()),
//       Err(_) => Err(())
//     }
//   }
// }



// pub fn shutdown() -> Result<(), ()> {
//   let output = Command::new("cmd.exe")
//     .arg("-C")
//     .arg("Shutdown.exe")
//     .arg("-s")
//     .arg("-t 00")
//     .output();

//   let Ok(output) = output else {
//     return Err(())
//   };

//   if output.status.success() {
//     return Ok(());
//   } else {
//     return Err(())
//   }
// }

// pub fn sign_out() -> Result<(), ()> {
//   let output = Command::new("cmd.exe")
//     .arg("-C")
//     .arg("signout")
//     .output();

//   let Ok(output) = output else {
//     return Err(())
//   };

//   if output.status.success() {
//     return Ok(());
//   } else {
//     return Err(())
//   }
// }

// fn shutdown() {
//   use std::env::consts::OS;

//   match OS {
//     "windows" => {
//       // let command = "Shutdown.exe -s -t 00";
//     }

//     "linux" => {
//       // let command = "sudo shutdown -h now";
//       // Command::
//     }
    
//     _ => {
//       // log that we could not shutdown the os
//     }
//   }

// }




// fn shutdown() {
//   use std::env::consts::OS;

//   match OS {
//     "windows" => {
//       // let command = "Shutdown.exe -s -t 00";
//       Command::new("cmd.exe")
//         .arg("-C")
//         .arg("Shutdown.exe")
//         .arg("-s")
//         .arg("-t 00")
//         .spawn();
//     }

//     "linux" => {
//       // let command = "sudo shutdown -h now";
//       // Command::
//     }
    
//     _ => {
//       // log that we could not shutdown the os
//     }
//   }

// }


// fn get_logged_in_users() -> HashSet<String> {
//   use std::ffi::CStr;
//   // use std::ptr;
//   use libc::{getutxent, setutxent, endutxent};

//   let mut users = HashSet::new();

//   unsafe {
//     // Rewind to the start of the utmp file
//     setutxent(); 

//     while let Some(entry) = getutxent().as_ref() {
//       if entry.ut_type == libc::USER_PROCESS {
//         let username = CStr::from_ptr(entry.ut_user.as_ptr()) // Convert to Rust string
//           .to_string_lossy()
//           .into_owned();

//         users.insert(username);
//       }
//     }

//     endutxent(); // Close the utmp file
//   }

//   println!("Discipline.ApplicationContext.GetLoggedInUsers: {:?}", users);
//   users
// }

// fn get_logged_in_users() -> HashSet<String> {
//   use libc::getlogin;
//   use std::ffi::CStr;
//   use std::ptr;

//   let mut users = HashSet::new();

//   unsafe {
//     let login = getlogin();
//     if login.is_null() {
//       // println!("Error: could not get the logged-in user.");
//     } else {
//       let c_str = CStr::from_ptr(login);
//       let str_slice = c_str.to_str().unwrap_or("Error converting to string");
//       // println!("Logged in user: {}", str_slice);
//     }
//   }

//   users
// }
