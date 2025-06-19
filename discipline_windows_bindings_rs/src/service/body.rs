use std::ffi::OsString;

static mut SVC_BODY: Option<fn(Vec<OsString>)> = None;

pub fn initialize(body: fn(Vec<OsString>)) {
  unsafe {
    SVC_BODY = Some(body)
  }
}

pub fn run(arguments: Vec<OsString>) {
  if let Some(body) = unsafe { SVC_BODY } {
    body(arguments);
  }
}