use crate::{DateTime, Duration};

pub struct DeviceUptimeTracker {
  beginning: DateTime,
  uptime_since_beginning: Duration,
}