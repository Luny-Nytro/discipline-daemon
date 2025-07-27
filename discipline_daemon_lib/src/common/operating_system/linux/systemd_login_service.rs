use std::time::Duration;
use chrono::{TimeZone, Utc};
use dbus::{arg, blocking::{stdintf::org_freedesktop_dbus::Properties, Connection, Proxy}, Path};
use crate::DateTime;
use super::{UserId, Username};

/// SECTION: FeatureInterface
pub struct FeatureInterface<'a> {
  proxy: Proxy<'a, &'a Connection>
}

impl<'a> FeatureInterface<'a> {
  pub fn new(system_bus: &'a Connection) -> Self {
    let proxy = system_bus.with_proxy("org.freedesktop.login1", "/org/freedesktop/login1", Duration::from_millis(5000));

    Self {
      proxy,
    }
  }

  pub fn power_off(&self, interactive: bool) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "PowerOff", (interactive, ))
  }

  pub fn reboot(&self, interactive: bool) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "Reboot", (interactive, ))
  }

  /// Halts the system (shuts down the OS but does not power off the hardware).
  pub fn halt(&self, interactive: bool) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "Halt", (interactive, ))
  }

  /// uspends the system (puts it into a low-power state while preserving the system state in RAM).
  pub fn suspend(&self, interactive: bool) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "Suspend", (interactive, ))
  }

  /// Hibernates the system (saves the system state to disk and powers off the device).
  pub fn hibernate(&self, interactive: bool) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "Hibernate", (interactive, ))
  }

  /// Performs a hybrid sleep (saves the system state to disk and suspends to RAM).
  pub fn hybird_sleep(&self, interactive: bool) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "HybridSleep", (interactive, ))
  }

  /// Suspends the system first, then hibernates after a period of inactivity.
  pub fn suspend_then_hybernate(&self, interactive: bool) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "SuspendThenHibernate", (interactive, ))
  }

  /// Locks a session, preventing user interaction until it is unlocked.
  pub fn lock_session(&self, session_id: SessionId) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "LockSession", (session_id.0, ))
  }

  /// Unlocks a previously locked session, allowing user interaction again.
  pub fn unlock_session(&self, session_id: SessionId) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "UnlockSession", (session_id.0, ))
  }
  
  /// Locks all active sessions on the system.
  pub fn lock_all_sessions(&self) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "LockSessions", ())
  }

  /// Unlocks all previously locked sessions on the system.
  pub fn unlock_all_sessions(&self) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "UnlockSessions", ())
  }

  pub fn get_sessions(&self) -> Result<Vec<SessionInformation>, GetSessionsError> {
    type ReturnType = (Vec<(String, u32, String, dbus::Path<'static>)>, );

    let (unprepared_sessions_info, ): ReturnType = match self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "ListSessions", ()) 
    {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(GetSessionsError::DBus(error));
      }
    };

    let mut prepared_sesions_info = Vec::with_capacity(unprepared_sessions_info.len());

    for unprepared_session_info in unprepared_sessions_info {
      prepared_sesions_info.push(match SessionInformation::prepare(
        unprepared_session_info.0, 
        unprepared_session_info.1, 
        unprepared_session_info.2, 
        unprepared_session_info.3,
      ) {
        Ok(value) => {
          value
        } 
        Err(error) => {
          return Err(GetSessionsError::PrepareSession(error));
        }
      });
    }
    
    Ok(prepared_sesions_info)
  }

  pub fn get_user(&self, user_id: UserId) -> Result<UserObject, dbus::Error> {
    type ReturnType = (dbus::Path<'static>, );

    let (user_path, ): ReturnType = match self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "GetUser", (user_id.as_raw(), ))
    {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(error);
      }
    };

    Ok(UserObject::new(
      user_path, 
      self.proxy.connection,
    ))
  }

  pub fn get_users(&self) -> Result<Vec<UserObject>, GetUsersError> {
    type ReturnType = (Vec<(u32, String, dbus::Path<'static>)>, );

    let (unprepared_users_info, ): ReturnType = match self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "ListUsers", ()) 
    {
      Ok(value) => {
        value
      }
      Err(error) => {
        return Err(GetUsersError::DBus(error));
      }
    };

    let mut prepared_users_info = Vec::with_capacity(unprepared_users_info.len());
    for unprepared_user_info in unprepared_users_info {

      prepared_users_info.push(match UserInformation::prepare(
        unprepared_user_info.0,
        unprepared_user_info.1,
        unprepared_user_info.2,
      ) {
        Ok(value) => {
          value
        } 
        Err(error) => {
          return Err(GetUsersError::PrepareUser(error));
        }
      });
    }
    
    Ok(prepared_users_info)
  }

  pub fn terminate_session(&self, session_id: SessionId) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "TerminateSession", (session_id.0, ))
  }

  pub fn terminate_user(&self, user_id: UserId) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.login1.Feature", "TerminateUser", (user_id.as_raw(), ))
  }
}

