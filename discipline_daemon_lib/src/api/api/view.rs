// use std::fmt::Write;
// use crate::{Condition, DateTime, NetworkAccessConfig, Rule, ScreenAccessConfig, UserAccessConfig};

// enum RuleConditionLocation {
//   ActiveCondition,
//   GuardedCondition,
// }

// fn view_condition(rule: &mut Rule, condition_location: RuleConditionLocation, into: &mut impl Write) {
//   writeln!(into, "<div>").unwrap();

//   let now = DateTime::now();

//   let condition = match condition_location {
//     RuleConditionLocation::ActiveCondition => &mut rule.active_condition,
//     RuleConditionLocation::GuardedCondition => &mut rule.guarded_condition,
//   };

//   match condition {
//     Condition::Never(_) => {
//       writeln!(into, "<p>status: inactive</p>").unwrap();
//       writeln!(into, "<p>condition: never active</p>").unwrap();
//     }
//     Condition::Always(_) => {
//       writeln!(into, "<p>status: active</p>").unwrap();
//       writeln!(into, "<p>condition: always active</p>").unwrap();
//     }
//     Condition::ForDuration(condition) => {
//       condition.synchronize(&now);
      
//       let status = if condition.evaluate() {
//         "active"
//       } else {
//         "inactive"
//       };

//       writeln!(into, "<p>status: {status}</p>").unwrap();

//       let remaining_duration = condition.remaining_duration.to_string();
//       writeln!(into, "<p>condition: active for {remaining_duration}</p>").unwrap();

//       if rule.is_fully_blocker() {
//         writeln!(into, "<p><a href=\"\">Increase by five minutes</a></p>").unwrap();
//       } else if rule.is_fully_allower() {
//         writeln!(into, "<p><a href=\"\">Decrease by five minutes</a></p>").unwrap();
//       }
//     }
//     Condition::ForTimeRange(condition) => {
//       let status = if condition.evaluate(&now) {
//         "active"
//       } else {
//         "inactive"
//       };

//       writeln!(into, "<p>status: {status}</p>").unwrap();

//       let min = condition.min.to_12_hour_based_string_with_period();
//       let max = condition.max.to_12_hour_based_string_with_period();
//       writeln!(into, "<p>condition: active within {min} .. {max}</p>").unwrap();
      
//       // TODO: Create actions make wider and make narrower.
//     }
//     Condition::ForHourRange(condition) => {
//       let status = if condition.evaluate(&now) {
//         "active"
//       } else {
//         "inactive"
//       };

//       writeln!(into, "<p>status: {status}</p>").unwrap();

//       let min = condition.min.to_12_based_string_with_period();
//       let max = condition.max.to_12_based_string_with_period();
//       writeln!(into, "<p>condition: active within hours {min} .. {max}</p>").unwrap();
      
//       // TODO: Create actions make wider and make narrower.
//     }
//     Condition::ForMinuteRange(condition) => {
//       let status = if condition.evaluate(&now) {
//         "active"
//       } else {
//         "inactive"
//       };

//       writeln!(into, "<p>status: {status}</p>").unwrap();

//       let min = condition.min.value0();
//       let max = condition.max.value0();
//       writeln!(into, "<p>condition: active within minutes {min} .. {max}</p>").unwrap();
      
//       // TODO: Create actions make wider and make narrower.
//     }
//     Condition::ForWeekdayRange(condition) => {
//       let status = if condition.evaluate(&now) {
//         "active"
//       } else {
//         "inactive"
//       };

//       writeln!(into, "<p>status: {status}</p>").unwrap();

//       let min = condition.min.to_string();
//       let max = condition.max.to_string();
//       writeln!(into, "<p>condition: active within weekdays {min} .. {max}</p>").unwrap();
      
//       // TODO: Create actions make wider and make narrower.
//     }
//     Condition::ByPasswordAuthentication(condition) => {
//       let status = if condition.evaluate() {
//         "active"
//       } else {
//         "inactive"
//       };

