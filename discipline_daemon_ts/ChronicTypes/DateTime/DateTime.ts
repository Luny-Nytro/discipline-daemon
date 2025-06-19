import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as Month from "../Month.ts"
import * as Time from "../Time.ts"
import * as Hour from "../Hour.ts"
import * as Minute from "../Minute.ts"
import * as Second from "../Second.ts"
import * as Weekday from "../Weekday.ts"
import * as MonthDay from "../MonthDay.ts"
import * as Duration from "../Duration.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as Meridiem from "../Meridiem.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { None, Option, Some } from "../../ElementaryTypes/Option.ts";
import { Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

// function useBaghdadTimeZone(): DateTime {
//   let timestamp = timestamp(me)
//   timestamp += TimeZone.getOffsetInMilliseconds(TimeZoneEnum.Baghdad)
//   return new Date(timestamp) as DateTime
// }

export type DateTime = Unique<"App.Chronic.DateTime", Date>

export function now(): DateTime {
  return Unique(new Date())
}

export const MIN_TIMESTAMP = -8640000000000000
export const MAX_TIMESTAMP = 8640000000000000

export function isValidTimestamp(timestamp: number): boolean {
  return Integer.isIntegerAndInRange(timestamp, 
    MIN_TIMESTAMP, 
    MAX_TIMESTAMP,
  )
}

export function fromTimestamp(timestamp: number): Option<DateTime> {
  if (isValidTimestamp(timestamp)) {
    return Some(Unique(new Date(timestamp)))
  } else {
    return None()
  }
}

export function asTimestamp(me: DateTime): number {
  return me.getTime()
}

export function isNativeDateValid(nativeDate: Date) {
  // If a Date object is in a valid state, `getTime()` returns 
  // a signed integer.
  return Number.isSafeInteger(nativeDate.getTime())
}

export function isNativeDateInvalid(nativeDate: Date) {
  // If a Date object is in an invalid state, `getTime()` 
  // returns `NaN`.
  return Number.isNaN(nativeDate.getTime())
}

export function fromNativeDate(date: Date): Option<DateTime> {
  if (isNativeDateValid(date)) {
    return Some(Unique(new Date(date)))
  } else {
    return None()
  }
}

export function fromNativeDateOrThrow(date: Date): DateTime {
  if (isNativeDateValid(date)) {
    return Unique(new Date(date))
  } 

  throw new Error(`FromNativeDateOrThrow: Argument 'date' is in an invalid state`)
}

export function second(me: DateTime): Second.Second {
  // SAFETY: `Date.getUTCSeconds()` returns a number between 0 and 59, inclusive both.
  return Tried.unwrap(
    Second.fromNumber(me.getUTCSeconds())
  )
}

export function minute(me: DateTime): Minute.Minute {
  // SAFETY: `Date.getUTCMinutes()` returns a number between 0 and 59, inclusive both.
  return Tried.unwrap(
    Minute.fromNumber(me.getUTCMinutes())
  )
}

export function hour(me: DateTime): Hour.Hour {
  // SAFETY: `Date.getUTCHours()` returns a number between 0 and 23, inclusive both.
  return Tried.unwrap(
    Hour.fromNumber(me.getUTCHours())
  )
}

export function time(me: DateTime): Time.Time {
  return Time.fromHourMinuteSecond(
    hour(me),
    minute(me),
    second(me),
  )
}

export function weekday(me: DateTime): Weekday.Weekday {
  // SAFETY: `Date.getUTCDay()` returns a number between 0 and 6, inclusive both.
  return Weekday.fromNumberOrThrow(me.getUTCDay())
}

export function month(me: DateTime): Month.Month {
  // SAFETY: `Date.getUTCMonth()` returns a number between 0 and 11, inclusive both.
  // We add 1, making it in the range of 1 .. 12.
  return Option.unwrap(
    Month.fromNumber(me.getUTCMonth() + 1)
  )
}

export function monthDay(me: DateTime): MonthDay.MonthDay {
  // SAFETY: `Date.getUTCDate()` returns a number between 1 and 31, inclusive both.
  return Tried.unwrap(
    MonthDay.fromNumber(me.getUTCDate())
  )
}

export function year(me: DateTime): number {
  return me.getUTCFullYear()
}

export function withSecond(me: DateTime, newSecond: Second.Second): Option<DateTime> {
  const clone = new Date(me)
  clone.setSeconds(Second.asNumber(newSecond))
  return fromNativeDate(clone)
}

export function withMinute(me: DateTime, newMinute: Minute.Minute): Option<DateTime> {
  const clone = new Date(me)
  clone.setMinutes(Minute.asNumber(newMinute))
  return fromNativeDate(clone)
}

export function withHour(me: DateTime, newHour: Hour.Hour): Option<DateTime> {
  const clone = new Date(me)
  clone.setHours(Hour.asNumber(newHour))
  return fromNativeDate(clone)
}

// withWeekday(newWeekday: Weekday): Option<DateTime> {
//   // TODO: Verify this function's implementation is correct.
//   const thisWeedkay = me.getDay();
//   const difference = newWeekday.toNumber() - this.weekday().toNumber();
//   const clone = new Date(me)
//   clone.setDate(clone.getDate() + difference);  
//   return fromNativeDate(clone)
// }

export function midnight(me: DateTime): DateTime {
  const midnight = new Date(me.getTime())
  midnight.setUTCHours(0, 0, 0, 0)
  return fromNativeDateOrThrow(midnight)
}

export function till(eariler: DateTime, later: DateTime): Option<Duration.Duration> {
  // TODO: Verify this is an integer in this range `0 ..= Duration.MAX_VALUE`
  // so that the statement below doesn't throw.
  const duration = later.getTime() - eariler.getTime()
  if (duration >= 0) {
    return Some(Duration.fromMillisecondsOrThrow(duration))
  } else {
    return None()
  }
}

export function tillOrZero(eariler: DateTime, later: DateTime): Duration.Duration {
  const duration = later.getTime() - eariler.getTime()
  if (duration < 0) {
    return Duration.fromMillisecondsOrThrow(0)
  }
  
  return Duration.fromMillisecondsOrThrow(
    // SAFETY: This won't be less than zero because laterTime is larger than earlierTime.
    //
    // SAFETY: This won't be greater than Number.MAX_SAFE_INTEGER because the maximum
    // value returned by `Date.getTime()` is `NativeDate.MAX_TIMESTAMP`, which is far less than
    // Number.MAX_SAFE_INTEGER. 
    duration
  )
}

export function plus(me: DateTime, duration: Duration.Duration): Option<DateTime> {
  const timestamp = me.getTime() + Duration.asMilliseconds(duration)
  if (isValidTimestamp(timestamp)) {
    return Some(Unique(new Date(timestamp))) 
  }

  return None()
}

/** 
 * Checkes whether two DateTimes are equal.
*/
export function isEqualTo(me: DateTime, other: DateTime): boolean {
  return me.getTime() === other.getTime()
}

export function isEarilerThan(me: DateTime, other: DateTime): boolean {
  return me.getTime() < other.getTime()
}

export function isEarilerThanOrEqualTo(me: DateTime, other: DateTime): boolean {
  return me.getTime() <= other.getTime()
}

export function isLaterThan(me: DateTime, other: DateTime): boolean {
  return me.getTime() > other.getTime()
}

export function isLaterThanOrEqualTo(me: DateTime, other: DateTime): boolean {
  return me.getTime() >= other.getTime()
}

/** 
 * Converts {@param me} to a datetime string similar to this format:
 * "YYYY-MM-DD HH:MM:SS"
*/
export function asString(me: DateTime, { hour12 = false }: { hour12?: boolean } = {}): string {
  let string = ""

  string += year(me).toString()

  string += ":"
  string += Month.asNumber(month(me)).toString().padStart(2, "0")

  string += ":"
  string += MonthDay.asNumber(monthDay(me)).toString().padStart(2, "0")

  string += " "
  if (hour12) {
    string += Hour.asNumber12(hour(me)).toString().padStart(2, "0")
  } else {
    string += Hour.asNumber(hour(me)).toString().padStart(2, "0")
  }

  string += ":"
  string += Minute.asNumber(minute(me)).toString().padStart(2, "0")

  string += ":"
  string += Second.asNumber(second(me)).toString().padStart(2, "0")

  if (hour12) {
    string += " " 
    string += Meridiem.asString(Hour.meridiem(hour(me)))
  }

  return string
}

export const displayer = Displayer.implement<DateTime>(me =>
  Displayer.asWrappedString("DateTime", asString(me, {
    hour12: true,
  }))
)

export const jsonSerializer = JsonSerializer.implement<DateTime>(me => (
  JsonSerializer.asInteger(asTimestamp(me))
))

export const jsonDeserializer = JsonDeserializer.implement<DateTime>(context => (
  Tried.andThen(JsonDeserializer.asInteger(context), integer => Option.okOrElse(
    fromTimestamp(integer),
    () => JsonDeserializer.err(`DateTime: Expected timestamp ${integer} to be in this range ${MIN_TIMESTAMP} ..= ${MAX_TIMESTAMP}`)
  ))
))


export const DateTime = {
  now,
  isValidTimestamp,
  fromTimestamp,
  asTimestamp,
  isNativeDateValid,
  isNativeDateInvalid,
  fromNativeDate,
  fromNativeDateOrThrow,
  second,
  minute,
  hour,
  time,
  weekday,
  month,
  monthDay,
  year,
  withSecond,
  withMinute,
  withHour,
  midnight,
  till,
  tillOrZero,
  plus,
  isEqualTo,
  isEarilerThan,
  isEarilerThanOrEqualTo,
  isLaterThan,
  isLaterThanOrEqualTo,
  asString,
}