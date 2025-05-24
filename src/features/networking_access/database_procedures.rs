use rusqlite::Connection;

use crate::{Duration, OperatingSystemUsername, Time, TimeRange, Uuid, Weekday, WeekdayRange};

use crate::database::deserialize;

use super::database_serde::{
  EnforcerNormalized, 
  FeatureNormalized, 
  RuleNormalized,
};

use super::{
  database_code, Enforcer, Rule, Feature
};

pub fn create_enforcer(
  connection: &Connection, 
  enforcer: &Enforcer,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::create_enforcer(&mut code, enforcer);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.CreateEnforcer: \nCode: {code}. \nError: {error}.");
  })
}

pub fn delete_enforcer(
  connection: &Connection, 
  enforcer: &Enforcer,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::delete_enforcer(&mut code, enforcer);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.DeleteEnforcer: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enable_enforcer(
  connection: &Connection, 
  username: &OperatingSystemUsername,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enable_enforcer(&mut code, username);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.EnableEnforcer: \nCode: {code}. \nError: {error}.");
  })
}

pub fn disable_enforcer(
  connection: &Connection, 
  username: &OperatingSystemUsername,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::disable_enforcer(&mut code, username);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.DisableEnforcer: \nCode: {code}. \nError: {error}.");
  })
}

pub fn change_enforcer_is_blocked(
  connection: &Connection, 
  username: &OperatingSystemUsername,
  new_value: bool,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::change_enforcer_is_blocked(&mut code, username, new_value);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.ChangeEnforcerIsBlocked: \nCode: {code}. \nError: {error}.");
  })
}

pub fn create_rule(
  connection: &Connection, 
  rule: &Rule,
  username: &OperatingSystemUsername,
  position: usize,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::create_rule(&mut code, rule, username, position);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.CreateRule: \nCode: {code}. \nError: {error}.");
  })
}

pub fn delete_rule(
  connection: &Connection, 
  rule_id: &Uuid,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::delete_rule(&mut code, rule_id);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.DeleteRule: \nCode: {code}. \nError: {error}.");
  })
}

pub fn find_all_rules(db: &Connection) -> Result<Vec<RuleNormalized>, ()> {
  let mut rules = Vec::new();

  let mut code = String::new();
  database_code::find_all_rules(&mut code);
  
  let mut statement = match db.prepare(&code) {
    Ok(value) => {
      value
    }
    Err(error) => {
      eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllRules.PrepareStatement: \nCode: {code}. \nError: {error}.");
      return Err(());
    }
  };

  let mut iterator = match statement.query(()) {
    Ok(value) => {
      value
    }
    Err(error) => {
      eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllRules.Query: \nCode: {code}. \nError: {error}.");
      return Err(())
    }
  };

  loop {
    let item = match iterator.next() {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllRules.Query.Iterator.Next: \nCode: {code}. \nError: {error}.");
        return Err(());
      }
    };

    let Some(item) = item else {
      break;
    };

    let item = match deserialize(item) {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllRules.Deserialize: \nCode: {code}. \nError: {:?}.", error);
        return Err(());
      }
    };

    rules.push(item);
  }

  Ok(rules)
}

pub fn find_all_enforcers(db: &Connection) -> Result<Vec<EnforcerNormalized>, ()> {
  let mut rules = Vec::new();

  let mut code = String::new();
  database_code::find_all_enforcers(&mut code);
  
  let mut statement = match db.prepare(&code) {
    Ok(value) => {
      value
    }
    Err(error) => {
      eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllEnforcers.PrepareStatement: \nCode: {code}. \nError: {error}.");
      return Err(());
    }
  };

  let mut iterator = match statement.query(()) {
    Ok(value) => {
      value
    }
    Err(error) => {
      eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllEnforcers.Query: \nCode: {code}. \nError: {error}.");
      return Err(())
    }
  };

  loop {
    let item = match iterator.next() {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllEnforcers.Query.Iterator.Next: \nCode: {code}. \nError: {error}.");
        return Err(());
      }
    };

    let Some(item) = item else {
      break;
    };

    let item = match deserialize(item) {
      Ok(value) => {
        value
      }
      Err(error) => {
        eprintln!("Discipline.NetworkAccessEnforcer.DatabaseProcedures.FindAllEnforcers.Deserialize: \nCode: {code}. \nError: {:?}.", error);
        return Err(());
      }
    };

    rules.push(item);
  }

  Ok(rules)
}

