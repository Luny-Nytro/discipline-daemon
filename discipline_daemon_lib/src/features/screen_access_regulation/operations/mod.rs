use serde::{
  Serialize, Deserialize,
};

use crate::database::screen_access_regulation_policy_collection as policy_db;
use crate::database::screen_access_regulation_rule_integration as rule_db;
use crate::database::user_collection as user_db;
// use crate::database::app_collection as app_db;

use crate::{
  Database,
  Daemon, DateTime, Uuid, Duration, WeekdayRange, 
  TimeRange, IsRemoteProcedureCall, IntoPublic,
};

use crate::screen_access_regulation::{
  PolicyName, PolicyCreator, PolicyPublicRepr,
  RulePublicRepr, RuleCreator, RuleActivator, Regulation,
};

// pub mod change_is_regulation_enabled;
// pub mod change_rule_activator_time_range;
// pub mod change_rule_activator_weekday_range;
// pub mod create_rule;
// pub mod delete_rule;
// pub mod create_policy;
// pub mod delete_policy;
// pub mod increase_policy_enabled_duration;
// pub mod change_policy_name;

mod x;
