import { Duration, Hour, Minute, MonthDay, None, Option, Second, Some, Weekday, Month, Time } from "../../Prelude.ts"
import { Units } from "./mod.ts";

export type DateTime = Date & { readonly __type: "Time.DateTime" }

function useBaghdadTimeZone(me: DateTime): DateTime {
  let timestamp = DateTime.timestamp(me)
  timestamp += TimeZone.getOffsetInMilliseconds(TimeZoneEnum.Baghdad)
  return new Date(timestamp) as DateTime
}

export const DateTime = {
  now(): DateTime {
    return useBaghdadTimeZone(new Date() as DateTime)
  },

  time(me: DateTime): Time {
    return Time.fromHMS(DateTime.hour(me), DateTime.minute(me), DateTime.second(me))
  },

  fromString(string: string): Option<DateTime> {
    return NativeDate.fromString(string) as Option<DateTime>
  },

  fromTimestamp(timestamp: number): Option<DateTime> {
    if (NativeDate.isValidTimestamp(timestamp)) {
      return Some(new Date(timestamp) as DateTime)
    } else {
      return None()
    }
  },

  /** Unsafe */
  uncheckedFromJsDate(date: Date): DateTime {
    return date as DateTime
  },

  second(me: DateTime): Second {
    // SAFETY: `Date.getUTCSeconds()` returns a number between 0 and 59, inclusive both.
    return Second.newUnchecked(me.getUTCSeconds())
  },

  minute(me: DateTime): Minute {
    // SAFETY: `Date.getUTCMinutes()` returns a number between 0 and 59, inclusive both.
    return Minute.newUnchecked(me.getUTCMinutes())
  },

  hour(me: DateTime): Hour {
    // SAFETY: `Date.getUTCHours()` returns a number between 0 and 23, inclusive both.
    return Hour.newUnchecked(me.getUTCHours())
  },

  weekday(me: DateTime): Weekday {
    // SAFETY: `Date.getUTCDay()` returns a number between 0 and 6, inclusive both.
    return Weekday.uncheckedFrom0To6(me.getUTCDay())
  },

  month(me: DateTime): Month {
    // SAFETY: `Date.getUTCMonth()` returns a number between 0 and 11, inclusive both.
    return Month.newUnchecked(me.getUTCMonth() + 1)
  },

  monthDay(me: DateTime): MonthDay {
    // SAFETY: `Date.getUTCDate()` returns a number between 1 and 31, inclusive both.
    return MonthDay.newUnchecked(me.getUTCDate())
  },

  year(me: DateTime): number {
    return me.getUTCFullYear()
  },

  withSecond(me: DateTime, newSecond: number): Option<DateTime> {
    if (Number.isSafeInteger(newSecond) && newSecond >= 0 && newSecond <= 59) {
      const clone = new Date(me)
      clone.setSeconds(newSecond)
      return Some(clone as DateTime)
    }

    return None()
  },

  withMinute(me: DateTime, newMinute: number): Option<DateTime> {
    if (Number.isSafeInteger(newMinute) && newMinute >= 0 && newMinute <= 59) {
      const clone = new Date(me)
      clone.setMinutes(newMinute)
      return Some(clone as DateTime)
    }

    return None()
  },

  withHour(me: DateTime, newHour: number): Option<DateTime> {
    if (Number.isSafeInteger(newHour) && newHour >= 0 && newHour <= 23) {
      const clone = new Date(me)
      clone.setHours(newHour)
      return Some(clone as DateTime)
    }

    return None()
  },

  withWeekday(me: DateTime, newWeekday: Weekday): DateTime {
    const thisWeedkay = me.getDay();
    const difference = newWeekday - thisWeedkay;
    const clone = new Date(me)
    // Adjust day to the target weekday
    clone.setDate(clone.getDate() + difference);  
    return clone as DateTime
  },

  // withTimeZone(me: DateTime, newTimeZone: TimeZoneEnum): DateTime {
  //   let timestamp = DateTime.timestamp(me)
  //   timestamp += TimeZone.getOffsetInMilliseconds(newTimeZone)
  //   return new Date(timestamp) as DateTime
  // },


  isAfter(a: DateTime, b: DateTime) {
    return a.getTime() > b.getTime()
  },

  isBefore(a: DateTime, b: DateTime) {
    return a.getTime() < b.getTime()
  },

  timeline2(a: DateTime, b: DateTime): boolean {
    return a.getTime() <= b.getTime()
  },

  timestamp(me: DateTime) {
    return me.getTime()
  },

  midnight(me: DateTime): DateTime {
    const midnight = new Date(me.getTime())
    midnight.setUTCHours(0, 0, 0, 0)
    return midnight as DateTime
  },

  // since(later: DateTime, earlier: DateTime): Duration {
  //   const timeInBetween = later.getTime() - earlier.getTime()
  //   if (timeInBetween < 0) {
  //     return Duration.newZero()
  //   } else {
  //     return Duration.uncheckedFromMilliseconds(timeInBetween)
  //   }
  // },

  tillOrZero(earlier: DateTime, later: DateTime): Duration {
    const earlierTime = earlier.getTime()
    const laterTime = later.getTime()
    if (earlierTime >= laterTime) {
      return Duration.newZero()
    } else {
      return Duration.uncheckedFromMilliseconds(laterTime - earlierTime)
    }
  },

  toPrettyFormat(me: DateTime, { hour12 = false }: { hour12?: boolean } = {}): string {
    const year = me.getUTCFullYear();
    const month = Month.value(DateTime.month(me)).toString().padStart(2, "0");
    const monthDay = MonthDay.value(DateTime.monthDay(me)).toString().padStart(2, "0");

    
    const minute = Minute.value(DateTime.minute(me)).toString().padStart(2, "0");
    const second = Second.value(DateTime.second(me)).toString().padStart(2, "0");

    if (hour12) {
      const hour24 = Hour.value(DateTime.hour(me))
      let hour12
      let period

      if (hour24 < 12) {
        hour12 = hour24.toString().padStart(2, "0")
        period = "AM"
      } else {
        hour12 = (hour24 - 12).toString().padStart(2, "0")
        period = "PM"
      }

      return `${year}-${month}-${monthDay} ${hour12}:${minute}:${second} ${period}`;
    } else {
      const hour = Hour.value(DateTime.hour(me)).toString().padStart(2, "0");
      return `${year}-${month}-${monthDay} ${hour}:${minute}:${second}`;
    }
  },

  plus(me: DateTime, duration: Duration): Option<DateTime> {
    const timestamp = me.getTime() + Duration.milliseconds(duration)
    if (NativeDate.isValidTimestamp(timestamp)) {
      return Some(new Date(timestamp) as DateTime) 
    }

    return None()
  },

  eq(a: DateTime, b: DateTime): boolean {
    return a.getTime() === b.getTime()
  },  

  gth(a: DateTime, b: DateTime) {
    return a.getTime() > b.getTime()
  },

  gte(a: DateTime, b: DateTime) {
    return a.getTime() >= b.getTime()
  },

  lth(a: DateTime, b: DateTime) {
    return a.getTime() < b.getTime()
  },

  lte(a: DateTime, b: DateTime) {
    return a.getTime() <= b.getTime()
  },

  clone(me: DateTime): DateTime {
    return new Date(me.getTime()) as DateTime
  },

  toString(me: DateTime): string {
    return me.toString()
  },
}

