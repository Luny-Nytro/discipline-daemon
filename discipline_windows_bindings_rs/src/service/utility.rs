use std::ffi::OsString;

use widestring::WideCStr;

pub type DynErr = Box<dyn std::error::Error>;
pub type DynErrOr<T> = Result<T, DynErr>;


/// Parse raw arguments received in `service_main` into `Vec<OsString>`.
///
/// This is an implementation detail and *should not* be called directly!
#[doc(hidden)]
pub unsafe fn parse_service_arguments(argc: u32, argv: *mut *mut u16) -> Vec<OsString> {
  (0..argc)
    .map(|i| {
      let array_element_ptr: *mut *mut u16 = argv.offset(i as isize);
      WideCStr::from_ptr_str(*array_element_ptr).to_os_string()
    })
    .collect()
}