pub enum GetSessionsError {
  DBus(dbus::Error),
  PrepareSession(SessionInformationDeserializeError),
}

pub enum GetUsersError {
  DBus(dbus::Error),
  PrepareUser(UserInformationDeserializeError),
}

/// SECTION: UserObject
pub enum UserGetNameError {
  DBus(dbus::Error),
  UserName,
}

pub enum UserGetUidError {
  DBus(dbus::Error)
}

pub enum UserGetLastActivityTime {
  DBus(dbus::Error),
  DateTime { timestamp: u64 },
}

pub enum UserGetStateError {
  DBus(dbus::Error),
  UnknownState { state: String },
}

/// [These states are mentiond in this DBus documentation](https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html)
/// 
/// [And are explained in this systemd documentation](https://www.freedesktop.org/software/systemd/man/latest/sd_uid_get_state.html)
pub enum UserState {
  /// User not logged in at all.
  Offline,
  /// User not logged in, but some user services are running.
  Lingering,
  /// User logged in, but not active, i.e. has no session in the foreground.
  Online,
  /// User logged in and has at least one active session, i.e. one session in the foreground
  Active,
  /// User not logged in, and not lingering, but some processes are still around.
  Closing,
}

impl UserState {
  pub fn from_system_value(value: &String) -> Option<UserState> {
    match value {
      "active" => {
        Some(Self::Active)
      }
      "online" => {
        Some(Self::Online)
      }
      "offline" => {
        Some(Self::Offline)
      }
      "closing" => {
        Some(Self::Closing)
      }
      "lingering" => {
        Some(Self::Lingering)
      }
      _ => {
        None
      }
    }
  }
}

pub enum UserGetIdleSinceError {
  DBuse(dbus::Error),
  DateTime { timestamp: u64 }
}
pub struct UserObject<'a> {
  proxy: Proxy<'a, &'a Connection>
}

impl<'a> UserObject<'a> {
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



/// Values passed to us by the "SessionNew" signal.
#[derive(Debug)]
pub struct LoginSessionNewSignal {
  pub session_id: SessionId,
  pub session_path: dbus::Path<'static>,
}

impl arg::AppendAll for LoginSessionNewSignal {
  fn append(&self, i: &mut arg::IterAppend) {
    arg::RefArg::append(&self.session_id, i);
    arg::RefArg::append(&self.session_path, i);
  }
}

impl arg::ReadAll for LoginSessionNewSignal {
  fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
    Ok(LoginSessionNewSignal {
      session_id: i.read()?,
      session_path: i.read()?,
    })
  }
}

impl dbus::message::SignalArgs for LoginSessionNewSignal {
  const NAME: &'static str = "SessionNew";
  const INTERFACE: &'static str = "org.freedesktop.login1.Feature";
}

#[derive(Debug)]
pub struct LoginSessionRemovedSignal {
  pub session_id: String,
  pub session_path: dbus::Path<'static>,
}

impl arg::AppendAll for LoginSessionRemovedSignal {
  fn append(&self, i: &mut arg::IterAppend) {
    arg::RefArg::append(&self.session_id, i);
    arg::RefArg::append(&self.session_path, i);
  }
}

impl arg::ReadAll for LoginSessionRemovedSignal {
  fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
    Ok(LoginSessionRemovedSignal {
      session_id: i.read()?,
      session_path: i.read()?,
    })
  }
}

impl dbus::message::SignalArgs for LoginSessionRemovedSignal {
  const NAME: &'static str = "SessionRemoved";
  const INTERFACE: &'static str = "org.freedesktop.login1.Feature";
}




