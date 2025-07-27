use llhttp_sys::*;
use std::ffi::CStr;

#[derive(Debug, Clone, Copy)]
pub struct Error(u32);

impl From<llhttp_errno_t> for Error {
  fn from(llhttp_error: llhttp_errno_t) -> Self {
    Error(llhttp_error.0)
  }
}

impl From<Error> for llhttp_errno_t {
  fn from(error: Error) -> Self {
    llhttp_errno_t(error.0)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message = unsafe { 
      CStr::from_ptr(llhttp_errno_name(llhttp_errno_t(self.0))) 
    };

    write!(f, "{}", message.to_str().unwrap_or("unknown"))
  }
}

impl std::error::Error for Error {

}

pub type ParserResult<T> = Result<T, Error>;

struct ParserContext<'a, H: Callbacks + 'a> {
  parser: &'a mut Parser,
  callbacks: &'a mut H,
}

#[inline]
unsafe fn unwrap_context<'a, H: Callbacks>(parser: *mut llhttp_t) -> &'a mut ParserContext<'a, H> {
  &mut *((*parser).data as *mut ParserContext<H>)
}

macro_rules! callback_caller {
  ($callback_name: ident) => {{
    extern "C" fn $callback_name<H: Callbacks>(parser: *mut llhttp_t) -> libc::c_int {
      let context = unsafe { 
        unwrap_context::<H>(parser) 
      };

      match context.callbacks.$callback_name(context.parser) {
        Ok(()) => {
          0
        }
        Err(error) => {
          error.0 as libc::c_int
        }
      }
    }

    $callback_name::<Self>
  }};
}

macro_rules! data_callback_caller {
  ($callback_name: ident) => {{    
    extern "C" fn $callback_name<H: Callbacks>(
      parser: *mut llhttp_t, 
      data: *const libc::c_char, 
      data_length: usize,
    ) -> 
      libc::c_int 
    {
      let data_as_slice = unsafe {
        std::slice::from_raw_parts(data as *const u8, data_length)
      };

      let context = unsafe {
        unwrap_context::<H>(parser)
      };

      match context.callbacks.$callback_name(context.parser, data_as_slice) {
        Ok(()) => {
          0
        }
        Err(error) => {
          error.0 as libc::c_int
        }
      }
    }

    $callback_name::<Self>
  }};
}

macro_rules! noop_callback {
  ($callback_name: ident) => {
    fn $callback_name(&mut self, _parser: &mut Parser) -> ParserResult<()> {
      Ok(())
    }
  };
}

macro_rules! noop_data_callback {
  ($callback_name: ident) => {
    fn $callback_name(&mut self, _parser: &mut Parser, _data: &[u8]) -> ParserResult<()> {
      Ok(())
    }
  };
}

/// a list of callbacks that can be used to handle events from the parser
/// https://github.com/nodejs/llhttp#api
pub trait Callbacks: Sized {
  noop_callback!(on_message_begin);
  noop_data_callback!(on_protocol);
  noop_callback!(on_protocol_complete);
  noop_data_callback!(on_url);
  noop_data_callback!(on_status);
  noop_data_callback!(on_method);
  noop_data_callback!(on_version);
  noop_data_callback!(on_header_field);
  noop_data_callback!(on_header_value);
  noop_data_callback!(on_chunk_extension_name);
  noop_data_callback!(on_chunk_extension_value);
  noop_callback!(on_headers_complete);
  noop_data_callback!(on_body);
  noop_callback!(on_message_complete);
  noop_callback!(on_url_complete);
  noop_callback!(on_status_complete);
  noop_callback!(on_method_complete);
  noop_callback!(on_version_complete);
  noop_callback!(on_header_field_complete);
  noop_callback!(on_header_value_complete);
  noop_callback!(on_chunk_extension_name_complete);
  noop_callback!(on_chunk_extension_value_complete);
  noop_callback!(on_chunk_header);
  noop_callback!(on_chunk_complete);
  noop_callback!(on_reset);

