import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as CreateFromStringError from "./CreateFromStringError.ts"
import { None, Option, Some } from "../../ElementaryTypes/Option.ts"
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export interface WeekdayCases<A, B, C, D, E, F, G> {
  readonly Sunday: () => A
  readonly Monday: () => B
  readonly Tuseday: () => C
  readonly Wednesday: () => D
  readonly Thursday: () => E
  readonly Friday: () => F
  readonly Saturday: () => G
}

const enum Value {
  Sunday = 0,
  Monday = 1,
  Tuseday = 2,
  Wednesday = 3,
  Thursday = 4,
  Friday = 5,
  Saturday = 6,  
}

export type Weekday = Unique<"Discipline.Chronic.Weekday", Value>

export const SUNDAY_AS_STRING = "Sunday"
export const MONDAY_AS_STRING = "Monday"
export const TUSEDAY_AS_STRING = "Tuseday"
export const WEDNESDAY_AS_STRING = "Wednesday"
export const THURSDAY_AS_STRING = "Thursday"
export const FRIDAY_AS_STRING = "Friday"
export const SATURDAY_AS_STRING = "Saturday"

export function Sunday(): Weekday {
  return Unique(Value.Sunday)
}

export function Monday(): Weekday {
  return Unique(Value.Monday)
}

export function Tuseday(): Weekday {
  return Unique(Value.Tuseday)
}

export function Wednesday(): Weekday {
  return Unique(Value.Wednesday)
}

export function Thursday(): Weekday {
  return Unique(Value.Thursday)
}

export function Friday(): Weekday {
  return Unique(Value.Friday)
}

export function Saturday(): Weekday {
  return Unique(Value.Saturday)
}

export function asNumber(me: Weekday): number {
  return me
}

export function fromNumber(number: number): Option<Weekday> {
  if (Integer.isIntegerAndInRange(number, 
    Value.Sunday, 
    Value.Saturday
  )) {
    return Some(Unique(Unique(number as Value)))
  } else {
    return None()
  }
}

/**
 * Caller must verify that {@param number} is an integer in the range
 * of 0 .. 6.
 */
export function fromNumberOrThrow(number: number): Weekday {
  if (Integer.isIntegerAndInRange(number, 
    Value.Sunday, 
    Value.Saturday,
  )) {
    throw new Error(`Weekday.FromNumberOrThrow: Argument must be an integer between 0 and 6, inclusive both. Argument: ${number}`)
  }

  return Unique(number as Value)
}

export function fromNumberWrapping(value: number): Option<Weekday> {
  if (Integer.isPositiveInteger(value)) {
    return Some(Unique(value % 7 as Value))
  }
  
  return None()
}

export function fromNumberWrappingOrThrow(value: number): Weekday {
  if (Integer.isPositiveInteger(value)) {
    return Unique(value % 7 as Value)
  }
  
  throw new Error(`Weekday.FromNumberWrapping: Argument must be a positive integer. Argument: ${value}`);
}

export function asString(me: Weekday): string {
  switch (me as Value) {
    case Value.Sunday: return SUNDAY_AS_STRING
    case Value.Monday: return MONDAY_AS_STRING
    case Value.Tuseday: return TUSEDAY_AS_STRING
    case Value.Wednesday: return WEDNESDAY_AS_STRING
    case Value.Thursday: return THURSDAY_AS_STRING
    case Value.Friday: return FRIDAY_AS_STRING
    case Value.Saturday: return SATURDAY_AS_STRING
  }
}

export function fromString(name: string): Tried<Weekday, CreateFromStringError.T> {
  switch (name) {
    case SUNDAY_AS_STRING: return Ok(Sunday())
    case MONDAY_AS_STRING: return Ok(Monday())
    case TUSEDAY_AS_STRING: return Ok(Tuseday())
    case WEDNESDAY_AS_STRING: return Ok(Wednesday())
    case THURSDAY_AS_STRING: return Ok(Thursday())
    case FRIDAY_AS_STRING: return Ok(Friday())
    case SATURDAY_AS_STRING: return Ok(Saturday())
    default: return Err(CreateFromStringError.UnknownWeekdayName(name))
  }
}

export function isEqualTo(lhs: Weekday, rhs: Weekday): boolean {
  return lhs === rhs
}

export function isLaterThan(lhs: Weekday, rhs: Weekday): boolean {
  return lhs > rhs
}

export function isLaterThanOrEqualTo(lhs: Weekday, rhs: Weekday): boolean {
  return lhs >= rhs
}

export function isEarilerThan(lhs: Weekday, rhs: Weekday): boolean {
  return lhs < rhs
}

export function isEarlierThanOrEqualTo(lhs: Weekday, rhs: Weekday): boolean {
  return lhs <= rhs
}

export function match<A, B, C, D, E, F, G>(
  me: Weekday,
  cases: WeekdayCases<A, B, C, D, E, F, G>
):
  A | B | C | D | E | F | G
{
  switch (me as Value) {
    case Value.Sunday: return cases.Sunday()
    case Value.Monday: return cases.Monday()
    case Value.Tuseday: return cases.Tuseday()
    case Value.Wednesday: return cases.Wednesday()
    case Value.Thursday: return cases.Thursday()
    case Value.Friday: return cases.Friday()
    case Value.Saturday: return cases.Saturday()
  }
}

export const displayer = Displayer.implement<Weekday>(me => 
  Displayer.asWrappedString("Weekday", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Weekday>(me =>
  JsonSerializer.asInteger(asNumber(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Weekday>(context =>
  Tried.andThen(JsonDeserializer.asString(context), string => Tried.mapErr(
    fromString(string),
    error => JsonDeserializer.err(CreateFromStringError.displayer.display(error)),
  ))
)