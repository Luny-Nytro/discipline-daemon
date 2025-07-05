use super::*;

pub fn define_user_screen_access_policy_collection(
  code: &mut String,
  schema: &Schema,
) {
  code.push_str("CREATE TABLE IF NOT EXISTS "); 
  code.push_str(&schema.user_screen_access_poilcy_collection); 
  code.push_str(" (");
  code.push_str(&schema.user_screen_access_poilcy_id);
  code.push_str(" TEXT PRIMARY KEY, ");
  code.push_str(&schema.user_screen_access_poilcy_name);
  code.push_str(" TEXT NOT NULL, ");
  code.push_str(&schema.user_screen_access_poilcy_enabled_countdown_timer_duration);
  code.push_str(" INTEGER NOT NULL, ");
  code.push_str(&schema.user_screen_access_poilcy_enabled_countdown_timer_remaining_duration);
  code.push_str(" INTEGER NOT NULL, ");
  code.push_str(&schema.user_screen_access_poilcy_enabled_countdown_timer_previous_synchronization_time);
  code.push_str(" INTEGER NOT NULL) STRICT, WITHOUT ROWID;");
}

pub fn define_user_screen_access_rule_collection(
  code: &mut String,
  schema: &Schema,
) {
  code.push_str("CREATE TABLE IF NOT EXISTS "); 
  code.push_str(&schema.user_screen_access_rule_collection); 
  code.push_str(" (");
  code.push_str(&schema.user_screen_access_rule_id);
  code.push_str(" TEXT PRIMARY KEY, ");
  code.push_str(&schema.user_screen_access_rule_activator_variant);
  code.push_str(" INTEGER NOT NULL, ");
  code.push_str(&schema.user_screen_access_rule_activator_a);
  code.push_str(", ");
  code.push_str(&schema.user_screen_access_rule_activator_b);
  code.push_str(") WITHOUT ROWID;");
}

impl Schema {
  fn screen_access_rule_activator_variant(&self) -> &String {
    &self.user_screen_access_rule_activator_variant
  }
  fn screen_access_rule_activator_time_range_from(&self) -> &String {
    &self.user_screen_access_rule_activator_a
  }
  fn screen_access_rule_activator_time_range_till(&self) -> &String {
    &self.user_screen_access_rule_activator_b
  }
  fn screen_access_rule_activator_weekday_range_from(&self) -> &String {
    &self.user_screen_access_rule_activator_a
  }
  fn screen_access_rule_activator_weekday_range_till(&self) -> &String {
    &self.user_screen_access_rule_activator_b
  }
  fn screen_access_rule_activator_weekday(&self) -> &String {
    &self.user_screen_access_rule_activator_a
  }
}

pub struct UserScreenAccessRuleUpdates<'a> {
  updates: CollectionItemUpdates,
  schema: &'a Schema,
}

impl<'a> UserScreenAccessRuleUpdates<'a> {
  pub fn update_activator_weekday(&mut self, new_value: &Weekday) {
    self.updates.update(self.schema.screen_access_rule_activator_weekday(), new_value);
  }

  pub fn update_activator_time_range(&mut self, new_value: &Weekday) {
    self.updates.update(self.schema.screen_access_rule_activator_weekday(), new_value);
  }
}