fn get_user_id_from_session_path(
  conn: &Connection,
  session_path: dbus::Path<'static>,
) -> Result<u32, Box<dyn Error>> {
  // Create a proxy for the session object
  let proxy = conn.with_proxy("org.freedesktop.login1", session_path, Duration::from_millis(5000));

  // Get the "User" property from the session object
  let user: (dbus::Path<'static>, ) = proxy.method_call(
    "org.freedesktop.DBus.Properties",
    "Get",
    ("org.freedesktop.login1.Session", "User"),
  )?;

  // The "User" property is a D-Bus path to the user object, e.g., `/org/freedesktop/login1/user/_1000`
  let user_path = user.0;

  // Extract the UID from the user path (e.g., `_1000` -> `1000`)
  let uid_str = user_path
    .split('_')
    .last()
    .ok_or("Failed to extract UID from user path")?;

  let uid = uid_str.parse::<u32>()?;

  Ok(uid)
}

fn monitor_operating_system_signals() -> Result<(), Box<dyn Error>> {
  // Connect to the system bus (not the session bus, since login1 is a system service)
  let c = Connection::new_system()?;

  {
    // Create a proxy for the org.freedesktop.login1.Feature interface
    let proxy = c.with_proxy("org.freedesktop.login1", "/org/freedesktop/login1", Duration::from_millis(5000));

    // Listen for SessionNew signals
    let _id_new = proxy.match_signal(|h: LoginSessionNewSignal, _: &Connection, _: &Message| {
      println!("New session created: {} at path {}", h.session_id, h.session_path);
      true
    });

    // Listen for SessionRemoved signals
    let _id_removed = proxy.match_signal(|h: LoginSessionRemovedSignal, _: &Connection, _: &Message| {
      println!("Session removed: {} at path {}", h.session_id, h.session_path);
      true
    });
  }

  // Listen to incoming signals forever.
  loop {
    c.process(Duration::from_millis(1000))?;
  }
}

// fn main() -> Result<(), Box<dyn Error>> {
//     // Connect to the system bus (not the session bus, since login1 is a system service)
//     let c = Connection::new_system()?;

//     {
//         // Create a proxy for the org.freedesktop.login1.Feature interface
//         let proxy = c.with_proxy("org.freedesktop.login1", "/org/freedesktop/login1", Duration::from_millis(5000));

//         // Listen for SessionNew signals
//         let _id_new = proxy.match_signal(|h: SessionNew, conn: &Connection, _: &Message| {
//             match get_user_id_from_session_path(conn, h.session_path.clone()) {
//                 Ok(uid) => println!("New session created for user with UID: {}", uid),
//                 Err(e) => eprintln!("Failed to get UID for new session: {}", e),
//             }
//             true
//         });

//         // Listen for SessionRemoved signals
//         let _id_removed = proxy.match_signal(|h: SessionRemoved, conn: &Connection, _: &Message| {
//             match get_user_id_from_session_path(conn, h.session_path.clone()) {
//                 Ok(uid) => println!("Session removed for user with UID: {}", uid),
//                 Err(e) => eprintln!("Failed to get UID for removed session: {}", e),
//             }
//             true
//         });
//     }

//     // Listen to incoming signals forever.
//     loop {
//         c.process(Duration::from_millis(1000))?;
//     }
// }


// pub trait SignalHandler {
//   fn on_user_login(&mut self, user_id: OperatingSystemUserId);
//   fn on_user_logout(&mut self, user_id: OperatingSystemUserId);
// }


// SECTION: SessionId

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId(String);


// SECTION: SessionInformation
pub struct SessionInformation {
  user_id: UserId,
  user_name: Username,
  session_id: SessionId,
  session_path: dbus::Path<'static>,
}

impl SessionInformation {
  pub fn prepare(
    session_id: String,
    user_id: u32,
    user_name: String,
    session_path: dbus::Path<'static>,
  ) -> Result<Self, SessionInformationDeserializeError> {
    Ok(Self {
      session_id,
      session_path,
      user_id: UserId::new(user_id),
      user_name: match Username::new(user_name) {
        Some(value) => {
          value
        }
        None => {
          return Err(SessionInformationDeserializeError::Username);
        }
      }
    })
  }
}

// SECTION: SessionInformationDeserializeError

pub enum SessionInformationDeserializeError {
  Username
}

// SECTION: UserInformation
pub struct UserInformation {
  id: UserId,
  name: Username,
  path: Path<'static>
}

impl UserInformation {
  pub fn prepare(
    id: u32,
    name: String,
    path: Path<'static>
  ) -> Result<Self, UserInformationDeserializeError> {
    Ok(Self {
      id: UserId::new(id),
      name: match Username::new(name) {
        Some(value) => {
          value
        }
        None => {
          return Err(UserInformationDeserializeError::Username)
        }
      },
      path,
    })
  }
}

// SECTION: UserInformationDeserializeError
pub enum UserInformationDeserializeError {
  Username
}