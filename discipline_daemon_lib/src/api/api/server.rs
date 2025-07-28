use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use crate::{Daemon, GenericError, find_operation_type};

use super::basic_web_server::*;
use super::*;

pub struct Api {
  server: BasicHttpServer
}

impl Api {
  pub fn new(daemon: Arc<Daemon>) -> Result<Self, GenericError> {

    let address = SocketAddrV4::new(
      Ipv4Addr::LOCALHOST, 
      daemon.configuration().api_tcp_port(),
    );
    
    let server = BasicHttpServer::new(
      &address, 
      |id, operation| {
        find_operation_type! {
          id,
          |Operation| {
            server_return(operation
                .try_as::<Operation>()?
                .execute(Arc::clone(&daemon))
            )
          }
          else {
            Ok(server_no_such_operation())
          }
        }
      }
    )?;

    Ok(Self {
      server,
    })
  }
}