import * as CreateFromTimestampError from "./CreateFromTimestampError.ts"
import * as Hour from "../Hour.ts"
import * as Minute from "../Minute.ts"
import * as Second from "../Second.ts"
import * as Duration from "../Duration.ts"
import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as Meridiem from "../Meridiem.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";

export type Time = Unique<"App.Chronic.Time", number>

export const MIN_TIMESTAMP = 0
export const MAX_TIMESTAMP = Duration.MILLISECONDS_PER_DAY - 1

export function fromTimestamp(timestamp: number): Tried<Time, CreateFromTimestampError.Error> {
  if (Integer.isIntegerAndInRange(timestamp, 
    MIN_TIMESTAMP, 
    MAX_TIMESTAMP,
  )) {
    return Ok(Unique(timestamp))
  } else {
    return Err(CreateFromTimestampError.InvalidTimestamp(timestamp))
  }
}

export function fromTimestampOrThrow(timestamp: number): Time {
  if (Integer.isIntegerAndInRange(timestamp, 
    MIN_TIMESTAMP, 
    MAX_TIMESTAMP,
  )) {
    return Unique(timestamp)
  } else {
    throw new Error(`Time.FromTimestampOrThrow: Argument must be a positive integer in range ${MIN_TIMESTAMP} ..= ${MAX_TIMESTAMP} inclusive both. Argument: ${timestamp}`)
  }
}

export function fromTimestampWrappingOrThrow(timestamp: number): Time {
  if (Integer.isPositiveInteger(timestamp)) {
    return Unique(timestamp % (Duration.MILLISECONDS_PER_DAY + 1))
  }
  
  throw new Error(`Time.FromTimestampWrappingOrThrow: Argument must be a positive integer. Argument: ${timestamp}`)
}

export function fromHour(hour: Hour.Hour): Time {
  return Unique(
    Hour.asNumber(hour) * Duration.MILLISECONDS_PER_HOUR
  )
}

export function fromHourMinute(hour: Hour.Hour, minute: Minute.Minute): Time {
  return Unique(
    Hour.asNumber(hour) * Duration.MILLISECONDS_PER_HOUR
    + 
    Minute.asNumber(minute) * Duration.MILLISECONDS_PER_MINUTE
  )
}

export function fromHourMinuteSecond(hour: Hour.Hour, minute: Minute.Minute, second: Second.Second): Time {
  return Unique(
    Hour.asNumber(hour) * Duration.MILLISECONDS_PER_HOUR
    + 
    Minute.asNumber(minute) * Duration.MILLISECONDS_PER_MINUTE
    + 
    Second.asNumber(second) * Duration.MILLISECONDS_PER_SECOND
  )
}

export function hour(me: Time): Hour.Hour {
  // SAFETY: Because `me` is not larger than `maxValue`, this operation
  // is safe.
  return Tried.unwrap(
    Hour.fromNumber(Math.floor(me / Duration.MILLISECONDS_PER_HOUR))
  )
}

export function minute(me: Time): Minute.Minute {
  // SAFETY: Because `me` is not larger than `maxValue`, this operation
  // is safe.
  return Tried.unwrap(
    Minute.fromNumber(Math.floor(me % Duration.MILLISECONDS_PER_HOUR / Duration.MILLISECONDS_PER_MINUTE))
  )
}

export function second(me: Time): Second.Second {
  // SAFETY: Because `me` is not larger than `maxValue`, this operation
  // is safe.
  return Tried.unwrap(
    Second.fromNumber(Math.floor(me % Duration.MILLISECONDS_PER_HOUR % Duration.MILLISECONDS_PER_MINUTE / Duration.MILLISECONDS_PER_SECOND))
  )
}

export function asTimestamp(me: Time): number {
  return me
}

export function asString(me: Time, { hour12 = false }: { hour12?: boolean } = {}): string {
  let string = ""
  
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

export function isEqualTo(lhs: Time, rhs: Time): boolean {
  return lhs === rhs
}

export function isEarilerThanOrEqualTo(lhs: Time, rhs: Time): boolean {
  return lhs <= rhs
}

export function isEarilerThan(lhs: Time, rhs: Time): boolean {
  return lhs < rhs
}

export function isLaterThanOrEqualTo(lhs: Time, rhs: Time): boolean {
  return lhs >= rhs
}

export function isLaterThan(lhs: Time, rhs: Time): boolean {
  return lhs > rhs
}

export const displayer = Displayer.implement<Time>(me =>
  Displayer.asWrappedString("Time", asString(me, { hour12: true }))
)

export const jsonSerializer = JsonSerializer.implement<Time>(me =>
  JsonSerializer.asInteger(asTimestamp(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Time>(context =>
  Tried.andThen(JsonDeserializer.asInteger(context), integer => Tried.mapErr(
    fromTimestamp(integer),
    error => JsonDeserializer.err(CreateFromTimestampError.displayer.display(error))
  ))
)
