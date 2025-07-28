// /// Allow the user to specify conditions for when to block access
// /// to specified web domains.
// /// 
// /// Use a proxy server to achive that.
// /// 
// /// This method is non-intrusive: We don't read the user's encrypted web traffic.
// /// 
// /// This method is limited.

// // TODO: Allow the user to specify a limit for how many times they are allowed to access specified domains in a specified duraion.

// // use std::net::TcpListener;

// // use httparse::{Request, Status};
// // use uuid::Uuid;
// // use crate::GenericError;

// // struct Question {
// //   method: String,
// //   path: String,
// //   version: String,
// //   headers: Vec<(String, String)>,
// // }

// // fn parse_question(question_as_bytes: &[u8]) -> Result<Option<Question>, GenericError> {
// //   // Prepare header storage
// //   let mut headers = [httparse::EMPTY_HEADER; 64];
// //   let mut parser = Request::new(&mut headers);
  
// //   // Parse request
// //   match parser.parse(question_as_bytes) {
// //     Ok(Status::Complete(_)) => {
// //       // Extract required fields
// //       let method = parser
// //         .method
// //         .map(|method| method.to_string())
// //         .ok_or_else(||
// //           GenericError::new("parse question")
// //             .add_error("parser returned 'completed' status but 'method' is none")
// //         )?;

// //       let path = parser
// //         .path
// //         .map(|path| path.to_string())
// //         .ok_or_else(||
// //           GenericError::new("parse question")
// //             .add_error("parser returned 'completed' status but 'path' is none")
// //         )?;
      
// //       let version = match parser.version {
// //         Some(0) => "HTTP/1.0".to_string(),
// //         Some(1) => "HTTP/1.1".to_string(),
// //         Some(2) => "HTTP/2.0".to_string(),
// //         Some(3) => "HTTP/3.0".to_string(),
// //         Some(minor_version) => return Err(
// //           GenericError::new("parsing question")
// //             .add_error("unknown http minor version")
// //             .add_attachment("http minor version", minor_version)
// //         ),
// //         None => return Err(
// //           GenericError::new("parsing question")
// //             .add_error("missing http minor version")
// //         )
// //       };
      
// //       // Collect headers
// //       let headers = parser.headers
// //         .iter()
// //         .map(|header| (
// //           header.name.to_string(),
// //           String::from_utf8_lossy(header.value).to_string()
// //         ))
// //         .collect();
      
// //       Ok(Some(Question {
// //         method,
// //         path,
// //         version,
// //         headers,
// //       }))
// //     }

// //     Ok(Status::Partial) => {
// //       Ok(None)
// //     },

// //     Err(error) => {
// //       Err(
// //         GenericError::new("parsing question")
// //           .add_error("malformed question")
// //           .add_attachment("error", error.to_string())
// //       )
// //     },
// //   }
// // }

// // pub struct ProxyServer {
// //   port: u32,
// // }

// // impl ProxyServer {
// //   pub fn new(port: u32) -> Result<Self, GenericError> {
// //     let listener = TcpListener::bind(format!("localhost:{port}"))
// //       .map_err(|error|
// //         GenericError::new("openining a proxy server")
// //           .add_error("bind tcp listener")
// //           .add_attachment("port", port.to_string())
// //       )?;

    
// //   }
// // }

// // pub enum RuleActivator {

// // }

// // pub struct Rule {
// //   id: Uuid,
// //   activator: RuleActivator,
// // }

// // pub struct PolicyName(String);


// // pub struct UserWebDomainsAccessRegulator {
  
// // }

// // // Example usage
// // fn main() -> Result<(), &'static str> {
// //   let request_bytes = b"\
// //     GET /api/data?user=123 HTTP/1.1\r\n\
// //     Host: example.com\r\n\
// //     Accept: application/json\r\n\
// //     \r\n";
  
// //   let parsed = parse_question(request_bytes)?;
  
// //   // Now you have structured access to:
// //   // parsed.method, parsed.path, parsed.version, parsed.headers
// //   Ok(())
// // }

// use std::{io::{Read, Write}, net::{Shutdown, TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::spawn};
// use std::io;
// use std::thread;
// use httparse::{Request, Status};
// use http::{Method, Uri};
// use http::uri::Authority;
// use crate::{GenericError, DaemonMutex};
// use llhttp_rs;

// pub struct State {
//   uri_buffer: Vec<u8>,
//   did_complete_parsing_uri: bool,
//   header_buffer: Vec<u8>,
//   is_parsing_host_header: bool,
//   have_not_completed_parsing_host_header_yet: bool,
//   method: Option<Method>
// }

