mod screen_access;

mod scalar_value_serialization;
use scalar_value_serialization::*;

mod scalar_value_deserialization;
use scalar_value_deserialization::*;

mod compound_value_serialization;
use compound_value_serialization::*;

mod compound_value_deserialization;
use compound_value_deserialization::*;

mod chronic_types;
use crate::Weekday;

mod implementation;

pub struct Schema {
  app_collection: String,
  app_id: String,
  app_user_screen_access_regulation_appling_interval: String,
  app_user_screen_access_regulation_private_password: String,

  user_screen_access_rule_collection: String,
  user_screen_access_rule_id: String,
  user_screen_access_rule_activator_variant: String,
  user_screen_access_rule_activator_a: String,
  user_screen_access_rule_activator_b: String,

  user_screen_access_poilcy_collection: String,
  user_screen_access_poilcy_id: String,
  user_screen_access_poilcy_name: String,
  user_screen_access_poilcy_creation_time: String,
  user_screen_access_poilcy_enabled_countdown_timer_duration: String,
  user_screen_access_poilcy_enabled_countdown_timer_remaining_duration: String,
  user_screen_access_poilcy_enabled_countdown_timer_previous_synchronization_time: String,

  user_collection: String,
  user_id: String,
  user_name: String,
  user_operating_system_user_id: String,
  user_operating_system_user_name: String,
  user_operating_system_user_password: String,
  user_screen_access_regulator_is_applying_enabled: String,
  user_screen_access_regulator_is_user_screen_access_blocked: String,
}

pub fn generate_code_define_app_collection(
  code: &mut String,
  schema: &Schema,
) {
  code.push_str("CREATE TABLE IF NOT EXISTS ");
  code.push_str(&schema.app_collection);
  code.push_str(" (");
  code.push_str(&schema.app_id);
  code.push_str(" INTEGER PRIMARY KEY, ");
  code.push_str(&schema.app_user_screen_access_regulation_appling_interval);
  code.push_str(" INTEGER NOT NULL, ");
  code.push_str(&schema.app_user_screen_access_regulation_private_password);
  code.push_str(" TEXT NOT NULL) STRICT, WITHOUT ROWID;");
}

pub fn define_user_collection(
  code: &mut String,
  schema: &Schema,
) {
  code.push_str("CREATE TABLE IF NOT EXISTS ");
  code.push_str(&schema.user_collection);
  code.push_str(" (");
  code.push_str(&schema.user_id);
  code.push_str(" TEXT PRIMARY KEY, ");
  code.push_str(&schema.user_name);
  code.push_str(" TEXT NOT NULL, ");
  code.push_str(&schema.user_operating_system_user_id);
  code.push_str(" INTEGER NOT NULL, ");
  code.push_str(&schema.user_operating_system_user_name);
  code.push_str(" TEXT NOT NULL, ");
  code.push_str(&schema.user_operating_system_user_password);
  code.push_str(" TEXT NOT NULL, ");
  code.push_str(&schema.user_screen_access_regulator_is_applying_enabled);
  code.push_str(" INTEGER NOT NULL, ");
  code.push_str(&schema.user_screen_access_regulator_is_user_screen_access_blocked);
  code.push_str(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
}

pub struct SerialzableCompoundValueContext {

}

pub trait SerialzableCompoundValue {
  fn serialize(&self, code: &mut String);
}

pub fn generate_code_add_collection_item<Item>(
  code: &mut String,
  collection: &String,
  item: &impl SerialzableCompoundValue,
) {
  code.push_str("INSERT INTO ");
  code.push_str(&collection);
  code.push_str(" ");
  item.serialize(code);
  code.push_str(";");
}

pub fn generate_code_delete_collection_item_matching_1_field(
  code: &mut String,
  collection: &String,
  field_1: &String,
  field_1_value: &impl SerializableScalarValue,
) {
  code.push_str("DELETE FROM ");
  code.push_str(collection);
  code.push_str(" WHERE ");

  code.push_str(field_1);
  code.push_str(" = ");
  field_1_value.serialize(code);

  code.push_str(";");
}

pub fn generate_code_delete_collection_item_matching_2_fields(
  code: &mut String,
  collection: &String,
  field_1: &String,
  field_1_value: &impl SerializableScalarValue,
  field_2: &String,
  field_2_value: &impl SerializableScalarValue,
) {
  code.push_str("DELETE FROM ");
  code.push_str(collection);
  code.push_str(" WHERE ");

  code.push_str(field_1);
  code.push_str(" = ");
  field_1_value.serialize(code);

  code.push_str(" AND ");

  code.push_str(field_2);
  code.push_str(" = ");
  field_2_value.serialize(code);

  code.push_str(";");
}

pub fn generate_code_delete_collection_item_matching_3_fields(
  code: &mut String,
  collection: &String,
  field_1: &String,
  field_1_value: &impl SerializableScalarValue,
  field_2: &String,
  field_2_value: &impl SerializableScalarValue,
  field_3: &String,
  field_3_value: &impl SerializableScalarValue,
) {
  code.push_str("DELETE FROM ");
  code.push_str(collection);
  code.push_str(" WHERE ");

  code.push_str(field_1);
  code.push_str(" = ");
  field_1_value.serialize(code);

  code.push_str(" AND ");
  
  code.push_str(field_2);
  code.push_str(" = ");
  field_2_value.serialize(code);

  code.push_str(" AND ");

  code.push_str(field_3);
  code.push_str(" = ");
  field_3_value.serialize(code);

  code.push_str(";");
}

pub struct CollectionItemUpdates {
  code: String,
}

impl CollectionItemUpdates {
  pub fn update(&mut self, field: &String, value: &impl SerializableScalarValue) {
    if self.code.len() > 0 {
      self.code.push_str(", ");
    }

    self.code.push_str(field);
    self.code.push_str(" = ");
    value.serialize(&mut self.code);
  }
}