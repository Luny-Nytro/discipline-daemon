use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  String(String),
  Bool(bool),
  I8(i8),
  U8(u8),
  I16(i16),
  U16(u16),
  I32(i32),
  U32(u32),
  I64(i64),
  U64(u64),
  I128(i128),
  U128(u128),
  ISize(isize),
  USize(usize), 
  Number(i64), 
  // HttpReq(HttpReq),
  // HttpRes(HttpRes),
  Uri(),
  HttpReq(HttpReq),
  HttpRes(HttpRes)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpReq {
  pub uri: Uri,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
  CONNECT,
  HEAD,
  GET,
  POST,
  PATCH,
  DELETE,
  OPTIONS,
  PUT,
  TRACE,
  
  // TODO(me): Add all other http methods.
}

// impl crate::db::SerializeValue for Method {
//   fn serialize(&self) -> String {
//     todo!()
//   }
// }

pub struct Headers {

}

impl HttpReq {
  pub fn get_uri_filename(&self) -> &str {
    todo!()
  }
  pub fn get_uri_hash(&self) -> &str {
    todo!()
  }
  pub fn get_port(&self) -> u64 {
    todo!()
  }
  pub fn get_path_as_str(&self) -> &str {
    todo!();
  }

  pub fn get_hostname(&self) -> &str {
    todo!();
  }

  pub fn get_method(&self) -> Method {
    todo!();
  }
  
  pub fn get_header_value_as_str(&self, header_name: &str) -> Option<&str> {
    todo!()
  }
}

impl Headers {
  pub fn get_header_as_str(&self, header_name: &str) -> Option<&str> {
    todo!()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
  pub port: usize,
  pub href: String,
  pub scheme: Scheme
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scheme {
  Http,
  Https,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRes {

}

impl HttpRes {
  pub fn get_header_value_as_str(&self, header_name: &str) -> Option<&str> {
    todo!()
  }
}

pub enum Number {
  I8(i8),
  U8(u8),
  I16(i16),
  U16(u16),
  I32(i32),
  U32(u32),
  I64(i64),
  U64(u64),
  I128(i128),
  U128(u128),
  ISize(isize),
  USize(usize),
}