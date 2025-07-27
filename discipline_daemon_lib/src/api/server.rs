use std::marker::PhantomData;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::rc::Rc;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use serde::{de::DeserializeOwned, Serialize};
use tiny_http::{Header, Method, Request, Response, Server};
use super::*;

pub struct Api {
  thread: Option<JoinHandle<()>>
}

impl Api {
  pub fn new(daemon: Arc<Daemon>) -> Result<Self, GenericError> {
    let address = SocketAddrV4::new(
      Ipv4Addr::LOCALHOST, 
      daemon.configuration().api_tcp_port(),
    );
    
    let server = BasicHttpServer::new(
      &address, 
      move |incoming| {
        find_operation_type! {
          incoming.id(),
          |Operation| {
            let operation = incoming.operation::<Operation>()?;
            let operation_return = operation.execute(&* daemon);
            Ok(operation_return)
          }
          else {
            ()
          }
        }
      }
    );

    // let server = match Server::http(&address) {
    //   Ok(server) => {
    //     server
    //   }
    //   Err(error) => {
    //     return Err(
    //       GenericError::new("Initializing discipline api")
    //         .add_error("failed to start api http server")
    //         .add_attachment("address", address.to_string())
    //         .add_attachment("error", error.to_string())
    //     )
    //   }
    // };

    let thread = spawn(move || {
      for mut operation in server.incoming_requests() {
        operation.respond_with(respond(
          Arc::clone(&daemon), 
          &mut operation,
        ));
      }
    });

    Ok(Self {
      thread: Some(thread)
    })
  }
}

fn respond(daemon: Rc<Daemon>, request: &mut Request) -> ResponseCreator {
  find_operation_type!(
    request.url(), 
    |Operation| {
      match question.body_as::<Operation>() {
        Ok(operation) => ResponseCreator::json(operation.execute(daemon)),
        Err(response) => response
      }
    }
    else {
      ResponseCreator::NotFound
    }
  )
}

pub struct BasicHttpServer<T> {
  __moon: PhantomData<T>
}


pub struct IncomingOperation {

}

impl IncomingOperation {
  pub fn id(&self) -> &str {
    todo!()
  }

  pub fn operation<T>(self) -> Result<T, GenericError> {
    todo!()
  }
}

impl<T> BasicHttpServer<T> 
where 
  T: Fn(IncomingOperation) -> Result<impl Serialize, GenericError>
{
  fn new(
    address: &SocketAddrV4,
    handler: T
  ) -> Result<Self, GenericError> {
    todo!()
  }
}


// 4 KB
const MAX_QUESTION_PAYLOAD_SIZE: usize = 4096; 

pub enum ResponseCreator {
  Json(Vec<u8>),
  BadQuestion,
  PayloadTooLarge,
  InternalServerError,
  NotFound,
}

impl ResponseCreator {
  fn json<T>(value: T) -> ResponseCreator 
  where
    T: Serialize
  {
    match serde_json::to_vec_pretty(&value) {
      Ok(value) => {
        ResponseCreator::Json(value)
      }
      Err(error) => {
        eprintln!("Discipline.Server.Respond.SerializeOutgoingOperationOutcome: {error}");
        ResponseCreator::InternalServerError
      }
    }
  }
}

trait QuestionMethods {
  fn respond_with_not_found(self);
  fn respond_with_bad_request(self);
  fn respond_with_internal_server_error(self);
  fn respond_with_json(self, data: Vec<u8>);
  fn respond_with_payload_too_large(self);
  fn respond_with(self, response_creator: ResponseCreator);
  fn matches(&self, path: &str, method: Method) -> bool;
  fn body_as<T>(&mut self) -> Result<T, ResponseCreator>
  where
    T: DeserializeOwned;
}

impl QuestionMethods for Request {
  fn respond_with_not_found(self) {
    if let Err(error) = self.respond(Response::empty(404)) {
      eprintln!("Discipline.Server.RespondWithNotFound: {error}");
    }
  }

  fn respond_with_payload_too_large(self) {
    if let Err(error) = self.respond(Response::empty(413)) {
      eprintln!("Discipline.Server.RespondeWithPayloodTooLarge: {error}");
    }
  }

  fn respond_with_bad_request(self) {
    if let Err(error) = self.respond(Response::empty(400)) {
      eprintln!("Discipline.Server.RespondWithBadRequest: {error}");
    }
  }

  fn respond_with_internal_server_error(self) {
    if let Err(error) = self.respond(Response::empty(500)) {
      eprintln!("Discipline.Server.RespondWithInternalServerError: {error}");
    }
  }

  fn respond_with_json(self, data: Vec<u8>) {
    let data_length = data.len();
    let response = Response::from_data(data)
      .with_status_code(200)
      .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap())
      .with_header(Header::from_bytes(b"Content-Length", data_length.to_string().as_bytes()).unwrap())
    ;

    if let Err(error) = self.respond(response) {
      eprintln!("Discipline.Server.RespondWithData: {error}");
    }
  }

  fn respond_with(self, response_creator: ResponseCreator) {
    match response_creator {
      ResponseCreator::BadQuestion => {
        self.respond_with_bad_request();
      }
      ResponseCreator::InternalServerError => {
        self.respond_with_internal_server_error();
      }
      ResponseCreator::Json(json) =>{
        self.respond_with_json(json);
      }
      ResponseCreator::NotFound => {
        self.respond_with_not_found();
      }
      ResponseCreator::PayloadTooLarge => {
        self.respond_with_payload_too_large();
      } 
    }
  }

  fn matches(&self, path: &str, method: Method) -> bool {
    self.url() == path && *self.method() == method
  }
  
  fn body_as<T>(&mut self) -> Result<T, ResponseCreator>
  where
    T: DeserializeOwned 
  {
    // Read in 0.5 KB chunks.
    let mut buffer = vec![0u8; 512];
    // May hold a maximum of 4 KB worth of data.
    let mut payload: Vec<u8> = Vec::new();
    // Total number of bytes read into `payload`.
    let mut total_size = 0;
    let payload_reader = self.as_reader();
 
    loop {
      let size_read = match payload_reader.read(&mut buffer) {
        Ok(value) => {
          value
        }
        Err(error) => {
          eprintln!("Discipline.Server.Respond.ReadIncomingPayload: {error}");
          return Err(ResponseCreator::InternalServerError);
        }
      };
  
      if size_read == 0 {
        break;
      }
  
      total_size += size_read;
      if total_size > MAX_QUESTION_PAYLOAD_SIZE {
        eprintln!("Discipline.Server.Respond.QuestionPayloadTooLarge.");
        return Err(ResponseCreator::PayloadTooLarge);
      }
  
      payload.extend_from_slice(&buffer[..size_read]);
    }
  
    match serde_json::from_slice(&payload) {
      Ok(value) => {
        Ok(value)
      }
      Err(error) => {
        if let Ok(payload) = String::from_utf8(payload)  {
          eprintln!("Discipline.Server.Respond.DeserializeIncomingOperation: \nError: {error}.\nPayload: {payload}.");
        } else {
          eprintln!("Discipline.Server.Respond.DeserializeIncomingOperation: \nError: {error}");
        }
        Err(ResponseCreator::BadQuestion)
      }
    }
  }
}
