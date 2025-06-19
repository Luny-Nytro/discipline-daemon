use super::utility::createZeroedPstr;
use windows::Win32::NetworkManagement::NetManagement::UNLEN;
use windows::Win32::System::WindowsProgramming::{GetUserNameA, GetComputerNameA, MAX_COMPUTERNAME_LENGTH};

static maxUserNameSize: usize = (UNLEN + 1) as usize;
static maxComputerNameSize: usize = (MAX_COMPUTERNAME_LENGTH + 1) as usize;

pub fn readUserName() -> String {
  unsafe {
    let size: *mut u32 = &mut (maxUserNameSize.clone() as u32);
    let name = createZeroedPstr(maxUserNameSize);
    GetUserNameA(name, size).unwrap();
    name.to_string().unwrap()
  }
}
pub fn readComputerName() -> String {
  unsafe {
    let name = createZeroedPstr(maxComputerNameSize);
    let size: *mut u32 = &mut (maxComputerNameSize.clone() as u32);
    GetComputerNameA(name, size).unwrap();
    name.to_string().unwrap()
  }
}
pub fn readAccountName() -> String {
  format!("{computer}\\{user}", 
    computer = readComputerName(), 
    user = readUserName(),
  )
}

pub fn test() {
  println!("User name: {string}", string = readUserName());
  println!("Compouter name: {string}", string = readComputerName());
  println!("account name: {string}", string = readAccountName());
}