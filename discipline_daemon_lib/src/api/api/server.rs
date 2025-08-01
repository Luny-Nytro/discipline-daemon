use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use crate::{Daemon, GenericError, find_operation_type};
use super::*;

// TODO: limit the server request payload size
// TODO: limit the server request payload size

use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};
use tiny_http::{Header, Request, Response};
use url::Url;
use std::thread::{spawn, JoinHandle};

// 4 KB
const MAXIMUM_HTTP_REQUEST_BODY_SIZE: usize = 4096;

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

fn deserialize_body_as_2<T: serde::de::DeserializeOwned>(request: &mut Request) -> Result<T, GenericError>{
  serde_json::de::from_reader(request.as_reader())
    .map_err(|error|
      GenericError::new("")
    )
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
    if body_size > MAXIMUM_HTTP_REQUEST_BODY_SIZE {
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

pub struct Api {
  server_thread: Option<JoinHandle<()>>,
  operation_specifications: Arc<OperationSpecifications>
}

impl Api {
  pub fn new(daemon: Arc<Daemon>) -> Result<Self, GenericError> {
    let address = SocketAddrV4::new(
      Ipv4Addr::LOCALHOST, 
      daemon.configuration().api_tcp_port(),
    );

    let operation_specifications = Arc::new(OperationSpecifications {
      specifications: HashMap::new(),
    });
    
    let server = tiny_http::Server::http(address).map_err(|error|
      GenericError::new("running an http server")
        .add_attachment("address", address.to_string())
    )?;

    let thread = spawn(move || {
      // TODO: Make sure "incoming_requests()" doesn't panic.
      for mut request in server.incoming_requests() {
        let Ok(url) = request.url().parse::<Url>() else {
          // TODO: Add a message "malformed url"
          respond_with_bad_request(request);
          continue;
        };

        let query_pairs: HashMap<Cow<str>, Cow<str>> = url.query_pairs().collect();

        match url.path() {
          "/Api/ExecuteOperation" => {
            if query_pairs.len() != 1 {
              // TODO: Add error message "Expected exactly one query parameter"
              respond_with_bad_request(request);
              continue;
            }

            let Some(operation_id) = query_pairs.get("operation_id") else {
              // TODO: Add error message "Expected an 'operation_id' parameter"
              respond_with_bad_request(request);
              continue;
            };

            let Some(operation_specification) = operation_specifications
              .specifications
              .get(operation_id.as_ref()) else 
            {
              let operation_return = GenericOpertionReturn::unknown_operation();
              let Some(serialized_operation_return) = serialize_operation_return(operation_return) else {
                respond_with_internal_server_error(request);
                continue;
              };
            
              respond_with_json(request, serialized_operation_return);
              continue;
            };

            let serialized_operation = SerializedOperation { request: &mut request };
            let operation_retrun = operation_specification.execute(
              serialized_operation, 
              Arc::clone(&daemon),
            );
            let Some(serialized_operation_retrn) = ser

          }
          "/Api/RetrieveOperationSpecificationsAsHtml" => {

          }
          _ => {
            respond_with_not_found(request);
          }
        }


        let server_return = match handler(uri, incoming) {
          Ok(server_return) => {
            server_return
          }
          Err(error) => {
            respond_with_internal_server_error(request);
            eprintln!("Discipline.Api.BasicWebServer: Error: {:?}", error.generic_error);
            continue;
          }
        };

        match server_return {
          BasicWebServerReturn::NoSuchOperation => {
            respond_with_not_found(request);
          }
          BasicWebServerReturn::OperationReturn(data) => {
            respond_with_json(request, data);
          }
        }
      }
    });

    let server = BasicHttpServer::new(
      &address, 
      |id, operation| {
        find_operation_type! {
          id,
          |Operation| {
            Ok(BasicWebServerReturn::operation_return(
              operation
                .try_into::<Operation>()?
                .execute(Arc::clone(&daemon))
            ))
          }
          else {
            Ok(BasicWebServerReturn::no_such_operation())
          }
        }
      }
    )?;

    Ok(Self {
      server,
    })
  }
}

fn handle_server(server: tiny_http::Server) {
  // TODO: Make sure "incoming_requests()" doesn't panic.
  for mut request in server.incoming_requests() {
  }
}

fn handle_request(
  mut request: Request,
) {
  let Ok(url) = request.url().parse::<Url>() else {
    // TODO: Add a message "malformed url"
    respond_with_bad_request(request);
    return;
  };

  let query_pairs: HashMap<Cow<str>, Cow<str>> = url.query_pairs().collect();

  match url.path() {
    "/Api/ExecuteOperation" => {
      handle_execute_operation_request();
    }
    "/Api/RetrieveOperationSpecificationsAsHtml" => {
      handle_retrieve_operation_specifications_as_html();
    }
    _ => {
      respond_with_not_found(request);
    }
  }
}

fn handle_execute_operation_request(
  daemon: Arc<Daemon>,
  request: Request,
  query_pairs: &HashMap<Cow<str>, Cow<str>>,
  operation_specifications: &OperationSpecifications,
) {
  if query_pairs.len() != 1 {
    // TODO: Add error message "Expected exactly one query parameter"
    respond_with_bad_request(request);
    return;
  }

  let Some(operation_id) = query_pairs.get("operation_id") else {
    // TODO: Add error message "Expected an 'operation_id' parameter"
    respond_with_bad_request(request);
    return;
  };

  let Some(operation_specification) = operation_specifications
    .specifications
    .get(operation_id.as_ref()) else 
  {
    return match serialize_operation_return(GenericOpertionReturn::unknown_operation()) {
      None => {
        respond_with_internal_server_error(request);
      }
      Some(serialized_operation_return) => {
        respond_with_json(request, serialized_operation_return);
      }
    };
  };

  let serialized_operation = SerializedOperation { request: &mut request };
  let operation_retrun = operation_specification.execute(
    serialized_operation, 
    Arc::clone(&daemon),
  );
  let Some(serialized_operation_retrn) = ser

}

fn handle_retrieve_operation_specifications_as_html() {

}
pub struct OperationSpecifications {
  specifications: HashMap<String, Box<dyn IsOperationSpecification>>,
}

impl OperationSpecifications {
  pub fn add_operation_specification_or_panic(
    &mut self, 
    operation_specification: Box<dyn IsOperationSpecification>
  ) {
    if self.specifications.contains_key(operation_specification.human_readable_id()) {
      panic!(
        "Duplicate operation specification human readable id: {}", 
        operation_specification.human_readable_id(),
      );
    }
  }
}


pub struct SerializedOperation<'a> {
  request: &'a mut Request
}

pub fn deserialize_operation<T: DeserializeOwned>(
  serialized_operation: SerializedOperation,
) -> Option<T>
{
  todo!()
}

pub fn serialize_operation_return<T: Serialize>(operation_return: T) -> Option<Vec<u8>> {
  todo!()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericOpertionReturn {
  // InvalidInput,
  // CallReturn

  _moon: PhantomData<()>
}

impl GenericOpertionReturn {
  pub fn internal_error() -> Self {
    todo!()
  }

  pub fn unknown_operation() -> Self {
    todo!()
  }

  pub fn malformed_operation() -> Self {
    todo!()
  }

  pub fn operation_return(value: Vec<u8>) -> Self {
    todo!()
  }
}

pub trait IsOperationSpecification {
  fn human_readable_id(&self) -> &String;
  fn human_readable_description(&self) -> &String;
  fn execute(&self, request: Request, daemon: Arc<Daemon>);
}

macro_rules! implement_execute_operation {
  () => {
    
  };
}

pub struct ExecuteOperationContext {

}

impl ExecuteOperationContext {
  fn operation() {}
  fn operation_return() {}
}