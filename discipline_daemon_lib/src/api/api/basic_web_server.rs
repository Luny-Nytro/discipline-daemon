// TODO: limit the server request payload size
// TODO: limit the server request payload size

use std::collections::HashMap;
use std::net::SocketAddrV4;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;
use crate::GenericError;

pub struct WebServer {
  actual_server: tiny_http::Server,
}

impl WebServer {
  pub fn new<Handler>(
    address: SocketAddrV4, 
  ) 
    -> Result<Self, GenericError> 
  {
    Ok(Self {
      actual_server: tiny_http
        ::Server
        ::http(address)
        .map_err(|error|
          GenericError::new("starting an http server")
            .add_error("faild to start an http server on the specified address")
            .add_attachment("address", address.to_string())
            .add_attachment("error", error.to_string())
        )?,
    })
  }

  pub fn recieve(&self) -> Result<Request, RecieveError> {
    match self.actual_server.recv() {
      Ok(request) => {
        Request::wrap(request)
      }
      Err(network_error) => {
        Err(RecieveError::NetworkError { network_error })
      }
    }
  }
}

pub enum RecieveError {
  MalformedUrl {
    url: String, 
    parse_error: url::ParseError,
  },
  NetworkError {
    network_error: std::io::Error
  }
}

pub struct Request {
  req: tiny_http::Request,
  url: Url,
  // query_parameters: HashMap<Cow<'a, str>, Cow<'a, str>>
  query_parameters: HashMap<String, String>
}

impl Request {
  pub fn wrap(request: tiny_http::Request) -> Result<Self, RecieveError> {
    let url = match request.url().parse::<Url>() {
      Ok(url) => {
        url
      }
      Err(parse_error) => {
        return Err(RecieveError::MalformedUrl {
          url: request.url().to_owned(),
          parse_error
        });
      }
    };

    let query_parameters = url.query_pairs().into_owned().collect();

    Ok(Self {
      query_parameters,
      req: request,
      url,
    })
  }

  pub fn url(&self) -> &Url {
    &self.url
  }

  pub fn query_parameters(&self) -> &HashMap<String, String> {
    &self.query_parameters
  }

  pub fn deserialize_body<T>(&mut self) -> Result<T, DeserializeBodyError>
  where
    T: DeserializeOwned
  {
    deserialize_body(self)
  }
  
  fn respond_with_http_success<T: Serialize>(self, value: T) {
    let Ok(value) = serde_json::to_vec_pretty(&value) else {
      self.respond_with_http_internal_server_error();
      // TODO: Log error.
      return;
    };

    let value_length = value.len();
    let Ok(content_type_header) = tiny_http::Header::from_bytes(
      b"Content-Type", 
      b"application/json",
    ) else {
      self.respond_with_http_internal_server_error();
      return;
    };
    let Ok(content_length_header) = tiny_http::Header::from_bytes(
      b"Content-Length", 
      value_length.to_string().as_bytes(),
    ) else {
      self.respond_with_http_internal_server_error();
      return;
    };

    let response = tiny_http::Response::from_data(value)
      .with_status_code(200)
      .with_header(content_type_header)
      .with_header(content_length_header);

    if let Err(error) = self.req.respond(response) {
      eprintln!("Discipline.Server.RespondWithData: {error}");
    }
  }

  pub fn respond_with_http_not_found(self) {
    if let Err(error) = self.req.respond(tiny_http::Response::empty(404)) {
      eprintln!("Discipline.Server.RespondWithNotFound: {error}");
    }
  }

  pub fn respond_with_http_internal_server_error(self) {
    if let Err(error) = self.req.respond(tiny_http::Response::empty(500)) {
      eprintln!("Discipline.Server.RespondWithInternalServerError: {error}");
    }
  }

  pub fn respond_with_http_bad_request(self) {
    if let Err(error) = self.req.respond(tiny_http::Response::empty(400)) {
      eprintln!("Discipline.Server.RespondWithBadRequest: {error}");
    }
  }

  pub fn respond_with_http_payload_too_large(self) {
    if let Err(error) = self.req.respond(tiny_http::Response::empty(413)) {
      eprintln!("Discipline.Server.RespondeWithPayloodTooLarge: {error}");
    }
  }
}

// 4 KB
const MAXIMUM_HTTP_REQUEST_BODY_SIZE: usize = 4096;

pub enum ReadBodyError {
  NetworkError { network_error: std::io::Error },
  BodyTooLarge { maximum_body_length: usize, }
}

fn read_body(request: &mut Request) -> Result<Vec<u8>, ReadBodyError> {
  // Read in 0.5 KB chunks.
  let mut temp_buffer = vec![0u8; 512];
  
  // May hold a maximum of 4 KB worth of data.
  let mut body: Vec<u8> = Vec::new();
  
  // Total number of bytes read into `body`.
  let mut body_size = 0;

  let reader = request.req.as_reader();

  loop {
    let chunk_size = match reader.read(&mut temp_buffer) {
      Ok(chunk_size) => {
        chunk_size
      }
      Err(network_error) => {
        return Err(ReadBodyError::NetworkError { network_error });
      }
    };

    // TODO: Should we check for Content-Length or other 
    // methods to figure out the expected body length and
    // fail if the actual body length differs?
    if chunk_size == 0 {
      return Ok(body);
    }

    body_size += chunk_size;
    if body_size > MAXIMUM_HTTP_REQUEST_BODY_SIZE {
      return Err(ReadBodyError::BodyTooLarge { 
        maximum_body_length: MAXIMUM_HTTP_REQUEST_BODY_SIZE,
      });
    }

    body.extend_from_slice(&temp_buffer[..chunk_size]);
  }
}

pub enum DeserializeBodyError {
  ReadError(ReadBodyError),
  DeserializeError(serde_json::Error)
}

fn deserialize_body<T: DeserializeOwned>(request: &mut Request) -> Result<T, DeserializeBodyError> {
  let body = match read_body(request) {
    Ok(body) => {
      body
    }
    Err(error) => {
      return Err(DeserializeBodyError::ReadError(error));
    }
  };

  let body = match serde_json::from_slice(&body) {
    Ok(body) => {
      body
    }
    Err(error) => {
      return Err(DeserializeBodyError::DeserializeError(error));
    }
  };

  Ok(body)
}