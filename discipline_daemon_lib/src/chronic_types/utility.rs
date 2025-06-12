use crate::month::*;

/// Returns if the provided year is a leap year in the proleptic Gregorian calendar. Uses
/// [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering).
pub const fn is_leap_year(year: i32) -> bool {
  year % 4 == 0 && (year % 25 != 0 || year % 16 == 0)
}

/// Returns the number of calendar days in a given year.
///
/// The returned value will always be either 365 or 366.
pub const fn days_in_year(year: i32) -> u16 {
  if is_leap_year(year) { 
    366 
  } else { 
    365 
  }
}

/// Returns the number of weeks in the ISO year.
///
/// The returned value will always be either 52 or 53.
pub const fn weeks_in_year(year: i32) -> u8 {
  match year.rem_euclid(400) {
      4 | 9 | 15 | 20 | 26 | 32 | 37 | 43 | 48 | 54 | 60 | 65 | 71 | 76 | 82 | 88 | 93 | 99
      | 105 | 111 | 116 | 122 | 128 | 133 | 139 | 144 | 150 | 156 | 161 | 167 | 172 | 178
      | 184 | 189 | 195 | 201 | 207 | 212 | 218 | 224 | 229 | 235 | 240 | 246 | 252 | 257
      | 263 | 268 | 274 | 280 | 285 | 291 | 296 | 303 | 308 | 314 | 320 | 325 | 331 | 336
      | 342 | 348 | 353 | 359 | 364 | 370 | 376 | 381 | 387 | 392 | 398 => 53,
      _ => 52,
  }
}


/// Returns the number of days in the month of a given year.
pub const fn days_in_year_month(year: i32, month: Month) -> u8 {
  match month {
      January | March | May | July | August | October | December => {
        31
      }
      April | June | September | November => {
        30
      }
      February if is_leap_year(year) => {
        29
      }
      February => {
        28
      }
  }
}