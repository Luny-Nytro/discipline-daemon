// use std::path::PathBuf;

// extern crate cc;

// pub struct Builder {

// }

// pub fn local_build(
//   original_llhttp_directory: PathBuf,
//   output_directory: PathBuf,
// ) {
//   let llhttp_bindings_file_path = output_directory.join("llhttp.rs");

//   let llhttp_c_header_path = original_llhttp_directory
//     .join("build")
//     .join("llhttp.h");

//   let llhttp_c_code_path = original_llhttp_directory
//     .join("build")
//     .join("c")
//     .join("llhttp.c");

//   bindgen::Builder::default()
//     .header(llhttp_c_header_path.to_string_lossy())
//     .use_core()
//     .ctypes_prefix("::libc")
//     .allowlist_var("^llhttp_.*")
//     .allowlist_type("^llhttp_.*")
//     .allowlist_function("^llhttp_.*")
//     .size_t_is_usize(true)
//     .rust_target(bindgen::LATEST_STABLE_RUST)
//     .derive_copy(true)
//     .derive_debug(true)
//     .derive_default(true)
//     .derive_partialeq(true)
//     .newtype_enum("llhttp_errno")
//     .newtype_enum("llhttp_flags")
//     .newtype_enum("llhttp_lenient_flags")
//     .newtype_enum("llhttp_type")
//     .newtype_enum("llhttp_method")
//     .generate()
//     .unwrap()
//     .write_to_file(llhttp_bindings_file_path)
//     .unwrap()
//   ;

//   cc::Build::new()
//     .file(llhttp_c_code_path.to_string_lossy().into_owned())
//     .include(llhttp_c_header_path.to_string_lossy().into_owned())
//     .warnings(false)
//     .compile("llhttp");
// }

// fn main() {
//   local_build(
//     std::env::current_dir().unwrap().join("../llhttp"), 
//     std::env::current_dir().unwrap().join("src"),
//   );
// }
