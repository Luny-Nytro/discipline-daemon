
pub struct User<'a> {
  proxy: Proxy<'a, &'a Connection>
}

impl<'a> User<'a> {
  pub fn new(path: Path<'a>, connection: Connection) -> Self {
    let proxy = connection.with_proxy("org.freedesktop.login1", path, Duration::from_millis(5000));

    Self {
      proxy
    }
  }

  pub fn terminate(&self) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.User", "Terminate", ())
  }

  pub fn kill(&self) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.User", "Kill", ())
  }

  pub fn id(&self) -> Result<UserId, dbus::Error> {
    self
      .proxy
      .get("org.freedesktop.login1.User", "UID")
      .map(UserId::new)
  }

  pub fn name(&self) -> Result<String, UserGetNameError> {
    let name: String = match self
      .proxy
      .get("org.freedesktop.login1.User", "Name")
    {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(UserGetNameError::DBus(error));
      }
    };

    match Username::new(name) {
      Some(username) => {
        Ok(username)
      }
      None => {
        Err(UserGetNameError::UserName)
      }
    }
  }

  pub fn last_activity_time(&self) -> Result<DateTime, UserGetLastActivityTime> {
    let timestamp: u64 = match self
      .proxy
      .get("org.freedesktop.login1.User", "Timestamp")
    {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(UserGetLastActivityTime::DBus(error));
      }
    };

    match DateTime::from_timestamp(timestamp) {
      Some(datetime) => {
        Ok(datetime)
      }
      None => {
        Err(UserGetLastActivityTime::DateTime { timestamp })
      }
    }
  }

  pub fn state(&self) -> Result<UserState, UserGetStateError> {
    let state: String = match self
      .proxy
      .get("org.freedesktop.login1.User", "State")
    {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(UserGetStateError::DBus(error));
      }
    };

    match UserState::from_system_value(&state) {
      Some(value) => {
        Ok(value)
      }
      None => {
        Err(UserGetStateError::UnknownState { state })
      }
    }
  }

  pub fn is_idle(&self) -> Result<bool, dbus::Error> {
    self
      .proxy
      .get("org.freedesktop.login1.User", "IdleHint")
  }

  // If user is idle, returns Ok(Some(datetime)) of when it became idle.
  // Otherwise, it returns Ok(None).
  pub fn idle_since(&self) -> Result<Option<DateTime>, UserGetIdleSinceError> {
    let timestamp: u64 = match self
      .proxy
      // This returns a microsecond-base unix epoch timestamp.
      .get("org.freedesktop.login1.User", "IdleSinceHint")
    {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(UserGetIdleSinceError::DBus(error));
      }
    };

    if timestamp == 0 {
      // User is active (IdleSinceHint=0 means not idle)
      return Ok(None); 
    }
  
    let seconds = (timestamp / 1_000_000) as i64;
    let nanos = ((timestamp % 1_000_000) * 1_000) as u32;
  
    match Utc.timestamp_opt(seconds, nanos).single() {
      Some(value) => {
        Ok(Some(value))
      }
      None => {
        Err(UserGetIdleSinceError::DateTime { timestamp })
      }
    }
  }

  // TODO: Create a method to listen to the "IdleHint" property.
}