// impl llhttp_rs::Callbacks for State {
//   fn on_url(&mut self, _parser: &mut llhttp_rs::Parser, data: &[u8]) -> llhttp_rs::ParserResult<()> {
//     self.uri_buffer.extend_from_slice(data);
//     Ok(())
//   }

//   fn on_url_complete(&mut self, _parser: &mut llhttp_rs::Parser) -> llhttp_rs::ParserResult<()> {
//     self.did_complete_parsing_uri = true;
//     Ok(())
//   }

//   fn on_header_field(&mut self, _parser: &mut llhttp_rs::Parser, data: &[u8]) -> llhttp_rs::ParserResult<()> {
//     if self.have_not_completed_parsing_host_header_yet {
//       self.header_buffer.extend_from_slice(data);
//     }
    
//     Ok(())
//   }

//   fn on_header_field_complete(&mut self, _parser: &mut llhttp_rs::Parser) -> llhttp_rs::ParserResult<()> {
//     if 
//       self.have_not_completed_parsing_host_header_yet 
//       &&
//       self.header_buffer == b"host"
//     {
//       self.is_parsing_host_header = true;
//       self.header_buffer.clear();
//     }
    
//     Ok(())
//   }

//   fn on_header_value(&mut self, _parser: &mut llhttp_rs::Parser, data: &[u8]) -> llhttp_rs::ParserResult<()> {
//     if self.is_parsing_host_header {
//       self.header_buffer.extend_from_slice(data);
//     }

//     Ok(())
//   }

//   fn on_header_value_complete(&mut self, _parser: &mut llhttp_rs::Parser) -> llhttp_rs::ParserResult<()> {
//     if self.is_parsing_host_header {
//       self.have_not_completed_parsing_host_header_yet = false;
//     }

//     Ok(())
//   }

//   fn on_method_complete(&mut self, parser: &mut llhttp_rs::Parser) -> llhttp_rs::ParserResult<()> {
//     self.method = parser.get_method();
//     Ok(())
//   }
// }

// struct HttpRequestHead {
//   method: String,
//   path: String,
//   // version: HttpVersion,
//   headers: Vec<Header>,
// }

// pub struct Header {
//   name: String,
//   value: String,
// }

// // enum HttpVersion {
// //   Version0_9,
// //   Version1_0,
// //   Version1_1,
// // }

// fn parse_req(request_as_bytes: &[u8]) -> Result<Option<(usize, HttpRequestHead)>, GenericError> {
//   let mut headers = [httparse::EMPTY_HEADER; 64];
//   let mut parser = Request::new(&mut headers);
  
//   // Parse request
//   match parser.parse(request_as_bytes) {
//     Ok(Status::Complete(body_start_index)) => {
//       // Extract required fields
//       let method = parser
//         .method
//         .ok_or_else(|| 
//           GenericError::new("parsing http request head")
//             .add_error("missing http request method after parsing is complete")
//         )
//         .map(|method|
//           method.to_uppercase()
//         )?;

//       let path = parser
//         .path
//         .ok_or_else(||
//           GenericError::new("parsing http request head")
//             .add_error("missing http request path after parsing is complete")
//         )?;
      
//       // Convert version number to string representation
//       // let version = match parser.version {
//       //   Some(9) => {
//       //     HttpVersion::Version0_9
//       //   }
//       //   Some(0) => {
//       //     HttpVersion::Version1_0
//       //   }
//       //   Some(1) => {
//       //     HttpVersion::Version1_1
//       //   }
//       //   Some(minor) => {
//       //     return Err(
//       //       GenericError::new("parsing http request head")
//       //         .add_error("unrecoginzed http minor version number")
//       //         .add_attachment("http minor version number", minor.to_string())
//       //     )
//       //   }
//       //   None => {
//       //     return Err(
//       //       GenericError::new("parsing http request head")
//       //         .add_error("missing http minor version number after parsing is complete")
//       //     ) 
//       //   }
//       // };
      
//       let headers = parser.headers
//         .iter()
//         .map(|header| 
//           Header {
//             name: header.name.to_string().to_lowercase(),
//             value: String::from_utf8_lossy(header.value).to_string()
//           }
//         )
//         .collect();
      
//       Ok(Some((
//         body_start_index, 
//         HttpRequestHead {
//           path: path.into(),
//           method: method.into(),
//           // version,
//           headers,
//         },
//       )))
//     }
    
