use windows::Win32::System::Services::DeleteService;
// use windows::Win32::System::Services::OpenServiceA;
use windows::Win32::System::Services::OpenServiceW;
use windows::Win32::System::Services::SERVICE_DEMAND_START;
use windows::Win32::System::Services::SERVICE_ERROR_NORMAL;
use windows::core::PCSTR;
// use super::errctrl::ErrorCtrl;
use super::startup::StartMode;
// use super::utility::pcstrFromStr;
use super::utility::pcwstrFromStr;
use windows::Win32::System::Services::CloseServiceHandle;
// use windows::Win32::System::Services::CreateServiceA;
use windows::Win32::System::Services::CreateServiceW;
// use windows::Win32::System::Services::OpenSCManagerA;
use windows::Win32::System::Services::OpenSCManagerW;
use windows::Win32::System::Services::SC_MANAGER_ALL_ACCESS;
use windows::Win32::System::Services::SERVICE_WIN32_OWN_PROCESS;

pub struct AddServiceArg {
  id: String,
  name: String,
  start: StartMode,
  binary: String,
  // username: &'username str,
  // password: &'password str,
}

fn Pcstr(string: String) -> PCSTR {
  let mut y = string.clone();
  let x: &mut Vec<u8> = unsafe { y.as_mut_vec() };
  x.push(0);
  PCSTR::from_raw(x.as_ptr())
}
pub fn add(id: String) {
  unsafe {
    // println!("a: {}", pcwstrFromStr(id).to_hstring().unwrap());
    // println!("a: {}", pcwstrFromStr(name).to_hstring().unwrap());

    let manager = OpenSCManagerW(
      None, 
      None, 
      SC_MANAGER_ALL_ACCESS,
    ).expect("OpenSCManagerW failed");


    let service = CreateServiceW(
      manager, 
      pcwstrFromStr(id),
      None, 
      SC_MANAGER_ALL_ACCESS, 
      SERVICE_WIN32_OWN_PROCESS, 
      SERVICE_DEMAND_START, 
      SERVICE_ERROR_NORMAL, 
      None, 
      None, 
      None, 
      None, 
      None, 
      None,
    ).unwrap();

    CloseServiceHandle(manager).expect("CloseServiceHandle(scMangaer)");
    CloseServiceHandle(service).expect("CloseServiceHandle(service)");
  };
}

pub fn remove(id: String) {
  unsafe {
    let mangager = OpenSCManagerW(
      None, 
      None, 
      SC_MANAGER_ALL_ACCESS,
    ).expect("OpenSCManagerW failed");
  
  
    let svc = OpenServiceW(
      mangager, 
      pcwstrFromStr(id), 
      SC_MANAGER_ALL_ACCESS,
    ).unwrap();
  
    DeleteService(svc).unwrap();
    CloseServiceHandle(mangager).expect("CloseServiceHandle(scMangaer)");
    CloseServiceHandle(svc).expect("CloseServiceHandle(service)");
  }
}

pub fn test() {
  add(String::from("Awesome"));

  // remove(String::from("Moon"));
}