pub fn load_enforcers_feature(
  db: &Connection,
  feature_intermediate_repr: FeatureNormalized,
) -> Result<Feature, ()> {
  let mut rule_intermediate_reprs = find_all_rules(db)?;
  rule_intermediate_reprs.sort_by(|a, b| a.position.cmp(&b.position));

  let enforcer_intermediate_reprs = find_all_enforcers(db)?;
  let mut enforcers = Vec::new();

  for enforcer_intermediate_repr in enforcer_intermediate_reprs {
    let mut rules = Vec::new();

    for rule_intermediate_repr in &rule_intermediate_reprs {
      if rule_intermediate_repr.username == enforcer_intermediate_repr.username {
        rules.push(rule_intermediate_repr.clone().finalize());
      }
    }

    enforcers.push(enforcer_intermediate_repr.finalize(rules));
  }

  Ok(Feature {
    enforcers,
    enforcing_interval: feature_intermediate_repr.enforcing_interval,
  })
}

pub fn activator_for_duration_change_remaining_duration(
  connection: &Connection,
  rule_id: &Uuid,
  new_remaining_duration: &Duration,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::activator_for_duration_change_remaining_duration(&mut code, rule_id, new_remaining_duration);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Activator.ForDuration.ChangeRemainingDuration: \nCode: {code}. \nError: {error}.");
  })
}

pub fn activator_in_time_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_value: &TimeRange,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::activator_in_time_range_replace(&mut code, rule_id, new_value);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Activator.InTimeRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn activator_not_in_time_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_value: &TimeRange,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::activator_not_in_time_range_replace(&mut code, rule_id, new_value);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Activator.NotInTimeRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn activator_in_weekday_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_weekday_range: &WeekdayRange,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::activator_in_weekday_range_replace(&mut code, rule_id, new_weekday_range);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Activator.InWeekdayRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn activator_not_in_weekday_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_value: &WeekdayRange
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::activator_not_in_weekday_range_replace(&mut code, rule_id, new_value);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Activator.NotInWeekdayRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enabler_for_duration_change_remaining_duration(
  connection: &Connection,
  rule_id: &Uuid,
  new_remaining_duration: &Duration
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enabler_for_duration_change_remaining_duration(&mut code, rule_id, new_remaining_duration);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Enabler.ForDuration.ChangeRemainingDuration: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enabler_in_time_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_from: &Time,
  new_till: &Time,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enabler_in_time_range_replace(&mut code, rule_id, new_from, new_till);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Enabler.InTimeRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enabler_not_in_time_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_from: &Time,
  new_till: &Time,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enabler_not_in_time_range_replace(&mut code, rule_id, new_from, new_till);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Enabler.NotInTimeRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enabler_in_weekday_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_from: &Weekday,
  new_till: &Weekday,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enabler_in_weekday_range_replace(&mut code, rule_id, new_from, new_till);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Enabler.InWeekdayRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enabler_not_in_weekday_range_replace(
  connection: &Connection,
  rule_id: &Uuid,
  new_from: &Weekday,
  new_till: &Weekday,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enabler_not_in_weekday_range_replace(&mut code, rule_id, new_from, new_till);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Enabler.NotInWeekdayRange.Replace: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enabler_by_password_make_effective(
  connection: &Connection,
  rule_id: &Uuid,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enabler_by_password_make_effective(&mut code, rule_id);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Enabler.ByPassword.MakeEffective: \nCode: {code}. \nError: {error}.");
  })
}

pub fn enabler_by_password_make_ineffective(
  connection: &Connection,
  rule_id: &Uuid,
) -> Result<(), ()> {
  let mut code = String::new();
  database_code::enabler_by_password_make_ineffective(&mut code, rule_id);
  connection.execute_batch(&code).map_err(|error| {
    eprintln!("Discipline.NetworkingAccessEnforcer.DatabaseProcedures.Enabler.ByPassword.MakeIneffective: \nCode: {code}. \nError: {error}.");
  })
}