//     Ok(Status::Partial) => {
//       Ok(None)
//     }

//     Err(_) => {
//       Err(
//         GenericError::new("parsing http request head")
//           .add_error("http request head is malformed")
//       )
//     }
//   }
// }

// pub fn run(daemon: DaemonMutex) -> Result<(), GenericError> {
//   let address = {
//     daemon.lock()?.proxy_server_address().clone()
//   };

//   let listener = TcpListener::bind(&address)
//     .map_err(|error|
//       GenericError::new("running a proxy server")
//         .add_error("failed to bind a tcp listener")
//         .add_attachment("address", address.to_string())
//         .add_attachment("io error", error.to_string())
//     )?;

//   loop {
//     let (incoming, _) = match listener.accept() {
//       Ok(value) => {
//         value
//       }
//       Err(error) => {
//         eprintln!("{:?}", 
//           GenericError::new("proxy server handling incoming connection")
//             .add_error("tcp listener error")
//             .add_attachment("address", address.to_string())
//             .add_attachment("error", error.to_string())
//             .change_context("action")
//         );
//         continue;
//       }
//     };

//     let daemon = daemon.clone();
//     spawn(move || {
//       if let Err(error) = intercept_tcp_connection(daemon, incoming) {
//         eprintln!(
//           "{:?}",
//           error.change_context("discipline proxy server intercepting a tcp connection")
//         );
//       }
//     });
//   }
// }

// fn intercept_tcp_connection(
//   daemon: DaemonMutex,
//   mut upstream: TcpStream,
// ) -> 
//   Result<(), GenericError>
// {
//   let mut request_buffer = Vec::new();
//   let mut request_buffer_index = 0;

//   loop {
//     let number_of_read_bytes = match upstream
//       .read(&mut request_buffer[request_buffer_index..]) 
//     {
//       Ok(value) => {
//         value
//       }
//       Err(error) => {
//         return Err(
//           GenericError::new("intercepting tcp connection")
//             .add_error("failed to read data from upstream")
//             .add_attachment("io error", error.to_string())
//             // TODO: Add the data read so far
//         );
//       }
//     };

//     request_buffer_index += number_of_read_bytes;
//     match parse_req(&request_buffer[..request_buffer_index + 1]) {
//       Ok(None) => {
//         continue;
//       }
//       Err(error) => {
//         return Err(
//           error
//             .change_context("parsing data coming from upstream as http request")
//             .add_error("data coming from upstream is either not an http request or is a malformed http request")
//             .change_context("intercepting tcp connection")
//             // TODO: Add data read so far
//         );
//       }
//       Ok(Some((request_buffer_body_start_index, request_head))) => {
//         return intercept_http_request(
//           daemon,
//           upstream,
//           request_buffer,
//           request_buffer_body_start_index,
//           request_head,
//         ).map_err(|error|
//           error.change_context("intercepting tcp connection")
//         )
//       }
//     }
//   }
// }

// fn get_http_request_destination_host(request_head: &HttpRequestHead) -> Result<(String, u16), GenericError> {
//   let uri = request_head.path.parse::<Uri>().map_err(|error|
//     GenericError::new("getting the destination host of an http request")
//       .add_error("failed to parse request uri")
//       .add_attachment("uri", &request_head.path)
//       .add_attachment("error", error.to_string())
//   )?;

//   if let Some(authority) = uri.authority() {
//     return Ok((
//       authority.host().to_string(),
//       authority.port().map(|port| port.as_u16()).unwrap_or(80)
//     ));
//   }

//   let Some(host) = request_head
//     .headers
//     .iter()
//     .find(|header| header.name == "host") else 
//   {
//     return Err(
//       GenericError::new("getting the destination host of an http request")
//         .add_error("destination host is not specified in the url and the request doesn't have a 'host' header")
//     )
//   };

//   let host = host.value.parse::<Authority>().map_err(|error|
//     GenericError::new("getting the destination host of an http request")
//       .add_error("destination host is not specified in the url and the 'host' header is malformed")
//       .add_attachment("error", error.to_string())
//   )?;

//   return Ok((
//     host.host().to_string(),
//     host.port().map(|port| port.as_u16()).unwrap_or(80)
//   ));
// }

// fn bidirectional_copy(stream1: TcpStream, stream2: TcpStream) -> Result<(), GenericError> {
//   let upstream_mutex = Arc::new(Mutex::new(stream1));
//   let downstream_mutex = Arc::new(Mutex::new(stream2));