  fn into_settings() -> llhttp_settings_t {
    llhttp_settings_t {
      on_protocol: Some(data_callback_caller!(on_protocol)),
      on_protocol_complete: Some(callback_caller!(on_protocol_complete)),
      on_message_begin: Some(callback_caller!(on_message_begin)),
      on_url: Some(data_callback_caller!(on_url)),
      on_status: Some(data_callback_caller!(on_status)),
      on_method: Some(data_callback_caller!(on_method)),
      on_version: Some(data_callback_caller!(on_version)),
      on_header_field: Some(data_callback_caller!(on_header_field)),
      on_header_value: Some(data_callback_caller!(on_header_value)),
      on_chunk_extension_name: Some(data_callback_caller!(on_chunk_extension_name)),
      on_chunk_extension_value: Some(data_callback_caller!(on_chunk_extension_value)),
      on_headers_complete: Some(callback_caller!(on_headers_complete)),
      on_body: Some(data_callback_caller!(on_body)),
      on_message_complete: Some(callback_caller!(on_message_complete)),
      on_url_complete: Some(callback_caller!(on_url_complete)),
      on_status_complete: Some(callback_caller!(on_status_complete)),
      on_method_complete: Some(callback_caller!(on_method_complete)),
      on_version_complete: Some(callback_caller!(on_version_complete)),
      on_header_field_complete: Some(callback_caller!(on_header_field_complete)),
      on_header_value_complete: Some(callback_caller!(on_header_value_complete)),
      on_chunk_extension_name_complete: Some(callback_caller!(on_chunk_extension_name_complete)),
      on_chunk_extension_value_complete: Some(callback_caller!(on_chunk_extension_value_complete)),
      on_chunk_header: Some(callback_caller!(on_chunk_header)),
      on_chunk_complete: Some(callback_caller!(on_chunk_complete)),
      on_reset: Some(callback_caller!(on_reset)),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Parser {
  inner: llhttp_t,
}

impl Parser {
  pub fn new_request_parser() -> Self {
    let mut parser = Parser {
      inner: llhttp_t::default(),
    };

    unsafe {
      llhttp_init(
        &mut parser.inner as *mut _,
        llhttp_type_t::HTTP_REQUEST,
        std::ptr::null::<llhttp_settings_t>() as *mut _,
      );
    };

    parser
  }

  pub fn new_response_parser() -> Self {
    let mut parser = Parser {
      inner: llhttp_t::default(),
    };

    unsafe {
      llhttp_init(
        &mut parser.inner as *mut _,
        llhttp_type_t::HTTP_RESPONSE,
        std::ptr::null::<llhttp_settings_t>() as *mut _,
      );
    };

    parser
  }

  pub fn new_message_parser() -> Self {
    let mut this = Parser {
      inner: llhttp_t::default(),
    };
    
    unsafe {
      llhttp_init(
        &mut this.inner as *mut _,
        llhttp_type_t::HTTP_BOTH,
        std::ptr::null::<llhttp_settings_t>() as *mut _,
      );
    };
    this
  }

  /// Parse full or partial request/response, invoking user callbacks along the way.
  ///
  /// If any of llhttp_data_cb returns errno not equal to HPE_OK - the parsing interrupts, and such errno is returned from llhttp_execute(). If HPE_PAUSED was used as a errno, the execution can be resumed with llhttp_resume() call.
  ///
  /// In a special case of CONNECT/Upgrade request/response HPE_PAUSED_UPGRADE is returned after fully parsing the request/response. If the user wishes to continue parsing, they need to invoke llhttp_resume_after_upgrade().
  pub fn parse<H: Callbacks>(&mut self, callbacks: &mut H, data: &[u8]) -> ParserResult<()> {
    let mut settings = H::into_settings();
    let mut context = ParserContext {
      parser: self,
      callbacks,
    };

    self.inner.data = &mut context as *mut _ as *mut _;
    self.inner.settings = &mut settings as *mut _ as *mut _;

    unsafe {
      let result = llhttp_execute(
        &mut self.inner as *mut _,
        data as *const _ as *const _,
        data.len() as _,
      );

      match result.0 {
        0 => Ok(()),
        _ => Err(result.into()),
      }
    }
  }

  /// get_version returns the HTTP version of the parsed message.
  pub fn get_version(&self) -> Option<http::version::Version> {
    match self.inner.http_major {
      0 => match self.inner.http_minor {
        9 => Some(http::version::Version::HTTP_09),
        _ => None,
      },
      1 => match self.inner.http_minor {
        0 => Some(http::version::Version::HTTP_10),
        1 => Some(http::version::Version::HTTP_11),
        _ => None,
      },
      2 => Some(http::version::Version::HTTP_2),
      3 => Some(http::version::Version::HTTP_3),
      _ => None,
    }
  }

  /// get_method returns the HTTP method of the parsed message.
  pub fn get_method(&self) -> Option<http::method::Method> {
    match llhttp_method_t(self.inner.method as _) {
      llhttp_method_t::HTTP_DELETE => Some(http::method::Method::DELETE),
      llhttp_method_t::HTTP_GET => Some(http::method::Method::GET),
      llhttp_method_t::HTTP_HEAD => Some(http::method::Method::HEAD),
      llhttp_method_t::HTTP_POST => Some(http::method::Method::POST),
      llhttp_method_t::HTTP_PUT => Some(http::method::Method::PUT),
      llhttp_method_t::HTTP_CONNECT => Some(http::method::Method::CONNECT),
      llhttp_method_t::HTTP_OPTIONS => Some(http::method::Method::OPTIONS),
      llhttp_method_t::HTTP_TRACE => Some(http::method::Method::TRACE),
      llhttp_method_t::HTTP_PATCH => Some(http::method::Method::PATCH),
      _ => None,
    }
  }

  /// get_status_code returns the HTTP status code of the parsed message.
  pub fn get_status_code(&self) -> Option<http::status::StatusCode> {
    http::status::StatusCode::from_u16(self.inner.status_code as _).ok()
  }

  /// get_upgrade returns true if the parsed message is an upgrade request.
  pub fn get_upgrade(&self) -> bool {
    self.inner.upgrade != 0
  }

  /// should_keep_alive returns true if the parsed message should keep the connection alive.
  pub fn should_keep_alive(&self) -> bool {
    unsafe { llhttp_should_keep_alive(&self.inner as *const _) != 0 }
  }

  /// pause pauses the parser.
  pub fn pause(&mut self) {
    unsafe {
      llhttp_pause(&mut self.inner as *mut _);
    }
  }

  /// resume resumes the parser.
  pub fn resume(&mut self) {
    unsafe {
      llhttp_resume(&mut self.inner as *mut _);
    }
  }

  /// resume_after_upgrade resumes the parser after an upgrade request.
  pub fn resume_after_upgrade(&mut self) {
    unsafe {
      llhttp_resume_after_upgrade(&mut self.inner as *mut _);
    }
  }
}

impl Default for Parser {
  fn default() -> Self {
    let mut this = Parser {
      inner: llhttp_t::default(),
    };
    unsafe {
      llhttp_init(
        &mut this.inner as *mut _,
        llhttp_type_t::HTTP_BOTH,
        std::ptr::null::<llhttp_settings_t>() as *mut _,
      );
    };
    this
  }
}

impl From<Parser> for llhttp_t {
  fn from(parser: Parser) -> Self {
    parser.inner
  }
}

impl From<llhttp_t> for Parser {
  fn from(inner: llhttp_t) -> Self {
    Parser { inner }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_request_parser() {
    #[derive(Default)]
    struct TestRequestParser {
      called_on_url: bool,
      called_on_header_field: bool,
      called_on_header_value: bool,
      called_on_body: bool,
    }

    impl Callbacks for TestRequestParser {
      fn on_url(&mut self, _: &mut Parser, url: &[u8]) -> ParserResult<()> {
        assert_eq!(b"/say_hello", url);
        self.called_on_url = true;
        Ok(())
      }

      fn on_header_field(&mut self, _: &mut Parser, hdr: &[u8]) -> ParserResult<()> {
        assert!(hdr == b"Host" || hdr == b"Content-Length");
        self.called_on_header_field = true;
        Ok(())
      }

      fn on_header_value(&mut self, _: &mut Parser, val: &[u8]) -> ParserResult<()> {
        assert!(val == b"localhost.localdomain" || val == b"11");
        self.called_on_header_value = true;
        Ok(())
      }

      fn on_body(&mut self, parser: &mut Parser, body: &[u8]) -> ParserResult<()> {
        assert_eq!(parser.get_method(), Some(http::method::Method::Method::POST));
        assert_eq!(parser.get_version(), Some(http::version::Version::HTTP_11));
        assert_eq!(parser.get_status_code(), None);
        assert_eq!(parser.get_upgrade(), false);
        assert_eq!(body, b"Hello world");
        self.called_on_body = true;
        Ok(())
      }
    }

    let req = b"POST /say_hello HTTP/1.1\r\nContent-Length: 11\r\nHost: localhost.localdomain\r\n\r\nHello world";
    let mut handler = TestRequestParser::default();
    let mut parser = Parser::new_request_parser();

    parser.parse(&mut handler, req).unwrap();

    assert!(handler.called_on_url);
    assert!(handler.called_on_header_field);
    assert!(handler.called_on_header_value);
    assert!(handler.called_on_body);
  }

  #[test]
  fn test_ws_upgrade() {
    struct DummyHandler;
    impl Callbacks for DummyHandler {}

    let req = b"GET / HTTP/1.1\r\nConnection: Upgrade\r\nUpgrade: websocket\r\n\r\n";

    let mut handler = DummyHandler;
    let mut parser = Parser::new_request_parser();

    match parser.parse(&mut handler, req) {
      Err(err) if llhttp_errno_t::HPE_PAUSED_UPGRADE == err.into() => {
        assert_eq!(parser.get_upgrade(), true);
        parser.resume_after_upgrade();
      }
      _ => panic!("Unexpected error"),
    }

    parser.parse(&mut handler, b"").unwrap();
  }

  #[test]
  fn test_streaming() {
    struct DummyHandler;
    impl Callbacks for DummyHandler {}

    let req = b"GET / HTTP/1.1\r\nHeader: hello\r\n\r\n";

    let mut handler = DummyHandler;
    let mut parser = Parser::new_request_parser();

    parser.parse(&mut handler, &req[0..10]).unwrap();
    assert_eq!(parser.get_version(), None);

    parser.parse(&mut handler, &req[10..]).unwrap();
    assert_eq!(parser.get_version(), Some(http::version::Version::HTTP_11));
  }
}
