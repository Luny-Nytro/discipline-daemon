pub struct Service {
  name: String,
  // body: 
}

static mut SERVICE_NAME: Option<&'static str> = None;

pub fn initialize_service_name(service_name: &'static str) {
  unsafe {
    SERVICE_NAME = Some(service_name);
  }
}

/// *SAFETY*: May not be called before the service name is initialized by calling
/// `initialize_service_name("some-service-name")`.
pub fn service_name() -> &'static str {
  unsafe {
    SERVICE_NAME.unwrap()
  }
}