
// pub enum Status {
//   Active,
//   ActiveIn(Duration),
//   Inactive,
//   InactiveIn(Duration),
// }

// impl Weekday {
//   pub fn status(self, datetime: DateTime) -> Status {
//     let weekday = datetime.weekday();
//     let time = datetime.time();

//     if self < weekday {
//       return Status::ActiveIn(
//         Duration::unchecked_from_days_u32(self.days_till(weekday))
//           .unchecked_sub(&time.from_midnight())
//       );
//     }

//     if self > weekday {
//       return Status::ActiveIn(
//         Duration::unchecked_from_days_u32(self.days_since(weekday))
//           .unchecked_sub(&time.from_midnight())
//       )
//     }

//     Status::InactiveIn(time.till_midnight())
//   }
// }

// impl Hour {
//   pub fn status(self, datetime: DateTime) -> Result<Status, ()> {
//     let hour = datetime.hour();
//     let time = datetime.time();

//     if self < hour {

//       return Ok(Status::ActiveIn(
//         Duration::unchecked_from_hours((hour.value() - self.value()) as u64)
//         .unchecked_sub(&Duration::unchecked_from_minutes(59 - ))
//       ))
//     }

//     if self == hour {
//       return Status::InactiveIn(
//         Duration::unchecked_from_minutes(59 - time.minute() as u64)
//       );
//     }


//     if self < hour {
//       return Status::ActiveIn(Duration::unchecked_from_hours(
//         (hour.value() - self.value()) as u64
//       ))
//     }

//     todo!()
//   }
// }
