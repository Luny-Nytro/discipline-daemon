import * as Integer from "../../ElementaryTypes/Integer/mod.ts";
import * as Weekday from "../Weekday.ts";
import * as Duration from "../Duration.ts";
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as RangeType from "./RangeType.ts"
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as CreateFromRawNumbersError from "./CreateFromRawNumbersError.ts"
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type WeekdayRange = Unique<"Discipline.Chronic.WeekdayRange", {
  /**
   * A positive integer from 0 to 6, inclusive both.
   * 
   * Is less than or equal to `till`.
   */
  readonly from: number
  /**
   * A positive integer from 0 to 13.
   * 
   * Is greater than or equal to `from`.
   */
  readonly till: number
}>

export const MIN_FROM_VALUE = 0;
export const MAX_FROM_VALUE = 6;
export const MIN_TILL_VALUE = 0;
export const MAX_TILL_VALUE = 13;

export function create(from: Weekday.Weekday, till: Weekday.Weekday): WeekdayRange {
  if (Weekday.isEarlierThanOrEqualTo(from, till)) {
    return Unique({
      from: Weekday.asNumber(from),
      till: Weekday.asNumber(till),
    })
  } else {
    return Unique({
      from: Weekday.asNumber(from),
      till: Weekday.asNumber(till) + 7,
    })
  }
}

export function fromNumbers(from: number, till: number): Tried<WeekdayRange, CreateFromRawNumbersError.Error> {
  if (Integer.isIntegerAndInRange(from, MIN_FROM_VALUE, MAX_FROM_VALUE)) {
    return Err(CreateFromRawNumbersError.InvalidFrom(from))
  }

  if (Integer.isIntegerAndInRange(till, MIN_TILL_VALUE, MAX_TILL_VALUE)) {
    return Err(CreateFromRawNumbersError.InvalidTill(till))
  }

  if (from > till) {
    return Err(CreateFromRawNumbersError.FromLaterThanTill(
      Weekday.fromNumberOrThrow(from), 
      Weekday.fromNumberWrappingOrThrow(till),
      till <= MAX_FROM_VALUE ? RangeType.IntraWeek() : RangeType.CrossWeek()
    ))
  }

  if (till - from > Duration.MILLISECONDS_PER_WEEK) {
    return Err(CreateFromRawNumbersError.InvalidDuration(
      Weekday.fromNumberOrThrow(from), 
      Weekday.fromNumberWrappingOrThrow(till),
      till <= MAX_FROM_VALUE ? RangeType.IntraWeek() : RangeType.CrossWeek()
    ))
  }

  return Ok(Unique({ from, till }))
}

export function from(me: WeekdayRange): Weekday.Weekday {
  return Weekday.fromNumberOrThrow(me.from)
}

export function till(me: WeekdayRange): Weekday.Weekday {
  return Weekday.fromNumberWrappingOrThrow(me.till)
}

export function duration(me: WeekdayRange): Duration.Duration {
  return Duration.fromDaysOrThrow(me.till - me.from)
}

export function containsWeekday(me: WeekdayRange, weekday: Weekday.Weekday): boolean {
  const weekdayAsNumber = Weekday.asNumber(weekday)
  
  return (
    // Weekday is later than or same as from
    weekdayAsNumber >= me.from
    && 
    // Weekday is earlier than or same as till
    weekdayAsNumber <= me.till
  )
}

export function isIntrWeek(me: WeekdayRange): boolean {
  return me.till <= MAX_FROM_VALUE
}

export function isCrossWeek(me: WeekdayRange): boolean {
  return me.till > MAX_FROM_VALUE
}

export function containsOrEquals(me: WeekdayRange, other: WeekdayRange): boolean {
  return (
    me.from <= other.from
    &&
    me.till >= other.till
  )
}

export function contains(me: WeekdayRange, other: WeekdayRange): boolean {
  return (
    containsOrEquals(me, other)
    &&
    (
      me.from !== other.from
      &&
      me.till !== other.till
    )
  )
}

export function isContainedByOrEqualTo(me: WeekdayRange, other: WeekdayRange): boolean {
  return containsOrEquals(other, me)
}

export function isContainedBy(me: WeekdayRange, other: WeekdayRange): boolean {
  return contains(other, me)
}

export const displayer = Displayer.implement<WeekdayRange>(me => 
  Displayer.asNamedObject("WeekdayRange",
    "from", Weekday.displayer, from(me),
    "till", Weekday.displayer, till(me),
    "type", Displayer.stringDisplayer, isIntrWeek(me) ? "Intr-Week" : "Cross-Week",
    "duration", Duration.displayer, duration(me)
  )
)

export const jsonSerializer = JsonSerializer.implement<WeekdayRange>(me => 
  JsonSerializer.asObject(
    "from", JsonSerializer.integerSerializer, me.from,
    "till", JsonSerializer.integerSerializer, me.till,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<WeekdayRange>(context =>
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.andThen2(
    JsonDeserializer.propertyAsInteger(context, "from"),
    JsonDeserializer.propertyAsInteger(context, "till"),
    (from, till) => Tried.mapErr(
      fromNumbers(from, till),
      error => JsonDeserializer.err(CreateFromRawNumbersError.displayer.display(error))
    )
  ))
)