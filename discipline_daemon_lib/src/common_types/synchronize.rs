use crate::{DateTime, Time};

pub struct SynchronizeArg {
  pub time: Time,
  pub datetime: DateTime,
  pub networking_enabled: bool,
}