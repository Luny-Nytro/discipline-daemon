use crate::{daemon::{AddBlocklistItemError, DaemonMutex}, utility::GenericError};
use tiny_http::{Response, Server};
use http::Uri;

pub fn run(daemon: DaemonMutex) -> Result<(), GenericError> {
  let address = {
    daemon.lock()?.api_server_address().clone()
  };

  let server = Server::http(address).map_err(|error| 
    GenericError::new("running api http server")
      .add_attachment("error", error.to_string())
      .add_attachment("address", address.to_string())
  )?;

  loop {
    let req = match server.recv() {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!(
          "error: {:?}", 
          GenericError::new("discipline api server resieving a request")
            .add_attachment("error", error.to_string())
        );

        continue;
      }
    };

    let uri = match req.url().parse::<Uri>() {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!(
          "error: {:?}",
          GenericError::new("duscipline api server parsing uri of incoming request")
            .add_attachment("request", format!("{req:?}"))
            .add_attachment("uri", req.url())
            .add_attachment("error", error.to_string())
        );

        _ = req.respond(Response::from_data("malformed uri"));
        continue;
      }
    };

    if uri.path() != "block_domain" {
      _ = req.respond(Response::from_data("malformed uri"));
      continue;
    }

    let Some(domain) = uri.query() else {
      _ = req.respond(Response::from_data("missing domain name to block. you must provide it as the query portion of the url"));
      continue;
    };

    match daemon.add_blocklist_item(domain.into()) {
      Ok(_) => {
        _ = req.respond(Response::from_data(format!("successfuly added '{domain}' to the blocklist, yay!")));
      }
      Err(AddBlocklistItemError::DomainIsTooLong) => {
        _ = req.respond(Response::from_data(format!("failed to add '{domain}' to the blocklist because it's too long")));
      }
      Err(AddBlocklistItemError::TooManyDomains) => {
        _ = req.respond(Response::from_data(format!("failed to add '{domain}' to the blocklist because the blocklist already contains too many domains")));
      }
      Err(AddBlocklistItemError::InternalError(error)) => {
        _ = req.respond(Response::from_data(format!("failed to add '{domain}' to the blocklist because of an internal error")));
        eprintln!(
          "error: {:?}", 
          error
            .change_context("discipline api server responding to a request to add a new domain to the blocklist")
            .add_attachment("domain", domain)
        );
      }
    }
  }
}