//       writeln!(into, "<p>status: {status}</p>").unwrap();
//       writeln!(into, "<p>condition: always active until password is enterd.</p>").unwrap();
//       if condition.evaluate() {
//         writeln!(into, "<p><a href=\"\"></a>activate</p>").unwrap();
//         writeln!(into, "
//           <p>
//             <form action=\"/change-password\" method=\"post\">
//               <label for=\"password\">Change Password:</label>
//               <input type=\"password\" id=\"password\" name=\"password\">
//               <button type=\"submit\">Submit</button>
//             </form>
//           </p>
//         ").unwrap();
//       } else {
//         writeln!(into, "
//           <p>
//             <form action=\"/inactivate\" method=\"POST\">
//               <label for=\"password\">deactivate:</label>
//               <input type=\"password\" id=\"password\" name=\"password\">
//               <button type=\"submit\">Submit</button>
//             </form>
//           </p>
//         ").unwrap();
//       }
//     }
//   }

//   writeln!(into, "</div>").unwrap();
// }

// fn view_rule_users(rule: &mut Rule, into: &mut impl Write) {
//   writeln!(into, "<div>").unwrap();

//   if rule.users.wildcard {
//     writeln!(into, "<p>rule applies to all users</p>").unwrap();
    
//     if rule.is_fully_allower() {
//       writeln!(into, "<p><a href=\"\">apply only to specific users</a></p>").unwrap();
//     }

//     return;
//   }

//   writeln!(into, "<p>rule applies to these users:</p>").unwrap();

//   writeln!(into, "<ul>").unwrap();
//   for user in &rule.users.users {
//     if rule.is_fully_allower() {
//       writeln!(into, "
//         <li>
//           <p>{user}</p>
//           <p><a href=\"\">remove</a></p>
//         </li>
//       ").unwrap();
//     } else {
//       writeln!(into, "<li>{user}</li>").unwrap();
//     }
//   }
//   writeln!(into, "</ul>").unwrap();

//   if rule.is_fully_blocker() {
//     writeln!(into, "<p><a href=\"\">apply only to all users</a></p>").unwrap();
//   }

//   writeln!(into, "</div>").unwrap();
// }

// fn view_user_access_config(rule: &mut Rule, into: &mut impl Write) {
//   let now = DateTime::now();

//   writeln!(into, "<div>").unwrap();

//   match rule.user_access_config {
//     None => {
//       writeln!(into, "<p>action: none</p>").unwrap();
//       writeln!(into, "<p>description: this rule neither allow nor block access to effected users</p>").unwrap();
//       writeln!(into, "<p><a href=\"\">change to block</a></p>").unwrap();
//       if !rule.guarded(&now) {
//         writeln!(into, "<p><a href=\"\">change to allow</a></p>").unwrap();
//       }
//     }
//     Some(UserAccessConfig::Allow) => {
//       writeln!(into, "<p>action: allow</p>").unwrap();
//       writeln!(into, "<p>description: when this rule is active, accessing the effected users is allowed</p>").unwrap();
//       writeln!(into, "<p><a href=\"\">change to block</a></p>").unwrap();
//     }
//     Some(UserAccessConfig::Block) => {
//       writeln!(into, "<p>action: block</p>").unwrap();
//       writeln!(into, "<p>description: when this rule is active, accessing the effected users is blocked</p>").unwrap();
//       if !rule.guarded(&now) {
//         writeln!(into, "<p><a href=\"\">change to allow</a></p>").unwrap();
//       }
//     }
//   }
// }

// fn view_screen_access_config(rule: &mut Rule, into: &mut impl Write) {
//   let now = DateTime::now();

//   writeln!(into, "<div>").unwrap();

