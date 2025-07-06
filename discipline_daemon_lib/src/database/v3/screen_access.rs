use super::*;

pub fn define_user_screen_access_policy_collection(
  code: &mut String,
  schema: &Schema,
) {}

pub fn define_user_screen_access_rule_collection(
  code: &mut String,
  schema: &Schema,
) {
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
    self.updates.write_update(self.schema.screen_access_rule_activator_weekday(), new_value);
  }

  pub fn update_activator_time_range(&mut self, new_value: &Weekday) {
    self.updates.write_update(self.schema.screen_access_rule_activator_weekday(), new_value);
  }
}
