// use std::fmt::Write;
// use uuid::Uuid;

// use crate::{app::OperatingSystemUsername, DateTime};
// use super::{Feature, Activator, Action};

// pub struct UI {
//   html: String
// }

// impl UI {
//   pub fn attr(&mut self, attribute_name: &str, attribute_value: &str) -> &mut Self {
//     write!(self.html, " {attribute_name}=\"{attribute_value}\"").unwrap();
//     self
//   }

//   pub fn text(&mut self, text: &str) -> &mut Self {
//     self.html.push_str(text);
//     self
//   }

//   pub fn open_div(&mut self) -> &mut Self {
//     self.html.push_str("<div>");
//     self
//   }
  
//   pub fn close_div(&mut self) -> &mut Self {
//     self.html.push_str("</div>");
//     self
//   }

//   pub fn open_p(&mut self) -> &mut Self {
//     self.html.push_str("<p>");
//     self
//   }

//   pub fn close_p(&mut self) -> &mut Self {
//     self.html.push_str("</p>");
//     self
//   }

//   pub fn open_a(&mut self) -> &mut Self {
//     self.html.push_str("<a>");
//     self
//   }

//   pub fn close_a(&mut self) -> &mut Self {
//     self.html.push_str("</a>");
//     self
//   }

//   pub fn open_li(&mut self) -> &mut Self {
//     self.html.push_str("<li>");
//     self
//   }

//   pub fn close_li(&mut self) -> &mut Self {
//     self.html.push_str("</li>");
//     self
//   }

//   pub fn open_ul(&mut self) -> &mut Self {
//     self.html.push_str("<ul>");
//     self
//   }

//   pub fn close_ul(&mut self) -> &mut Self {
//     self.html.push_str("</ul>");
//     self
//   }

//   pub fn open_h1(&mut self) -> &mut Self {
//     self.html.push_str("<h1>");
//     self
//   }

//   pub fn close_h1(&mut self) -> &mut Self {
//     self.html.push_str("</h1>");
//     self
//   }
//   pub fn open_h2(&mut self) -> &mut Self {
//     self.html.push_str("<h2>");
//     self
//   }
//   pub fn close_h2(&mut self) -> &mut Self {
//     self.html.push_str("</h2>");
//     self
//   }
//   pub fn open_h3(&mut self) -> &mut Self {
//     self.html.push_str("<h3>");
//     self
//   }
//   pub fn close_h3(&mut self) -> &mut Self {
//     self.html.push_str("</h3>");
//     self
//   }
//   pub fn open_h4(&mut self) -> &mut Self {
//     self.html.push_str("<h4>");
//     self
//   }
//   pub fn close_h4(&mut self) -> &mut Self {
//     self.html.push_str("</h4>");
//     self
//   }
//   pub fn open_h5(&mut self) -> &mut Self {
//     self.html.push_str("<h5>");
//     self
//   }
//   pub fn close_h5(&mut self) -> &mut Self {
//     self.html.push_str("</h5>");
//     self
//   }
//   pub fn open_h6(&mut self) -> &mut Self {
//     self.html.push_str("<h6>");
//     self
//   }
//   pub fn close_h6(&mut self) -> &mut Self {
//     self.html.push_str("</h6>");
//     self
//   }
//   pub fn open_span(&mut self) -> &mut Self {
//     self.html.push_str("<span>");
//     self
//   }
//   pub fn close_span(&mut self) -> &mut Self {
//     self.html.push_str("</span>");
//     self
//   }  
// }


// pub fn user_access_feature_view(ui: &mut UI, feature: &Feature) {
//   ui
//     .open_h1()
//       .text("User Access Management")
//     .close_h1()
//     .open_p()
//       .text("Regulated Users")
//     .close_p()
//     .open_ul();

//   for enforcer in &feature.enforcers {
//     let user = enforcer.username.as_ref();

//     ui
//       .open_li()
//         .open_p()
//           .open_a()
//             .attr("href", &format!("/UserAccess?user={user}"))
//             .text(&user)
//           .close_a()
//         .close_p()
//       .close_li();
//   }

//   ui.close_ul();
// }

// pub fn enforcer_view(ui: &mut UI, feature: &mut Feature, username: &OperatingSystemUsername) {
//   let username_as_str = username.as_ref();
//   let enforcer = match feature.enforcers.iter_mut().find(|enforcer| enforcer.username == *username) {
//     Some(value) => {
//       value
//     }
//     None => {
//       ui
//       .open_h1()
//         .text("No Access Enforcer for ")
//         .text(username_as_str)
//         .text(" exists")
//       .close_h1();
//       return;
//     }
//   };

//   let now = DateTime::now();

//   ui
//     .open_h1()
//       .text("user access enforcer for ")
//       .text(&username_as_str)
//     .close_h1()
//     .open_p()
//       .text("enforcer.is_enabled")
//   writeln!(ui, "<h1>user access enforcer for {username_as_str}</h1>").unwrap();
//   writeln!(ui, "<p>enforcer is enabled: {}</p>", enforcer.is_enabled).unwrap();
//   writeln!(ui, "<p>enforcer is protected: {}</p>", enforcer.is_protected(now)).unwrap();
//   writeln!(ui, "<p>user is locked: {}</p>", enforcer.is_blocked).unwrap();
//   writeln!(ui, "<p>user public password: {}</p>", enforcer.public_password.as_ref()).unwrap();
//   writeln!(ui, "<p>rules</p>").unwrap();
//   writeln!(ui, "<ul>").unwrap();
//   for rule in &mut enforcer.rules {
//     let rule_id = rule.id.to_string();

