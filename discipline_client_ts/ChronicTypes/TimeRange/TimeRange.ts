import * as CreateFromRawNumbersError from "./CreateFromRawNumbersError.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as Time from "../Time.ts"
import * as Duration from "../Duration.ts"
import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as RangeType from "./TimeRangeType.ts"
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";

export const MIN_FROM_VALUE = 0
export const MAX_FROM_VALUE = Duration.MILLISECONDS_PER_DAY - 1
export const MIN_TILL_VALUE = 0
export const MAX_TILL_VALUE = Duration.MILLISECONDS_PER_DAY * 2 - 1

export type TimeRange = Unique<"Discipline.Chronic.TimeRange", {
  /**
   * A positive integer in this range `Time.MIN_TIMESTAMP` .. `Time.MAX_TIMESTAMP`, inclusive both.
   * 
   * Is less than or equal to `TimeRange.till`.
   */
  readonly from: number
  /**
   * A positive integer in this range `Time.MIN_TIMESTAMP` .. `Time.MAX_TIMESTAMP * 2`, 
   * inclusive both.
   * 
   * Is greater than or equal to `TimeRange.from`.
   */
  readonly till: number
}>

export function create(from: Time.Time, till: Time.Time): TimeRange {
  if (Time.isEarilerThanOrEqualTo(from, till)) {
    return Unique({
      from: Time.asTimestamp(from),
      till: Time.asTimestamp(till),
    })
  } else {
    return Unique({
      from: Time.asTimestamp(from),
      till: Time.asTimestamp(till) + Time.MAX_TIMESTAMP,
    })
  }
}

export function fromRawNumbers(from: number, till: number): Tried<TimeRange, CreateFromRawNumbersError.Error> {
  if (!Integer.isIntegerAndInRange(from, MIN_FROM_VALUE, MAX_FROM_VALUE)) {
    return Err(CreateFromRawNumbersError.InvalidFrom(from))
  }

  if (!Integer.isIntegerAndInRange(till, MIN_TILL_VALUE, MAX_TILL_VALUE)) {
    return Err(CreateFromRawNumbersError.InvalidTill(till))
  }

  if (from > till) {
    return Err(CreateFromRawNumbersError.FromLaterThanTill(
      Time.fromTimestampOrThrow(from), 
      Time.fromTimestampWrappingOrThrow(till),
      till <= MIN_FROM_VALUE ? RangeType.Intraday() : RangeType.Crossday(),
    ));
  }

  if (till - from > Duration.MILLISECONDS_PER_DAY) {
    return Err(CreateFromRawNumbersError.InvalidDuration(
      Time.fromTimestampOrThrow(from), 
      Time.fromTimestampWrappingOrThrow(till),
      till <= MIN_FROM_VALUE ? RangeType.Intraday() : RangeType.Crossday(),
    ))
  }

  return Ok(Unique({ from, till }))
}

export function from(me: TimeRange): Time.Time {
  return Time.fromTimestampOrThrow(me.from)
}

export function till(me: TimeRange): Time.Time {
  return Time.fromTimestampWrappingOrThrow(me.till)
}

export function duration(me: TimeRange): Duration.Duration {
  return Duration.fromMillisecondsOrThrow(me.till - me.from)
}

export function containsTime(me: TimeRange, time: Time.Time): boolean {
  const timeAsTimestamp = Time.asTimestamp(time)
  return timeAsTimestamp >= me.from  && timeAsTimestamp <= me.till
}

export function isIntrday(me: TimeRange): boolean {
  return me.till <= Time.MAX_TIMESTAMP 
}

export function isCrossday(me: TimeRange): boolean {
  return me.till > Time.MAX_TIMESTAMP 
}

export function containsOrEquals(lhs: TimeRange, rhs: TimeRange): boolean {
  return (
    lhs.from <= rhs.from 
    &&
    lhs.till >= rhs.till
  )
}

export function contains(lhs: TimeRange, rhs: TimeRange): boolean {
  return (
    containsOrEquals(lhs, rhs) 
    &&
    (
      lhs.from !== rhs.from 
      &&
      lhs.till !== rhs.till
    )
  )
}

export function isContainedByOrEqualTo(lhs: TimeRange, rhs: TimeRange): boolean {
  return containsOrEquals(rhs, lhs)
}

export function isContainedBy(lhs: TimeRange, rhs: TimeRange): boolean {
  return contains(rhs, lhs)
}

export const displayer = Displayer.implement<TimeRange>(me => 
  Displayer.asNamedObject("TimeRange",
    "from", Time.displayer, from(me),
    "till", Time.displayer, till(me),
    "type", Displayer.stringDisplayer, isIntrday(me) ? "Intraday" : "Crossday",
    "duration", Duration.displayer, duration(me)
  )
)

export const jsonSerializer = JsonSerializer.implement<TimeRange>(me => 
  JsonSerializer.asObject(
    "from", JsonSerializer.integerSerializer, me.from,
    "till", JsonSerializer.integerSerializer, me.till,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<TimeRange>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.andThen2(
    JsonDeserializer.propertyAsInteger(context, "from"),
    JsonDeserializer.propertyAsInteger(context, "till"),
    (from, till) => Tried.mapErr(
      fromRawNumbers(from, till),
      error => JsonDeserializer.err(CreateFromRawNumbersError.displayer.display(error))
    )
  ))
)
