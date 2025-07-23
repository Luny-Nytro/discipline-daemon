use std::{fmt::Write, mem};
use crate::{database::ScalarTypeSerde, DateTime};
use super::{database_code, Activator, Feature, Enabler, Rule, Enforcer};

pub fn synchronize_activator(
  into: &mut impl Write, 
  now: DateTime,
  activator: &mut Activator, 
  serialized_rule_id: &String,
) {
  if let Activator::ForDuration(countdown_timer) = activator {
    countdown_timer.on_time_synchronize(now);

    let remaining_duration = countdown_timer.remaining_duration().serialize();
    let previous_synchronization_time = countdown_timer.previous_synchronization_time().serialize();
    writeln!(
      into, 
      "UPDATE NetworkingAccessRules 
        SET 
          ActivatorB = {remaining_duration}, 
          ActivatorC = {previous_synchronization_time} 
        WHERE 
          Id = {serialized_rule_id};
      "
    ).unwrap();
  }
}

pub fn synchronize_enabler(
  into: &mut impl Write, 
  now: DateTime,
  enabler: &mut Enabler, 
  serialized_rule_id: &String,
) {
  if let Enabler::ForDuration(enabler) = enabler {
    enabler.on_time_synchronize(now);
  
    let remaining_duration = enabler.remaining_duration().serialize();
    let previous_synchronization_time = enabler.previous_synchronization_time().serialize();
    writeln!(
      into, 
      "UPDATE NetworkingAccessRules 
        SET 
          EnablerA = {remaining_duration}, 
          EnablerB = {previous_synchronization_time} 
        WHERE 
          Id = {serialized_rule_id};
      "
    ).unwrap();
  }
}

pub fn synchronize_rule(
  into: &mut impl Write,
  now: DateTime,
  rule: &mut Rule,
) {
  let serialized_rule_id = rule.id.serialize();
  synchronize_activator(into, now, &mut rule.activator, &serialized_rule_id);
  synchronize_enabler(into, now, &mut rule.enabler, &serialized_rule_id);
}

fn synchronize_enforcer(
  into: &mut impl Write,
  now: DateTime,
  enforcer: &mut Enforcer,
) {
  enforcer.rules = mem::take(&mut enforcer.rules).into_iter().filter_map(|mut rule| {
    if rule.is_enabled(now) {
      synchronize_rule(into, now, &mut rule);
      // keep the rule.
      Some(rule)
    } else {
      database_code::delete_rule(into, &rule.id);
      // remove the rule
      None
    }
  }).collect();
}

pub fn synchronize_enforcers_feature(
  into: &mut impl Write,
  now: DateTime,
  enforcers_feature: &mut Feature,
) {
  for enforcer in &mut enforcers_feature.enforcers {
    synchronize_enforcer(into, now, enforcer);
  }
}