//     writeln!(ui, "<div>").unwrap();
//     writeln!(ui, "<p><a href='/user_access_feature/{username_as_str}/{rule_id}'>").unwrap();

//     match rule.action {
//       Action::Allow => {
//         writeln!(ui, "Allow ").unwrap();
//         serialize_activator(ui, &mut rule.activator);
//       }
//       Action::Block => {
//         writeln!(ui, "Block ").unwrap();
//         serialize_activator(ui, &mut rule.activator);
//       }
//     }

//     writeln!(ui, "</a></p>").unwrap();
//     writeln!(ui, "<p>Activated: {}</p>", rule.is_activated(now)).unwrap();
//     writeln!(ui, "</div>").unwrap();
//   }

//   writeln!(ui, "<ul>").unwrap();
// }

// fn serialize_activator(into: &mut impl Write, activator: &mut Activator) {
//   match activator {
//     Activator::AtHour(hour) => {
//       if hour.value0() < 12 {
//         writeln!(into, "at hour {} AM", hour.value0()).unwrap();
//       } else {
//         writeln!(into, "at hour {} PM", hour.value0() - 12).unwrap();
//       }
//     }
//     Activator::AtWeekday(weekday) => {
//       writeln!(into, "at {}", weekday.to_string()).unwrap();
//     }
//     Activator::CountdownTimer(countdown_timer) => {
//       writeln!(into, "for {}", countdown_timer.duration().to_string()).unwrap();
//     }
//     Activator::InTimeRange(time_range) => {
//       let from = time_range.from().to_12_hour_based_string_with_period();
//       let till = time_range.till().to_12_hour_based_string_with_period();
//       writeln!(into, "within time range {} .. {}", from, till).unwrap();
//     }
//     Activator::InWeekdayRange(weekday_range) => {
//       let from = weekday_range.from().to_string();
//       let till = weekday_range.till().to_string();
//       writeln!(into, "within weekday range {from} .. {till}").unwrap();
//     }
//     Activator::NotAtHour(hour) => {
//       writeln!(into, "all the time except for hour ").unwrap();
//       if hour.value0() < 12 {
//         writeln!(into, "{} AM", hour.value0()).unwrap();
//       } else {
//         writeln!(into, "{} PM", hour.value0() - 12).unwrap();
//       }
//     }
//     Activator::NotAtWeekday(weekday) => {
//       writeln!(into, "all the time except for weekday {}", weekday.to_string()).unwrap();
//     }
//     Activator::NotInTimeRange(time_range) => {
//       let from = time_range.from().to_12_hour_based_string_with_period();
//       let till = time_range.till().to_12_hour_based_string_with_period();
//       writeln!(into, "all the time except in time range {} .. {}", from, till).unwrap();
//     }
//     Activator::NotInWeekdayRange(weekday_range) => {
//       let from = weekday_range.from().to_string();
//       let till = weekday_range.till().to_string();
//       writeln!(into, "all the time except at weekdays {from} .. {till}").unwrap();
//     }
//   }
// }

// pub fn rule_view(
//   into: &mut impl Write, 
//   feature: &mut Feature,
//   username: &OperatingSystemUsername, 
//   rule_id: &Uuid,
// ) {
//   writeln!(into, "<h1>User Access Rule</h1>").unwrap();

//   let enforcer = match feature.enforcers.iter_mut().find(|enforcer| enforcer.username == *username) {
//     Some(value) => {
//       value
//     }
//     None => {
//       writeln!(into, "<p>No enforcer for user {}</p>", username.as_ref()).unwrap();
//       return;
//     }
//   };

//   let rule = match enforcer.rules.iter_mut().find(|rule| rule.id == *rule_id) {
//     Some(value) => {
//       value
//     }
//     None => {
//       writeln!(into, "<p>No rule with this id {}</p>", rule_id.to_string()).unwrap();
//       return;
//     }
//   };

//   let now = DateTime::now();
//   let username = username.as_ref();
//   let id = rule_id.to_string();

//   writeln!(into, "<p>rule activated: {}</p>", rule.is_activated(now)).unwrap();
//   match rule.activator {
//     Activator::AtHour(hour) => {
//       writeln!(into, "<p>rule is active at hour: {}</p>", hour.value0()).unwrap();
//     }
//     Activator::AtWeekday(weekday) => {
//       writeln!(into, "<p>rule is active at weekday: {}</p>", weekday.to_string()).unwrap();
//     }
//     Activator::CountdownTimer(countdown_timer) => {
//       writeln!(into, "<p>rule is active for: {}</p>", countdown_timer.remaining_duration().to_string()).unwrap();
//       writeln!(into, "<p>activator duration: {}</p>", countdown_timer.duration().to_string()).unwrap();
//       writeln!(into, "<p>activator previous synchronization time: {}</p>", countdown_timer.previous_synchronization_time().to_iso_8601_like()).unwrap();
//       if rule.action == Action::Allow {
//         writeln!(into, "<p><a href='/UserAccessFeature/Rule/Activator/CountdownTimer/DecreaseByFiveMinutes?user={username}&&rule={id}'>Decrease by five minutes</a></p>").unwrap();
//       } else {
//         writeln!(into, "<p><a href='/UserAccessFeature/Rule/Activator/CountdownTimer/IncreaseByFiveMinutes?user={username}&&rule={id}'>Increase by five minutes</a></p>").unwrap();
//       }
//     }
//     Activator::InTimeRange(time_range) => {
//       let from = time_range.from().to_12_hour_based_string_with_period();
//       let till = time_range.till().to_12_hour_based_string_with_period();

//       writeln!(into, "<p>rule is activated within time range: {from} .. {till}</p>").unwrap();
      
//     }
//   }
// }