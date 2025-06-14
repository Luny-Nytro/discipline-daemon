use crate::GenericError;

mod serializing_utiliites;
use serializing_utiliites::escape_string_for_sqilte_into;

mod compound_type_serialization;
pub use compound_type_serialization::*;

mod specifications;
pub use specifications::*;

mod scalar_value_deserialization;
pub use scalar_value_deserialization::*;

mod scalar_value_serialization;
pub use scalar_value_serialization::*;

mod compound_type_deserialization;
pub use compound_type_deserialization::*;

mod actions;
pub use actions::*;

mod database_connection;
pub use database_connection::Database;

mod database_modifications_draft;
pub use database_modifications_draft::*;

mod collection_item_modifications_draft;
pub use collection_item_modifications_draft::*;

mod collection_item_matcher;
pub use collection_item_matcher::*;

mod sql_code_generators;
pub use sql_code_generators::*;

mod namespace;
pub use namespace::*;

mod global_namespace;
pub use global_namespace::*;

mod collection;
pub use collection::*;

mod identifier;
pub use identifier::*;

mod compound_type_specification;
pub use compound_type_specification::*;

mod collection_item_specification;
pub use collection_item_specification::*;