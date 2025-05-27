use crate::countdown_timer::database_serde::Schema as CountdownTimerAdapter;

use crate::database::{
  generate_sql_delete_where_3_columns, UpdateStatement, 
  InitializeTableStatement, WriteColumns, WriteColumnsContext,
  generate_sql_insert_row, generate_sql_delete_where_1_column, generate_sql_initialize_table, generate_update_column_where_column_statement, generate_update_where_column_statement_given_set_clause, Connection, Table, UpdateStatementSetClause};
use crate::{Duration, DateTime, GenericError, TimeRange, time_range, weekday_range, Uuid, WeekdayRange};
use super::{Regulator, CommonInfo, Rule};
use crate::database::*;
use super::{
  RuleActivator, PolicyEnabler, 
  OperatingSystemCalls, Policy, PolicyName,
};
use crate::{
  OperatingSystemPassword, OperatingSystemUsername, 
  OperatingSystemUserId,
};

use crate::database::{
  ColumnValue, DeserializableScalarValue, SerializableScalarValue, 
  ToSerializableScalarValue,
};


mod rule_activator_variant;
use rule_activator_variant::RuleActivatorVariant;

mod rule_activator;
use rule_activator::RuleActivatorSchema;

mod policy_enabler;
use policy_enabler::PolicyEnablerSchema;

mod rule;
use rule::{RuleSchema, NormalizedRule, RuleSerializer};

mod rules;
use rules::RuleTableSchema;

mod regulator;
pub use regulator::{RegulatorSchema, NormalizedRegulator};

mod common_info;
pub use common_info::{CommonInfoSchema, NormalizedFeature};

mod policy;
pub use policy::*;

mod policies;
pub use policies::*;