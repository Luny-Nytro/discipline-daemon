use windows::core::{PSTR, PCSTR, PCWSTR};
use crate::utility::myAlloc;

pub fn createZeroedPstr(size: usize) -> PSTR {
  PSTR::from_raw(myAlloc(size))
}
// pub fn pcstrFromStr(arg: String) -> PCSTR {
//   return PCSTR::from_raw(arg.as_bytes() as *const [u8]);
// }
pub fn pcwstrFromStr(arg: String) -> PCWSTR {
  let mut vec_: Vec<u16> = vec![];
  for x in arg.encode_utf16() {
    // println!("char: {}", String::from_utf16(&[ x ]).unwrap());
    vec_.push(x);  
  }
  vec_.push(0);


  let ptr = vec_.as_ptr();

  return PCWSTR::from_raw(ptr);
}

use std::alloc::{Layout, alloc};

pub fn myAlloc(size: usize) -> *mut u8 {
  unsafe {
    alloc(Layout::from_size_align(size, 1).unwrap())
  }
}