use std::fmt::Write;
use crate::database::{ScalarTypeSerde, Serialize};
use crate::{Duration, OperatingSystemUsername, Time, TimeRange, Uuid, Weekday, WeekdayRange};
use super::{database_serde::RuleSerializer, Enforcer, Rule};

pub fn initialize(into: &mut impl Write) {
  writeln!(into, "
    CREATE TABLE IF NOT EXISTS NetworkingAccessEnforcers (
      UserId INTEGER NOT NULL,
      Username TEXT NOT NULL,
      IsLocked INTEGER NOT NULL,
      IsEnabled INTEGER NOT NULL,
      PRIMARY KEY(UserId, Username)
    ) STRICT, WITHOUT ROWID;

    CREATE TABLE IF NOT EXISTS NetworkingAccessRules (
      Id TEXT NOT NULL PRIMARY KEY,
      ActivatorType INTEGER NOT NULL,
      ActivatorA,
      ActivatorB,
      ActivatorC,
      EnablerType INTEGER NOT NULL,
      EnablerA,
      EnablerB,
      EnablerC,
      Username TEXT NOT NULL,
      Position INTEGER NOT NULL
    );
  ").unwrap();
}

pub fn create_enforcer(into: &mut impl Write, enforcer: &Enforcer) {
  writeln!(into, "INSERT INTO NetworkingAccessEnforcers VALUES ({});", enforcer.serialize()).unwrap();
}

pub fn delete_enforcer(into: &mut impl Write, enforcer: &Enforcer) {
  writeln!(into, 
    "DELETE FROM NetworkingAccessEnforcers WHERE Username = {};", 
    enforcer.username.serialize(),
  ).unwrap();

  let mut wrote_nothing_yet = true;
  for rule in &enforcer.rules {
    if wrote_nothing_yet {
      writeln!(into, "DELETE FROM NetworkingAccessRules WHERE Id IN ({}", rule.id.serialize()).unwrap();
      wrote_nothing_yet = false;
    } else {
      writeln!(into, ", {}", rule.id.serialize()).unwrap()
    }
  }

  if !wrote_nothing_yet {
    writeln!(into, ");").unwrap();
  }
}

pub fn change_enforcer_is_blocked(
  into: &mut impl Write, 
  username: &OperatingSystemUsername,
  new_value: bool,
) {
  let username = username.serialize();
  let new_value = new_value.serialize();

  writeln!(into, "UPDATE NetworkingAccessEnforcer SET IsBlocked = {new_value} WHERE Username = {username};").unwrap();
}

pub fn enable_enforcer(
  into: &mut impl Write, 
  username: &OperatingSystemUsername,
) {
  let username = username.serialize();
  let is_enabled = true.serialize();

  writeln!(into, "UPDATE NetworkingAccessEnforcer SET IsEnabled = {is_enabled} WHERE Username = {username};").unwrap();
}

pub fn disable_enforcer(
  into: &mut impl Write, 
  username: &OperatingSystemUsername,
) {
  let username = username.serialize();
  let is_enabled = false.serialize();

  writeln!(into, "UPDATE NetworkingAccessEnforcer SET IsEnabled = {is_enabled} WHERE Username = {username};").unwrap();
}

pub fn create_rule(
  into: &mut impl Write, 
  rule: &Rule, 
  username: &OperatingSystemUsername,
  position: usize,
) {
  let rule = RuleSerializer::new(rule, username, position).serialize();

  writeln!(into, "INSERT INTO NetworkingAccessRules VALUES ({rule});").unwrap();
}

pub fn delete_rule(
  into: &mut impl Write, 
  rule_id: &Uuid,
) {
  writeln!(
    into, 
    "DELETE FROM NetworkingAccessRules WHERE Id = {};", 
    rule_id.serialize(),
  ).unwrap();
}

pub fn find_all_enforcers(into: &mut impl Write) {
  writeln!(into, "SELECT * FROM NetworkingAccessEnforcers;").unwrap();
}

pub fn find_all_rules(into: &mut impl Write) {
  writeln!(into, "SELECT * FROM NetworkingAccessRules;").unwrap();
}

pub fn activator_for_duration_change_remaining_duration(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_remaining_duration: &Duration,
) {
  let rule_id = rule_id.serialize();
  let new_remaining_duration = new_remaining_duration.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET ActivatorB = {new_remaining_duration} WHERE Id = {rule_id};").unwrap();
}

pub fn activator_in_time_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_value: &TimeRange,
) {
  let rule_id = rule_id.serialize();
  let new_value = new_value.as_numbers();
  let new_from = new_value.0.serialize();
  let new_till = new_value.1.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET ActivatorA = {new_from}, ActivatorB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn activator_not_in_time_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_value: &TimeRange,
) {
  let rule_id = rule_id.serialize();
  let new_value = new_value.as_numbers();
  let new_from = new_value.0.serialize();
  let new_till = new_value.1.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET ActivatorA = {new_from}, ActivatorB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn activator_in_weekday_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_value: &WeekdayRange
) {
  let rule_id = rule_id.serialize();
  let new_value = new_value.as_numbers();
  let new_from = new_value.0.serialize();
  let new_till = new_value.1.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET ActivatorA = {new_from}, ActivatorB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn activator_not_in_weekday_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_value: &WeekdayRange,
) {
  let rule_id = rule_id.serialize();
  let new_value = new_value.as_numbers();
  let new_from = new_value.0.serialize();
  let new_till = new_value.1.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET ActivatorA = {new_from}, ActivatorB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn enabler_for_duration_change_remaining_duration(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_remaining_duration: &Duration,
) {
  let rule_id = rule_id.serialize();
  let new_remaining_duration = new_remaining_duration.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET EnablerB = {new_remaining_duration} WHERE Id = {rule_id};").unwrap();
}

pub fn enabler_in_time_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_from: &Time,
  new_till: &Time,
) {
  let rule_id = rule_id.serialize();
  let new_from = new_from.serialize();
  let new_till = new_till.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET EnablerA = {new_from}, EnablerB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn enabler_not_in_time_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_from: &Time,
  new_till: &Time,
) {
  let rule_id = rule_id.serialize();
  let new_from = new_from.serialize();
  let new_till = new_till.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET EnablerA = {new_from}, EnablerB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn enabler_in_weekday_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_from: &Weekday,
  new_till: &Weekday,
) {
  let rule_id = rule_id.serialize();
  let new_from = new_from.serialize();
  let new_till = new_till.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET EnablerA = {new_from}, EnablerB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn enabler_not_in_weekday_range_replace(
  into: &mut impl Write,
  rule_id: &Uuid,
  new_from: &Weekday,
  new_till: &Weekday,
) {
  let rule_id = rule_id.serialize();
  let new_from = new_from.serialize();
  let new_till = new_till.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET EnablerA = {new_from}, EnablerB = {new_till} WHERE Id = {rule_id};").unwrap();
}

pub fn enabler_by_password_make_effective(
  into: &mut impl Write,
  rule_id: &Uuid,
) {
  let value = true.serialize();
  let rule_id = rule_id.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET EnablerB = {value} WHERE Id = {rule_id};").unwrap();
}

pub fn enabler_by_password_make_ineffective(
  into: &mut impl Write,
  rule_id: &Uuid,
) {
  let value = false.serialize();
  let rule_id = rule_id.serialize();
  writeln!(into, "UPDATE NetworkingAccessRules SET EnablerB = {value} WHERE Id = {rule_id};").unwrap();
}