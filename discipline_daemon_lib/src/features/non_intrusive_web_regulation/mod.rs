/// Allow the user to specify conditions for when to block access
/// to specified web domains.
/// 
/// Use a proxy server to achive that.
/// 
/// This method is non-intrusive: We don't read the user's encrypted web traffic.
/// 
/// This method is limited.

// use std::net::TcpListener;

// use httparse::{Request, Status};
// use uuid::Uuid;
// use crate::GenericError;

// struct Question {
//   method: String,
//   path: String,
//   version: String,
//   headers: Vec<(String, String)>,
// }

// fn parse_question(question_as_bytes: &[u8]) -> Result<Option<Question>, GenericError> {
//   // Prepare header storage
//   let mut headers = [httparse::EMPTY_HEADER; 64];
//   let mut parser = Request::new(&mut headers);
  
//   // Parse request
//   match parser.parse(question_as_bytes) {
//     Ok(Status::Complete(_)) => {
//       // Extract required fields
//       let method = parser
//         .method
//         .map(|method| method.to_string())
//         .ok_or_else(||
//           GenericError::new("parse question")
//             .add_error("parser returned 'completed' status but 'method' is none")
//         )?;

//       let path = parser
//         .path
//         .map(|path| path.to_string())
//         .ok_or_else(||
//           GenericError::new("parse question")
//             .add_error("parser returned 'completed' status but 'path' is none")
//         )?;
      
//       let version = match parser.version {
//         Some(0) => "HTTP/1.0".to_string(),
//         Some(1) => "HTTP/1.1".to_string(),
//         Some(2) => "HTTP/2.0".to_string(),
//         Some(3) => "HTTP/3.0".to_string(),
//         Some(minor_version) => return Err(
//           GenericError::new("parsing question")
//             .add_error("unknown http minor version")
//             .add_attachment("http minor version", minor_version)
//         ),
//         None => return Err(
//           GenericError::new("parsing question")
//             .add_error("missing http minor version")
//         )
//       };
      
//       // Collect headers
//       let headers = parser.headers
//         .iter()
//         .map(|header| (
//           header.name.to_string(),
//           String::from_utf8_lossy(header.value).to_string()
//         ))
//         .collect();
      
//       Ok(Some(Question {
//         method,
//         path,
//         version,
//         headers,
//       }))
//     }

//     Ok(Status::Partial) => {
//       Ok(None)
//     },

//     Err(error) => {
//       Err(
//         GenericError::new("parsing question")
//           .add_error("malformed question")
//           .add_attachment("error", error.to_string())
//       )
//     },
//   }
// }

// pub struct ProxyServer {
//   port: u32,
// }

// impl ProxyServer {
//   pub fn new(port: u32) -> Result<Self, GenericError> {
//     let listener = TcpListener::bind(format!("localhost:{port}"))
//       .map_err(|error|
//         GenericError::new("openining a proxy server")
//           .add_error("bind tcp listener")
//           .add_attachment("port", port.to_string())
//       )?;

    
//   }
// }

// pub enum RuleActivator {

// }

// pub struct Rule {
//   id: Uuid,
//   activator: RuleActivator,
// }

// pub struct PolicyName(String);


// pub struct UserWebDomainsAccessRegulator {
  
// }

// // Example usage
// fn main() -> Result<(), &'static str> {
//   let request_bytes = b"\
//     GET /api/data?user=123 HTTP/1.1\r\n\
//     Host: example.com\r\n\
//     Accept: application/json\r\n\
//     \r\n";
  
//   let parsed = parse_question(request_bytes)?;
  
//   // Now you have structured access to:
//   // parsed.method, parsed.path, parsed.version, parsed.headers
//   Ok(())
// }