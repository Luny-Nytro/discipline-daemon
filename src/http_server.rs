use serde::{de::DeserializeOwned, Serialize};
use tiny_http::{Header, Method, Request, Response, Server};
use super::{
  AppMutex, shadow_vaults, user_access, networking_access,
  GetData,
};

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

fn respond(mut app: AppMutex, question: &mut Request) -> ResponseCreator {
  if question.matches("/ShadowVaults/Create", Method::Post) {
    let operation = match question.body_as::<shadow_vaults::CreateShadowVault>() {
      Ok(value) => value,
      Err(response) => return response,
    };

    let operation_outcome = app.execute(operation);

    return ResponseCreator::json(operation_outcome);
  }
  
  if question.matches("/ShadowVaults/Delete", Method::Post) {
    let operation = match question.body_as::<shadow_vaults::DeleteShadowVault>() {
      Ok(value) => value,
      Err(response) => return response,
    };

    let operation_outcome = app.execute(operation);

    return ResponseCreator::json(operation_outcome);
  }

  if question.matches("/ShadowVaults/ChangeName", Method::Post) {
    let operation = match question.body_as::<shadow_vaults::ChangeShadowVaultName>() {
      Ok(value) => value,
      Err(response) => return response,
    };

    let operation_outcome = app.execute(operation);

    return ResponseCreator::json(operation_outcome);
  }
  
  if question.matches("/ShadowVaults/Protector/ForDuration/Increment", Method::Post) {
    let operation = match question.body_as::<shadow_vaults::ProtectorForDurationIncrement>() {
      Ok(value) => value,
      Err(response) => return response,
    };

    let operation_outcome = app.execute(operation);

    return ResponseCreator::json(operation_outcome);
  }

  // UserAccess  
  if question.matches("/UserAccess/Enforcers/Create", Method::Post) {
    return match question.body_as::<user_access::create_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Enforcers/Delete", Method::Post) {
    return match question.body_as::<user_access::delete_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Enforcers/Enable", Method::Post) {
    return match question.body_as::<user_access::enable_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Enforcers/Disable", Method::Post) {
    return match question.body_as::<user_access::disable_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Create", Method::Post) {
    return match question.body_as::<user_access::rules_create::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Delete", Method::Post) {
    return match question.body_as::<user_access::rules_delete::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Activator/ForDuration/Increment", Method::Post) {
    return match question.body_as::<user_access::activator_for_duration_increment::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Activator/InTimeRange/Modify", Method::Post) {
    return match question.body_as::<user_access::rules_activator_in_time_range_update_range::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Activator/InWeekdayRange/Modify", Method::Post) {
    return match question.body_as::<user_access::rules_activator_in_weekday_range_update_range::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Activator/NotInTimeRange/Modify", Method::Post) {
    return match question.body_as::<user_access::activator_not_in_time_range_modify::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Activator/NotInWeekdayRange/Modify", Method::Post) {
    return match question.body_as::<user_access::activator_not_in_weekday_range_modify::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Enabler/ByPassword/MakeIneffective", Method::Post) {
    return match question.body_as::<user_access::enabler_by_password_make_ineffective::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Enabler/ForDuration/Increment", Method::Post) {
    return match question.body_as::<user_access::rules_deactivator_increment::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/UserAccess/Rules/Enabler/ByPassword/MakeEffective", Method::Post) {
    return match question.body_as::<user_access::enabler_by_password_make_effective::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }

  // NetworkingAccess
  if question.matches("/NetworkingAccess/Enforcers/Create", Method::Post) {
    return match question.body_as::<networking_access::create_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Enforcers/Delete", Method::Post) {
    return match question.body_as::<networking_access::delete_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Enforcers/Enable", Method::Post) {
    return match question.body_as::<networking_access::enable_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Enforcers/Disable", Method::Post) {
    return match question.body_as::<networking_access::disable_enforcer::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Create", Method::Post) {
    return match question.body_as::<networking_access::create_rule::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Delete", Method::Post) {
    return match question.body_as::<networking_access::delete_rule::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Activator/ForDuration/Increment", Method::Post) {
    return match question.body_as::<networking_access::activator_for_duration_increment::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Activator/InTimeRange/Modify", Method::Post) {
    return match question.body_as::<networking_access::activator_in_time_range_modify::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Activator/InWeekdayRange/Modify", Method::Post) {
    return match question.body_as::<networking_access::activator_in_weekday_range_modify::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Activator/NotInTimeRange/Modify", Method::Post) {
    return match question.body_as::<networking_access::activator_not_in_time_range_modify::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Activator/NotInWeekdayRange/Modify", Method::Post) {
    return match question.body_as::<networking_access::activator_not_in_weekday_range_modify::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Enabler/ByPassword/MakeIneffective", Method::Post) {
    return match question.body_as::<networking_access::enabler_by_password_make_ineffective::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Enabler/ForDuration/Increment", Method::Post) {
    return match question.body_as::<networking_access::enabler_for_duration_increment::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }
  if question.matches("/NetworkingAccess/Rules/Enabler/ByPassword/MakeEffective", Method::Post) {
    return match question.body_as::<networking_access::enabler_by_password_make_effective::Operation>() {
      Ok(operation) => {
        ResponseCreator::json(app.execute(operation))
      }
      Err(response) => {
        response
      }
    }
  }

  if question.matches("/App/GetData", Method::Post) {
    return ResponseCreator::json(app.execute(GetData));
  }

  ResponseCreator::NotFound
}

pub fn run(app: AppMutex) {
  println!("moon: run http server is just called");
  
  let address = app.http_server_address();
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
    let response_creator = respond(app.clone(), &mut question);
    question.respond_with(response_creator);
  }
}
