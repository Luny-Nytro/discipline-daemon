use std::{collections::HashSet, io::Read, net::{IpAddr, Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::spawn};

use httparse::{Request, Status};
use http::Uri;
use http::uri::Authority;
use crate::{GenericError, DaemonMutex};

struct HttpRequestHead {
  method: String,
  path: String,
  version: HttpVersion,
  headers: Vec<Header>,
}

pub struct Header {
  name: String,
  value: String,
}

enum HttpVersion {
  Version0_9,
  Version1_0,
  Version1_1,
}

/// Parses HTTP request head from bytes
fn parse_question(request_bytes: &[u8]) -> Result<Option<HttpRequestHead>, GenericError> {
  // Prepare header storage
  let mut headers = [httparse::EMPTY_HEADER; 64];
  let mut parser = Request::new(&mut headers);
  
  // Parse request
  match parser.parse(request_bytes) {
    Ok(Status::Complete(_)) => {
      // Extract required fields
      let method = parser
        .method
        .ok_or_else(|| 
          GenericError::new("parsing http request head")
            .add_error("missing http request method after parsing is complete")
        )?;

      let path = parser
        .path
        .ok_or_else(||
          GenericError::new("parsing http request head")
            .add_error("missing http request path after parsing is complete")
        )?;
      
      // Convert version number to string representation
      let version = match parser.version {
        Some(9) => {
          HttpVersion::Version0_9
        }
        Some(0) => {
          HttpVersion::Version1_0
        }
        Some(1) => {
          HttpVersion::Version1_1
        }
        Some(minor) => {
          return Err(
            GenericError::new("parsing http request head")
              .add_error("unrecoginzed http minor version number")
              .add_attachment("http minor version number", minor.to_string())
          )
        }
        None => {
          return Err(
            GenericError::new("parsing http request head")
              .add_error("missing http minor version number after parsing is complete")
          ) 
        }
      };
      
      // Collect headers
      let headers = parser.headers
        .iter()
        .map(|header| 
          Header {
            name: header.name.to_string(),
            value: String::from_utf8_lossy(header.value).to_string()
          }
        )
        .collect();
      
      Ok(Some(HttpRequestHead {
        path: path.into(),
        method: method.into(),
        version,
        headers,
      }))
    }
    
    Ok(Status::Partial) => {
      Ok(None)
    }

    Err(_) => {
      Err(
        GenericError::new("parsing http request head")
          .add_error("http request head is malformed")
      )
    }
  }
}


pub fn run(daemon: DaemonMutex, port: u16) -> Result<(), GenericError> {
  let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);

  let listener = TcpListener::bind(address)
    .map_err(|error|
      GenericError::new("running a proxy server")
        .add_error("failed to bind a tcp listener")
        .add_attachment("address", address.to_string())
        .add_attachment("io error", error.to_string())
    )?;

  loop {
    let (incoming, _) = match listener.accept() {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("{:?}", 
          GenericError::new("proxy server handling incoming connection")
            .add_error("tcp listener error")
            .add_attachment("address", address.to_string())
            .add_attachment("error", error.to_string())
            .change_context("action")
        );
        continue;
      }
    };

    spawn(move || {
      handle_incomming_connection(
        daemon.clone(),
        incoming,
      );
    });
  }
}

fn handle_incomming_connection(
  daemon: DaemonMutex,
  mut upstream: TcpStream,
) -> 
  Result<(), GenericError>
{
  let mut buffer = Vec::new();
  let mut index = 0;

  loop {
    let number_of_read_bytes = match upstream.read(&mut buffer[index..]) {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(
          GenericError::new("proxifying connection")
            .add_error("failed to read from upstream")
            .add_attachment("io error", error.to_string())
        );
      }
    };

    index += number_of_read_bytes;
    match parse_question(&buffer[..index + 1]) {
      Ok(None) => {
        continue;
      }
      Err(error) => {
        return Err(
          error
            .change_context("proxifying connection")
            .add_error("incoming http request is malformed")
        );
      }
      Ok(Some(question)) => {
        let uri = question.path.parse::<Uri>().map_err(|error|
          GenericError::new("proxifying incoming http request")
            .add_error("failed to parse request uri")
            .add_attachment("error", error.to_string())
        )?;

        let host = question.headers.iter().find(|header| header.name == "host");
        let host = if let Some(header) = host {
          header.value.parse::<Authority>().map_err(|error|
            GenericError::new("proxifying incoming http request")
              .add_error("host header is malformed")
              .add_attachment("error", error.to_string())
          )
        };

          // .ok_or_else(||
          //   GenericError::new("proxifying incoming http request")
          //     .add_error("error_message")
          // )?;

        let authority = host.value.parse::<Authority>().map_err(|error|
          GenericError::new("action")
        )?;

        let downstream = TcpStream::connect("addr").unwrap();
        return Ok(());
      }
    }
  }
}