//   match rule.screen_access_config {
//     None => {
//       writeln!(into, "<p>action: none</p>").unwrap();
//       writeln!(into, "<p>description: this rule neither allow nor block access the device</p>").unwrap();
//       writeln!(into, "<p><a href=\"\">change to shutdown</a></p>").unwrap();
//       if !rule.guarded(&now) {
//         writeln!(into, "<p><a href=\"\">change to allow</a></p>").unwrap();
//       }
//     }
//     Some(ScreenAccessConfig::Allow) => {
//       writeln!(into, "<p>action: allow</p>").unwrap();
//       writeln!(into, "<p>description: when this rule is active, effected users are allowed to access the device</p>").unwrap();
//       writeln!(into, "<p><a href=\"\">change to shutdown</a></p>").unwrap();
//     }
//     Some(ScreenAccessConfig::Shutdown) => {
//       writeln!(into, "<p>action: shutdown</p>").unwrap();
//       writeln!(into, "<p>description: when this rule is active, effected users are blocked from accesing the device, that is done by shutting the device down when one of them is signed in</p>").unwrap();
//       if !rule.guarded(&now) {
//         writeln!(into, "<p><a href=\"\">change to allow</a></p>").unwrap();
//       }
//     }
//   }
// }

// fn view_network_access_config(rule: &mut Rule, into: &mut impl Write) {
//   let now = DateTime::now();

//   writeln!(into, "<div>").unwrap();

//   match rule.network_access_config {
//     None => {
//       writeln!(into, "<p>action: none</p>").unwrap();
//       writeln!(into, "<p>description: this rule neither allow nor block effected users from accessing the internet</p>").unwrap();
//       writeln!(into, "<p><a href=\"\">change to block</a></p>").unwrap();
//       if !rule.guarded(&now) {
//         writeln!(into, "<p><a href=\"\">change to allow</a></p>").unwrap();
//       }
//     }
//     Some(NetworkAccessConfig::Allow) => {
//       writeln!(into, "<p>action: allow</p>").unwrap();
//       writeln!(into, "<p>description: when this rule is active, effected users are allowed to access the internet</p>").unwrap();
//       writeln!(into, "<p><a href=\"\">change to block</a></p>").unwrap();
//     }
//     Some(NetworkAccessConfig::Block) => {
//       writeln!(into, "<p>action: block</p>").unwrap();
//       writeln!(into, "<p>description: when this rule is active, effected users are blocked from accessing the internet</p>").unwrap();
//       if !rule.guarded(&now) {
//         writeln!(into, "<p><a href=\"\">change to allow</a></p>").unwrap();
//       }
//     }
//   }
// }

// pub fn view_rule(rule: &mut Rule, into: &mut impl Write) {
//   writeln!(into, "<div>").unwrap();

//   writeln!(into, "<p>name: {}</p>", rule.name).unwrap();
//   writeln!(into, "<p>description: {}</p>", rule.description).unwrap();
//   writeln!(into, "<p>created at: {}</p>", rule.created_at.to_iso_8601_like()).unwrap();

//   writeln!(into, "<p>active condition</p>").unwrap();
//   writeln!(into, "<section>").unwrap();
//   view_condition(rule, RuleConditionLocation::ActiveCondition, into);
//   writeln!(into, "</section>").unwrap();

//   writeln!(into, "<p>guarded condition</p>").unwrap();
//   writeln!(into, "<section>").unwrap();
//   view_condition(rule, RuleConditionLocation::GuardedCondition, into);
//   writeln!(into, "</section>").unwrap();

//   writeln!(into, "<p>users</p>").unwrap();
//   writeln!(into, "<section>").unwrap();
//   view_rule_users(rule, into);
//   writeln!(into, "</section>").unwrap();

//   writeln!(into, "<p>user access</p>").unwrap();
//   writeln!(into, "<section>").unwrap();
//   view_user_access_config(rule, into);
//   writeln!(into, "</section>").unwrap();

//   writeln!(into, "<p>screen access</p>").unwrap();
//   writeln!(into, "<section>").unwrap();
//   view_screen_access_config(rule, into);
//   writeln!(into, "</section>").unwrap();

//   writeln!(into, "<p>network access</p>").unwrap();
//   writeln!(into, "<section>").unwrap();
//   view_network_access_config(rule, into);
//   writeln!(into, "</section>").unwrap();

//   writeln!(into, "</div>").unwrap();
// }

// pub fn view_create_rule(into: &mut impl Write) {
//   write
// }
// pub fn view_rules(rules: &mut Vec<Rule>, into: &mut impl Write) {
  
// }