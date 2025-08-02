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
use url::Url;
use std::thread::{spawn, JoinHandle};
use super::WebServer;

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
    
    let server = WebServer::new(address)?;

    let thread = spawn(move || {
      // TODO: Make sure "incoming_requests()" doesn't panic.

      loop {
        let Ok(request) = server.recieve() else {
          continue;
        };

        match request.url().path() {
          "/Api/ExecuteOperation" => {
            if request.query_parameters().len() != 1 {
              // TODO: Add error message "Expected exactly one query parameter"
              request.respond_with_http_bad_request();
              continue;
            }

            let Some(operation_id) = request.query_parameters().get("operation_id") else {
              // TODO: Add error message "Expected an 'operation_id' parameter"
              request.respond_with_http_bad_request();
              continue;
            };

            let Some(operation_specification) = operation_specifications
              .specifications
              .get(operation_id.as_ref()) else 
            {
              request.respond_with_http_success(GenericOpertionReturn::unknown_operation());
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