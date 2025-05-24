use crate::countdown_timer::database_serde::Adapter as CountdownTimerAdapter;

use crate::database::{
  generate_sql_delete_where_3_columns, UpdateStatement,
  generate_create_row_statement, generate_sql_where_1_column, generate_sql_initialize_table, generate_update_column_where_column_statement, generate_update_where_column_statement_given_set_clause, Connection, Table, UpdateStatementSetClause};
use crate::{Duration, GenericError, TimeRange, Uuid, WeekdayRange};
use super::{Regulator, CommonInfo, Rule};
use crate::database::*;
use super::{
  RuleActivator, PolicyEnabler, 
  OperatingSystemCalls,
};
use crate::{
  OperatingSystemPassword, OperatingSystemUsername, 
  OperatingSystemUserId,
};

mod rule_activator_variant;
use rule_activator_variant::RuleActivatorVariant;

mod rule_activator;
use rule_activator::RuleActivatorSchema;

mod policy_enabler;
use policy_enabler::RuleDeactivatorAdapter;

mod rule;
use rule::{RuleSchema, NormalizedRule, RuleSerializer};

mod rules;
use rules::RuleTableSchema;

mod enforcer;
use enforcer::{EnforcerAdapter, NormalizedEnforcer};

mod enforcers;
use enforcers::EnforcersAdapter;

mod feature;
pub use feature::{FeatureAdapter, NormalizedFeature};