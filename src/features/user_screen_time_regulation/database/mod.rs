use crate::countdown_timer::database_serde::Adapter as CountdownTimerAdapter;

use crate::database::{generate_create_row_statement, generate_delete_where_column_statement, generate_ensure_table_created_statement, generate_update_column_where_column_statement, generate_update_where_column_statement_given_set_clause, Connection, Table, UpdateStatementSetClause};
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

mod rule_deactivator;
use rule_deactivator::RuleDeactivatorAdapter;

mod rule;
use rule::{RuleAdapter, RuleNormalized, RuleSerializer};

mod rules;
use rules::RuleTableAdapter;

mod enforcer;
use enforcer::{EnforcerAdapter, NormalizedEnforcer};

mod enforcers;
use enforcers::EnforcersAdapter;

mod feature;
pub use feature::{FeatureAdapter, NormalizedFeature};