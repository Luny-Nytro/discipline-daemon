import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { None, Option, Some } from "../../ElementaryTypes/Option.ts"
import { Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export const MILLISECONDS_PER_SECOND = 1000
export const MILLISECONDS_PER_MINUTE = MILLISECONDS_PER_SECOND * 60
export const MILLISECONDS_PER_HOUR = MILLISECONDS_PER_MINUTE * 60
export const MILLISECONDS_PER_DAY = MILLISECONDS_PER_HOUR * 24
export const MILLISECONDS_PER_WEEK = MILLISECONDS_PER_DAY * 7

function millisToSeconds(milliseconds: number): number {
  return Math.floor(milliseconds / MILLISECONDS_PER_SECOND)
}

function millisToMinutes(milliseconds: number): number {
  return Math.floor(milliseconds / MILLISECONDS_PER_MINUTE)
}

function millisToHours(milliseconds: number): number {
  return Math.floor(milliseconds / MILLISECONDS_PER_HOUR)
}

function millisToDays(milliseconds: number): number {
  return Math.floor(milliseconds / MILLISECONDS_PER_DAY)
}

function millisToWeeks(milliseconds: number): number {
  return Math.floor(milliseconds / MILLISECONDS_PER_WEEK)
}

function secondsToMillisOrInfinity(seconds: number): number {
  return seconds * MILLISECONDS_PER_SECOND
}

function minutesToMillisOrInfinity(minutes: number): number {
  return minutes * MILLISECONDS_PER_MINUTE
}

function hoursToMillisecondsOrInfinity(hours: number): number {
  return hours * MILLISECONDS_PER_HOUR
}

function daysToMillisOrInfinity(days: number): number {
  return days * MILLISECONDS_PER_DAY
}

function weeksToMillisOrInfinity(weeks: number): number {
  return weeks * MILLISECONDS_PER_WEEK
}

export const MIN_VALUE = 0
export const MAX_VALUE = Number.MAX_SAFE_INTEGER

export type Duration = Unique<"Discipline.Chronic.Duration", number>

export function Zero(): Duration {
  return Unique(0)
}

export function OneMinute(): Duration {
  return Unique(MILLISECONDS_PER_MINUTE)
}

export function TwoMinutes(): Duration {
  return Unique(MILLISECONDS_PER_MINUTE * 2)
}

export function OneWeek(): Duration {
  return Unique(MILLISECONDS_PER_WEEK)
}

export function fromDaysHoursMinutes(days: number, hours: number, minutes: number): Option<Duration> {
  return fromMilliseconds(
    days * MILLISECONDS_PER_DAY
    + 
    hours * MILLISECONDS_PER_HOUR
    + 
    minutes * MILLISECONDS_PER_MINUTE
  )
}

export function fromMillisecondsOrThrow(milliseconds: number): Duration {
  if (Integer.isSafeAndPositive(milliseconds)) {
    return Unique(milliseconds)
  } else {
    throw new Error(`FromMillisecondsOrThrow: Argument must be a safe positive integer. Argument: ${milliseconds}`)
  }
}

export function fromMilliseconds(milliseconds: number): Option<Duration> {
  if (Integer.isSafeAndPositive(milliseconds)) {
    return Some(Unique(milliseconds))
  } else {
    return None()
  }
}

export function fromSeconds(seconds: number): Option<Duration> {
  if (!Integer.isSafeAndPositive(seconds)) {
    return None()
  }
  
  const millis = secondsToMillisOrInfinity(seconds)
  if (millis === Infinity) {
    return None()
  }
  
  return Some(Unique(millis))
}

export function fromMinutes(minutes: number): Option<Duration> {
  if (!Integer.isSafeAndPositive(minutes)) {
    return None()
  }
  
  const millis = minutesToMillisOrInfinity(minutes)
  if (millis === Infinity) {
    return None()
  } 
  
  return Some(Unique(millis))
}
/**
 * Creates a new duration that is {@param hours} long.
 */
export function fromHours(hours: number): Option<Duration> {
  if (!Integer.isSafeAndPositive(hours)) {
    return None()
  }

  const millis = hoursToMillisecondsOrInfinity(hours)
  if (millis === Infinity) {
    return None()
  } 
  
  return Some(Unique(millis))
}

export function fromDays(days: number): Option<Duration> {
  if (!Integer.isSafeAndPositive(days)) {
    return None()
  }
  
  const millis = daysToMillisOrInfinity(days)
  if (millis === Infinity) {
    return None()
  }

  return Some(Unique(millis))
}

export function fromDaysOrThrow(days: number): Duration {
  if (!Integer.isSafeAndPositive(days)) {
    throw new Error(`FromDaysOrThrow: Argument 'days' must be a safe positive integer. Argument: ${days}`)
  }

  const millis = daysToMillisOrInfinity(days)
  if (days === Infinity ) {
    throw new Error(`FromDaysOrThrow: Argument 'days' is too large: Cannot represent a duration ${days} days long`)
  }

  return Unique(millis)
}

export function fromWeeks(weeks: number): Option<Duration> {
  if (!Integer.isSafeAndPositive(weeks)) {
    return None()
  }
  
  const millis = weeksToMillisOrInfinity(weeks)
  if (millis === Infinity) {
    return None()
  } 

  return Some(Unique(millis))
}

export function asMilliseconds(me: Duration): number {
  return me
}

export function asSeconds(me: Duration): number {
  return millisToSeconds(me)
}

export function asMinutes(me: Duration): number {
  return millisToMinutes(me)
}

export function asHours(me: Duration): number {
  return millisToHours(me)
}

export function asDays(me: Duration): number {
  return millisToDays(me)
}

export function asWeeks(me: Duration): number {
  return millisToWeeks(me)
}

export function isEqualTo(lhs: Duration, rhs: Duration): boolean {
  return lhs === rhs
}

export function isLongerThan(lhs: Duration, rhs: Duration): boolean {
  return lhs > rhs
}

export function isLongerThanOrEqualTo(lhs: Duration, rhs: Duration): boolean {
  return lhs >= rhs
}

export function isShorterThan(lhs: Duration, rhs: Duration): boolean {
  return lhs < rhs
}

export function isShorterThanOrEqualTo(lhs: Duration, rhs: Duration): boolean {
  return lhs <= rhs
}

export function isZero(me: Duration): boolean {
  return me === 0
}

export function minus(lhs: Duration, rhs: Duration): Option<Duration> {
  return lhs >= rhs
    ? Some(Unique(lhs - rhs))
    : None()
}

export function minusOrZero(lhs: Duration, rhs: Duration): Duration {
  return lhs >= rhs
    ? Unique(lhs - rhs)
    : Zero()
}

export function plus(lhs: Duration, rhs: Duration): Option<Duration> {
  // TODO: Create a dedicated integer type that handles integer
  // arthmetic in a safe way, for example, it should have a "checkedPlus"
  // method that returns an error if the addition fails. 
  const total = lhs + rhs

  return Integer.isSafeInteger(total)
    ? Some(Unique(total))
    : None()
}

export function plusOrMax(lhs: Duration, rhs: Duration): Duration {
  const total = lhs + rhs

  return Integer.isSafeInteger(total)
    ? Unique(total)
    : Unique(Number.MAX_SAFE_INTEGER)
}

export function asString(me: Duration): string {
  let millies = me as number

  const days = Math.floor(millies / MILLISECONDS_PER_DAY);
  millies %= MILLISECONDS_PER_DAY;

  const hours = Math.floor(millies / MILLISECONDS_PER_HOUR);
  millies %= MILLISECONDS_PER_HOUR;

  const minutes = Math.floor(millies / MILLISECONDS_PER_MINUTE);
  millies %= MILLISECONDS_PER_MINUTE;

  const seconds = Math.floor(millies / MILLISECONDS_PER_SECOND);

  let string = ""

  if (days > 0) {
    if (string.length) {
      string += " "
    }

    string += `${days}D`
  }

  if (hours > 0) {
    if (string.length) {
      string += " "
    }

    string += `${hours}H`
  }
  
  if (minutes > 0) {
    if (string.length) {
      string += " "
    }

    string += `${minutes}M`
  }

  if (seconds > 0) {
    if (string.length) {
      string += " "
    }

    string += `${seconds}S`
  }

  return string.length > 0 ? string : 'Zero';
}

export const displayer = Displayer.implement<Duration>(me =>
  Displayer.asWrappedString("Duration", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Duration>(me => 
  JsonSerializer.asInteger(asMilliseconds(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Duration>(context =>
  Tried.andThen(JsonDeserializer.asInteger(context), integer => Option.okOrElse(
    fromMilliseconds(integer),
    () => JsonDeserializer.err(`Duration: Expected integer ${integer} to be in this range ${MIN_VALUE} ..= ${MAX_VALUE}`)
  ))
)