use std::{io::{Read, Write}, net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream}, sync::{Arc, Mutex}, thread::spawn};
use std::io;
use std::thread;
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

fn parse_question(request_bytes: &[u8]) -> Result<Option<HttpRequestHead>, GenericError> {
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
        )
        .map(|method|
          method.to_uppercase()
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
            name: header.name.to_string().to_lowercase(),
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

    let daemon = daemon.clone();
    spawn(move || {
      handle_incomming_tcp_connection(daemon, incoming);
    });
  }
}

fn handle_incomming_tcp_connection(
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
          GenericError::new("intercepting incoming tcp connection")
            .add_error("failed to read data from upstream")
            .add_attachment("io error", error.to_string())
            // TODO: Add the data read so far
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
            .change_context("intercepting incoming tcp connection")
            .add_error("incoming http request is malformed")
        );
      }
      Ok(Some(request)) => {
        let destination = get_http_request_destination_host(&request)
          .map_err(|error| error.change_context("intercepting incoming http request"))?;

        let downstream = match TcpStream::connect(&destination) {
          Ok(value) => {
            value
          }
          Err(error) => {
            let mut generic_error = GenericError::new("intercepting incoming http request")
              .add_error("failed to connect to downstream")
              .add_attachment("downstream address", destination)
              .add_attachment("connect to downstream io error", error.to_string());

            if let Err(io_error) = upstream.write(b"HTTP/1.1 502 Bad Gateway\r\n\r\n") {
              generic_error = generic_error
                .add_error("failed to send '502 Bad Gateway' to upstream")
                .add_attachment("send '502 Bad Gateway' to upstream io error", io_error.to_string());
            }

            return Err(generic_error);
          }
        };

        if request.method == "CONNECT" {
          if let Err(io_error) = upstream.write(b"HTTP/1.1 200 Connection Established\r\n\r\n") {
            let mut generic_error = GenericError::new("intercepting incoming http request");

            generic_error = generic_error
              .add_error("failed to write '200 Connection Established' to upstream")
              .add_attachment("write '200 Connection Established' to upstream io error", io_error.to_string());

            if let Err(io_error) = downstream.shutdown(Shutdown::Both) {
              generic_error = generic_error
                .add_error("failed to shutdown downstream after failing to send '200 Connection Established' to upstream")
                .add_attachment("shutdown downstream after failing to send '200 Connection Established' to upstream io error", io_error.to_string());
            }

            return Err(generic_error);
          }

        }
        
        bidirectional_copy(upstream, downstream);
        return Ok(());
      }
    }
  }
}

fn get_http_request_destination_host(request: &HttpRequestHead) -> Result<String, GenericError> {
  let uri = request.path.parse::<Uri>().map_err(|error|
    GenericError::new("intercepting incoming http request")
      .add_error("failed to parse request uri")
      .add_attachment("error", error.to_string())
  )?;

  if let Some(authority) = uri.authority() {
    return Ok(format!(
      "{}:{}", 
      authority.host(),
      authority.port().map(|port| port.as_u16()).unwrap_or(80)
    ));
  }

  let Some(host) = request
    .headers
    .iter()
    .find(|header| header.name == "host") else 
  {
    return Err(
      GenericError::new("getting the destination host for an http request")
        .add_error("request destination host is not specified in the url and the request doesn't have a 'host' header")
    )
  };

  let host = host.value.parse::<Authority>().map_err(|error|
    GenericError::new("getting the destination host for an http request")
      .add_error("request destination host is not specified in the url and the 'host' header is malformed")
      .add_error("")
      .add_attachment("error", error.to_string())
  )?;

  return Ok(format!(
    "{}:{}", 
    host.host(),
    host.port().map(|port| port.as_u16()).unwrap_or(80)
  ));
}

fn intercept_incoming_http_request() {

}


fn bidirectional_copy(stream1: TcpStream, stream2: TcpStream) -> io::Result<()> {
  let stream1 = Arc::new(Mutex::new(stream1));
  let stream2 = Arc::new(Mutex::new(stream2));

  let x1 = Arc::clone(&stream1);
  let x2 = Arc::clone(&stream2);
  let t1 = thread::spawn(move || {
    let mut buffer = [0u8; 4096];
    loop {
      let number_of_read_bytes = match x1.lock().unwrap().read(&mut buffer) {
        Ok(0) => {
          break;
        }
        Ok(value) => {
          value
        }
        Err(_) => {
          break;
        }
      };

      if let Err(_) = x2.lock().unwrap().write_all(&buffer[..number_of_read_bytes]) {
        break;
      }
    }
  });
  

  let x1 = Arc::clone(&stream1);
  let x2 = Arc::clone(&stream2);
  let t2 = thread::spawn(move || {
    let mut buffer = [0u8; 4096];
    loop {
      let number_of_read_bytes = match x2.lock().unwrap().read(&mut buffer) {
        Ok(0) => {
          break;
        }
        Ok(value) => {
          value
        }
        Err(_) => {
          break;
        }
      };

      if let Err(_) = x1.lock().unwrap().write_all(&buffer[..number_of_read_bytes]) {
        break;
      }
    }
  });

  let _ = t1.join();
  let _ = t2.join();

  Ok(())
}
