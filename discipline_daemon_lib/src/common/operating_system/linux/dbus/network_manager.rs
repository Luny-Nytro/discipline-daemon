use std::time::Duration;
use dbus::{arg::{AppendAll, PropMap, ReadAll}, blocking::{Connection, Proxy}, message::SignalArgs, Message};

pub struct NetworkFeature<'a> {
  proxy: Proxy<'a, &'a Connection>,
}

impl<'a> NetworkFeature<'a> {
  pub fn new(system_bus: &'a Connection) -> Self {
    Self {
      proxy: system_bus.with_proxy(
        "org.freedesktop.NetworkFeature", 
        "/org/freedesktop/NetworkFeature", 
        Duration::from_millis(5000),
      )
    }
  }

  pub fn enable(&self) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.NetworkFeature", "Enable", (true, ))
  }

  pub fn disable(&self) -> Result<(), dbus::Error> {
    self
      .proxy
      .method_call("org.freedesktop.NetworkFeature", "Enable", (false, ))
  }

  pub fn networking_enabled(&self) -> Result<bool, dbus::Error> {
    self
      .proxy
      .get("org.freedesktop.NetworkFeature", "Enable")
  }

  pub fn listen_to_netwotking_enabled_property(&self) {
    let x = self.proxy.match_signal(|signal: PropertiesChanged, _: &Connection, _: &Message| {
      true
    });

    // self.proxy.
  }
}


pub struct PropertiesChanged  {
  properties: PropMap
}

impl ReadAll for PropertiesChanged {
  fn read(i: &mut dbus::arg::Iter) -> Result<Self, dbus::arg::TypeMismatchError> {
    Ok(Self {
      properties: i.read()?,
    })
  }
}

impl AppendAll for PropertiesChanged {
  fn append(&self, i: &mut dbus::arg::IterAppend) {
    i.append(self.properties);
  }
}

impl SignalArgs for PropertiesChanged {
  const NAME: &'static str = "PropertiesChanged";
  const INTERFACE: &'static str = "org.freedesktop.NetworkFeature";
}

pub enum ConnectivityState {
  /// Network connectivity is unknown.
  /// 
  /// Corresponds to NM_CONNECTIVITY_UNKNOWN.
  Unknown = 1,
  /// The host is not connected to any network.
  /// 
  /// Corresponds to NM_CONNECTIVITY_NONE.
  None = 2,
  /// The host is behind a captive portal and cannot reach the full Internet.
  /// 
  ///  Corresponds to NM_CONNECTIVITY_PORTAL.
  Portal = 3,
  /// The host is connected to a network, but does not appear to be able to 
  /// reach the full Internet.
  /// 
  /// Corresponds to NM_CONNECTIVITY_LIMITED.
	Limited = 4,
  /// The host is connected to a network, and appears to be able to reach the 
  /// full Internet.
  /// 
  /// Corresponds to NM_CONNECTIVITY_FULL.
  Full = 5,
}