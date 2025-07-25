use super::*;

pub enum UserIdentificationMethod {
  Id(OperatingSystemUserId),
  Name(OperatingSystemUserName),
}