//   let upstream_mutex_clone = Arc::clone(&upstream_mutex);
//   let downstream_mutex_clone = Arc::clone(&downstream_mutex);
//   let upstream_to_downstream_thread = thread::spawn(move || {
//     let mut buffer = [0u8; 4096];
//     loop {
//       let number_of_read_bytes = match upstream_mutex_clone
//         .lock()
//         .unwrap()
//         .read(&mut buffer) 
//       {
//         Ok(0) => {
//           break;
//         }
//         Ok(value) => {
//           value
//         }
//         Err(_) => {
//           break;
//         }
//       };

//       if let Err(_) = downstream_mutex_clone
//         .lock()
//         .unwrap()
//         .write_all(&buffer[..number_of_read_bytes]) 
//       {
//         break;
//       }
//     }
//   });
  

//   let upstream_mutex_clone = Arc::clone(&upstream_mutex);
//   let downstream_mutex_clone = Arc::clone(&downstream_mutex);
//   let downstream_to_upstream_thread = thread::spawn(move || {
//     let mut buffer = [0u8; 4096];
//     loop {
//       let number_of_read_bytes = match downstream_mutex_clone
//         .lock()
//         .unwrap()
//         .read(&mut buffer) 
//       {
//         Ok(0) => {
//           break;
//         }
//         Ok(value) => {
//           value
//         }
//         Err(_) => {
//           break;
//         }
//       };

//       if let Err(_) = upstream_mutex_clone
//         .lock()
//         .unwrap()
//         .write_all(&buffer[..number_of_read_bytes]) 
//       {
//         break;
//       }
//     }
//   });

//   let _ = upstream_to_downstream_thread.join();
//   let _ = downstream_to_upstream_thread.join();

//   Ok(())
// }

// fn intercept_http_request(
//   daemon: DaemonMutex,
//   mut upstream: TcpStream,
//   request_buffer: Vec<u8>,
//   request_buffer_body_start_index: usize,
//   request_head: HttpRequestHead,
// ) -> 
//   Result<(), GenericError> 
// {
//   let (host, port) = get_http_request_destination_host(&request_head).map_err(|error|
//     error.change_context("intercepting http request")
//   )?;

//   if daemon.lock()?.is_hostname_in_block_list(&host) {
//     _ = upstream.write(BLOCKED_RESPONSE);
//     _ = upstream.shutdown(Shutdown::Both);
//     return Ok(());
//   }

//   let destination = format!("{host}:{port}");

//   let downstream = match TcpStream::connect(&destination) {
//     Ok(value) => {
//       value
//     }
//     Err(error) => {
//       let mut generic_error = GenericError::new("intercepting http request")
//         .add_error("failed to connect to downstream")
//         .add_attachment("downstream address", destination)
//         .add_attachment("connection error", error.to_string());

//       if let Err(io_error) = upstream.write(b"HTTP/1.1 502 Bad Gateway\r\n\r\n") {
//         generic_error = generic_error
//           .add_error("failed to send '502 Bad Gateway' to upstream")
//           .add_attachment("send '502 Bad Gateway' to upstream io error", io_error.to_string());
//       }

//       return Err(generic_error);
//     }
//   };

//   if request_head.method == "CONNECT" {
//     if let Err(io_error) = upstream.write(b"HTTP/1.1 200 Connection Established\r\n\r\n") {
//       let mut generic_error = GenericError::new("intercepting incoming http request");

//       generic_error = generic_error
//         .add_error("failed to write '200 Connection Established' to upstream")
//         .add_attachment("write '200 Connection Established' to upstream io error", io_error.to_string());

//       if let Err(io_error) = downstream.shutdown(Shutdown::Both) {
//         generic_error = generic_error
//           .add_error("failed to shutdown downstream after failing to send '200 Connection Established' to upstream")
//           .add_attachment("shutdown downstream after failing to send '200 Connection Established' to upstream io error", io_error.to_string());
//       }

//       return Err(generic_error);
//     }

//   }

//   bidirectional_copy(upstream, downstream)
// }

// static BLOCKED_RESPONSE: &[u8] = b"HTTP/1.1 403 Forbidden\r\n\
// Content-Type: text/plain; charset=utf-8\r\n\
// Content-Length: 57\r\n\
// Connection: close\r\n\
// \r\n\
// Access to the requested host has been blocked by the proxy.";


// block domains
// limit times the user is allowed to access domains
// delay domain access