use crate::GenericError;
use serde::{de::DeserializeOwned, Serialize};
use tiny_http::{Header, Request, Response};
use std::{net::SocketAddrV4, thread::{spawn, JoinHandle}};

// 4 KB
const MAXIMUM_REQUEST_BODY_SIZE: usize = 4096;

pub enum ApiReturn {
  Success(String),
  InternalError,
  MalformedOperation,
  UnknownOperation,
}

pub enum ResponseCreator {
  Json(Vec<u8>),
  BadRequest,
  PayloadTooLarge,
  InternalServerError,
  NotFound,
}

impl ResponseCreator {
  fn json<T>(value: T) -> ResponseCreator
  where
    T: Serialize,
  {
    match serde_json::to_vec_pretty(&value) {
      Ok(value) => ResponseCreator::Json(value),
      Err(error) => {
        eprintln!("Discipline.Server.Respond.SerializeOutgoingOperationOutcome: {error}");
        ResponseCreator::InternalServerError
      }
    }
  }
}

fn respond_with_not_found(request: Request) {
  if let Err(error) = request.respond(Response::empty(404)) {
    eprintln!("Discipline.Server.RespondWithNotFound: {error}");
  }
}

fn respond_with_payload_too_large(request: Request) {
  if let Err(error) = request.respond(Response::empty(413)) {
    eprintln!("Discipline.Server.RespondeWithPayloodTooLarge: {error}");
  }
}

fn respond_with_bad_request(request: Request) {
  if let Err(error) = request.respond(Response::empty(400)) {
    eprintln!("Discipline.Server.RespondWithBadRequest: {error}");
  }
}

fn respond_with_internal_server_error(request: Request) {
  if let Err(error) = request.respond(Response::empty(500)) {
    eprintln!("Discipline.Server.RespondWithInternalServerError: {error}");
  }
}

fn respond_with_json(request: Request, data: Vec<u8>) {
  let data_length = data.len();
  let response = Response::from_data(data)
    .with_status_code(200)
    .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap())
    .with_header(
      Header::from_bytes(b"Content-Length", data_length.to_string().as_bytes()).unwrap(),
    );

  if let Err(error) = request.respond(response) {
    eprintln!("Discipline.Server.RespondWithData: {error}");
  }
}

fn respond_with(request: Request, response_creator: ResponseCreator) {
  match response_creator {
    ResponseCreator::BadRequest => {
      respond_with_bad_request(request);
    }
    ResponseCreator::InternalServerError => {
      respond_with_internal_server_error(request);
    }
    ResponseCreator::Json(json) => {
      respond_with_json(request, json);
    }
    ResponseCreator::NotFound => {
      respond_with_not_found(request);
    }
    ResponseCreator::PayloadTooLarge => {
      respond_with_payload_too_large(request);
    }
  }
}

fn deserialize_body_as<T>(request: &mut Request) -> Result<T, ResponseCreator>
where
  T: DeserializeOwned,
{
  // Read in 0.5 KB chunks.
  let mut buffer = vec![0u8; 512];
  // May hold a maximum of 4 KB worth of data.
  let mut payload: Vec<u8> = Vec::new();
  // Total number of bytes read into `payload`.
  let mut body_size = 0;
  let payload_reader = request.as_reader();

  loop {
    let chunk_size = match payload_reader.read(&mut buffer) {
      Ok(chunk_size) => {
        chunk_size
      }
      Err(error) => {
        eprintln!("Discipline.Server.Respond.ReadIncomingPayload: {error}");
        return Err(ResponseCreator::InternalServerError);
      }
    };

    if chunk_size == 0 {
      break;
    }

    body_size += chunk_size;
    if body_size > MAXIMUM_REQUEST_BODY_SIZE {
      eprintln!("Discipline.Server.Respond.QuestionPayloadTooLarge.");
      return Err(ResponseCreator::PayloadTooLarge);
    }

    payload.extend_from_slice(&buffer[..chunk_size]);
  }

  match serde_json::from_slice(&payload) {
    Ok(value) => {
      Ok(value)
    }
    Err(error) => {
      if let Ok(payload) = String::from_utf8(payload) {
        eprintln!("Discipline.Server.Respond.DeserializeIncomingOperation: \nError: {error}.\nPayload: {payload}.");
      } else {
        eprintln!("Discipline.Server.Respond.DeserializeIncomingOperation: \nError: {error}");
      }
      Err(ResponseCreator::BadRequest)
    }
  }
}

pub struct IncomingOperation<'a> {
  request: &'a mut Request
}

impl<'a> IncomingOperation<'a> {
  pub fn try_as<T: DeserializeOwned>(self) -> Result<T, ServerError> {
    match deserialize_body_as(self.request) {
      Ok(value) => {
        Ok(value)
      }
      Err(error) => {
        Err(ServerError {  })
      }
    }
  }
}

pub struct ServerError {

}

pub struct ServerReturn {

}


pub struct BasicHttpServer {
  server_thread: Option<JoinHandle<()>>,
}

impl BasicHttpServer {
  pub fn new<T>(
    address: &SocketAddrV4,
    handler: T
  ) -> Result<Self, GenericError>
  where
    T: Send + 'static + Fn(String, IncomingOperation) -> Result<ServerReturn, ServerError>
  {
    let server = tiny_http::Server::http(address).map_err(|error|
      GenericError::new("running an http server")
        .add_attachment("address", address.to_string())
    )?;

    let thread = spawn(move || {
      // TODO: Make sure "incoming_requests()" doesn't panic.
      for mut request in server.incoming_requests() {
        let uri = request.url().to_owned();

        let incoming = IncomingOperation {
          request: &mut request,
        };

        let n = match handler(uri, incoming) {
          Ok(x) => {
            x
          }
          Err(error) => {
            // TODO: Do something about the error
            continue;
          }
        };

        match
      }
    });

    Ok(Self {
      server_thread: Some(thread),
    })
  }
}

macro_rules! run_basic_http_server {
  ($daemon:expr, $code:block) => {
    BasicHttpServer {
      server_thread: Some(spawn(move || {
        find_operation_type!(
        request.url(), 
        |Operation| {
          match request.body_as::<Operation>() {
            Ok(operation) => ResponseCreator::json(operation.execute(daemon)),
            Err(response) => response
          }
        }
        else {
          ResponseCreator::NotFound
        }
      )
      }))
    }
  };
}

pub fn server_no_such_operation() -> ServerReturn {
  todo!()
}
pub fn server_return<T: Serialize>(operation_return: T) -> Result<ServerReturn, ServerError> {
  todo!()
}