const NativeDate = {
  isValidTimestamp(timestamp: number) {
    return Number.isSafeInteger(timestamp) 
      && -8640000000000000 <= timestamp 
      && timestamp <= 8640000000000000
  },

  isInvalid(me: Date) {
    return Object.is(me.getTime(), NaN)
  },

  fromString(string: string): Option<Date> {
    const date = new Date(string)
    return NativeDate.isInvalid(date)
      ? None()
      : Some(date)
  }
}

// Option.mapAll(
//   DateTime.withHour(DateTime.now(), 0),
//   datetime => DateTime.withMinute(datetime, 1),
//   datetime => DateTime.withSecond(datetime, 1),
// )

export type MaybeDateTime = Option<DateTime>

export const MaybeDateTime = {
  eq(a: MaybeDateTime, b: MaybeDateTime): boolean {
    return Option.eq(a, b, DateTime.eq)
  }
}

// export type TimeZoneUTC = 0 & { readonly __type: "Time.TimeZone.UTC" }
// export type TimeZoneBaghdad = 1 & { readonly __type: "Time.TimeZone.Baghdad" }

// export const TimeZoneUTC = 0 as TimeZoneUTC
// export const TimeZoneBaghdad = 1 as TimeZoneBaghdad

// export type TimeZone = (
//   | TimeZoneUTC
//   | TimeZoneBaghdad
// )

export const enum TimeZoneEnum {
  UTC,
  Baghdad,
}

export const TimeZone = {
  UTCTimeZoneOffset: 0,
  BaghdadTimeZoneOffset: 3 * Units.millisecondsPerHour,

  getOffsetInMilliseconds(me: TimeZoneEnum): number {
    switch (me) {
      case TimeZoneEnum.UTC: return TimeZone.UTCTimeZoneOffset
      case TimeZoneEnum.Baghdad: return TimeZone.BaghdadTimeZoneOffset
    }
  }
}