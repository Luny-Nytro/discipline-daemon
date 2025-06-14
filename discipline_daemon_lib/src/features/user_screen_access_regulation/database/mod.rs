use crate::database::*;

// use crate::database::{
//   Connection, Table, UpdateStatement, 
//   WriteColumns, WriteColumnsContext,
//   generate_sql_delete_where_3_columns, 
//   generate_sql_insert_row, 
//   generate_sql_delete_where_1_column, 
//   generate_sql_initialize_table,
// ColumnValue, DeserializableScalarValue, SerializableScalarValue, 
// ToSerializableScalarValue,
// };

use crate::{
  Duration, GenericError, time_range, 
  weekday_range, Uuid
};

use crate::countdown_timer::database::Specification as CountdownTimerSpecification;

use super::{
  RuleActivator, PolicyEnabler, 
  OperatingSystemCalls, Policy, PolicyName,
  Regulator, CommonInfo, Rule,
};

mod rule_activator_variant;
pub use rule_activator_variant::RuleActivatorVariant;

mod rule_activator;
pub use rule_activator::RuleActivatorSpecification;

mod policy_enabler;
pub use policy_enabler::PolicyEnablerSpecification;

mod rule;
pub use rule::{RuleSpecification, NormalizedRule, RuleSerializer};

mod regulator;
pub use regulator::{RegulatorSpecification, NormalizedRegulator};

mod common_info;
pub use common_info::CommonInfoSpecification;

mod policy;
pub use policy::*;

mod policy_name;
mod feature;
pub use feature::Specification;