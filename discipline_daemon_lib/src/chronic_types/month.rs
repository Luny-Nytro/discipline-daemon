#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, PartialOrd, Ord)]
pub enum Month {
  /// January
  January = 0,
  /// February
  February = 1,
  /// March
  March = 2,
  /// April
  April = 3,
  /// May
  May = 4,
  /// June
  June = 5,
  /// July
  July = 6,
  /// August
  August = 7,
  /// September
  September = 8,
  /// October
  October = 9,
  /// November
  November = 10,
  /// December
  December = 11,
}

pub use Month::*;

impl Month {
  /// The next month.
  pub const fn successor(&self) -> Month {
    match *self {
      January => February,
      February => March,
      March => April,
      April => May,
      May => June,
      June => July,
      July => August,
      August => September,
      September => October,
      October => November,
      November => December,
      December => January,
    }
  }

  /// The previous month.
  pub const fn predecessor(&self) -> Month {
    match *self {
      January => December,
      February => January,
      March => February,
      April => March,
      May => April,
      June => May,
      July => June,
      August => July,
      September => August,
      October => September,
      November => October,
      December => November,
    }
  }

  /// Returns a month-of-year number starting from January = 1.
  #[inline]
  #[must_use]
  pub const fn number(&self) -> u32 {
    match *self {
      January => 1,
      February => 2,
      March => 3,
      April => 4,
      May => 5,
      June => 6,
      July => 7,
      August => 8,
      September => 9,
      October => 10,
      November => 11,
      December => 12,
    }
  }

  /// Get the name of the month
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match *self {
      January => "January",
      February => "February",
      March => "March",
      April => "April",
      May => "May",
      June => "June",
      July => "July",
      August => "August",
      September => "September",
      October => "October",
      November => "November",
      December => "December",
    }
  }
}

impl TryFrom<u8> for Month {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      1 => Ok(January),
      2 => Ok(February),
      3 => Ok(March),
      4 => Ok(April),
      5 => Ok(May),
      6 => Ok(June),
      7 => Ok(July),
      8 => Ok(August),
      9 => Ok(September),
      10 => Ok(October),
      11 => Ok(November),
      12 => Ok(December),
      _ => Err(()),
    }
  }
}

/// A duration in calendar months
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Months(pub(crate) u32);

impl Months {
  /// Construct a new `Months` from a number of months
  pub const fn new(num: u32) -> Self {
    Self(num)
  }

  /// Returns the total number of months in the `Months` instance.
  #[inline]
  pub const fn as_u32(&self) -> u32 {
    self.0
  }
}