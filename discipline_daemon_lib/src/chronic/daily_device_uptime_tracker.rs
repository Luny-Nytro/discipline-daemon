use crate::{DateTime, Duration};

pub struct DailyDeviceUptimeTracker {
  uptime_duration_today: Duration,
  previous_synchronization_time: DateTime,
}

