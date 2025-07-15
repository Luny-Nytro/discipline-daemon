use std::ops::DerefMut;

use serde::{de::DeserializeOwned, Serialize};
use tiny_http::{Header, Method, Request, Response, Server};

use super::*;

use crate::find_operation_type;
use crate::DaemonMutex;
use crate::IsRemoteProcedureCall;

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

fn respond(daemon_mutex: DaemonMutex, question: &mut Request) -> ResponseCreator {
  find_operation_type!(
    question.url(), 
    |Operation| {
      let operation = match question.body_as::<Operation>() {
        Ok(operation) => operation,
        Err(response) => return response
      };

      let Ok(mut daemon_guard) = daemon_mutex.lock() else {
        return ResponseCreator::InternalServerError;
      };

      let daemon = daemon_guard.deref_mut();

      ResponseCreator::json(operation.execute(daemon))
    }
    else {
      ResponseCreator::NotFound
    }
  )
}

pub fn run(daemon_mutex: DaemonMutex) {
  println!("moon: run http server is just called");
  
  let address = {
    let Ok(daemon) = daemon_mutex.lock() else {
      return;
    };

    daemon.http_server_address.clone()
  };
  
  let server = match Server::http(&address) {
    Ok(server) => {
      println!("Discipline.Server.RunHTTPServer: Server running on {address}");
      server
    }
    Err(error) => {
      eprintln!("Discipline.Server.RunHTTPServer: {error}");
      return;
    }
  };

  for mut question in server.incoming_requests() {
    let response_creator = respond(daemon_mutex.clone(), &mut question);
    question.respond_with(response_creator);
  }
}


pub fn launch_thread(daemon_mutex: DaemonMutex) -> std::thread::JoinHandle<()> {
  std::thread::spawn(|| {
    run(daemon_mutex)
  })
}