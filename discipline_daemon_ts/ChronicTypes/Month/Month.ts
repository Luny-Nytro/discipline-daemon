import * as CreateFromStringError from "./CreateFromStringError.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Option, Some, None } from "../../ElementaryTypes/Option.ts";
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

const enum MonthType {
  January = 1,
  February = 2,
  March = 3,
  April = 4,
  May = 5,
  June = 6,
  July = 7,
  August = 8,
  September = 9,
  October = 10,
  November = 11,
  December = 12,
}

export type Month = Unique<"App.Chronic.Month", MonthType>

export const JANUARY_AS_STRING = "January"
export const FEBRUARY_AS_STRING = "February"
export const MARCH_AS_STRING = "March"
export const APRIL_AS_STRING = "April"
export const MAY_AS_STRING = "May"
export const JUNE_AS_STRING = "June"
export const JULY_AS_STRING = "July"
export const AUGUST_AS_STRING = "August"
export const SEPTEMBER_AS_STRING = "September"
export const OCTOBER_AS_STRING = "October"
export const NOVEMBER_AS_STRING = "November"
export const DECEMBER_AS_STRING = "December"

export const MIN_VALUE = 1
export const MAX_VALUE = 12

export function January(): Month {
  return Unique(MonthType.January)
}

export function February(): Month {
  return Unique(MonthType.February)
}

export function March(): Month {
  return Unique(MonthType.March)
}

export function April(): Month {
  return Unique(MonthType.April)
}

export function May(): Month {
  return Unique(MonthType.May)
}

export function June(): Month {
  return Unique(MonthType.June)
}

export function July(): Month {
  return Unique(MonthType.July)
}

export function August(): Month {
  return Unique(MonthType.August)
}

export function September(): Month {
  return Unique(MonthType.September)
}

export function October(): Month {
  return Unique(MonthType.October)
}

export function November(): Month {
  return Unique(MonthType.November)
}

export function December(): Month {
  return Unique(MonthType.December)
}

export function isJanuary(me: Month): boolean {
  return me === MonthType.January
}

export function isFebruary(me: Month): boolean {
  return me === MonthType.February
}

export function isMarch(me: Month): boolean {
  return me === MonthType.March
}

export function isApril(me: Month): boolean {
  return me === MonthType.April
}

export function isMay(me: Month): boolean {
  return me === MonthType.May
}

export function isJune(me: Month): boolean {
  return me === MonthType.June
}

export function isJuly(me: Month): boolean {
  return me === MonthType.July
}

export function isAugust(me: Month): boolean {
  return me === MonthType.August
}

export function isSeptember(me: Month): boolean {
  return me === MonthType.September
}

export function isOctober(me: Month): boolean {
  return me === MonthType.October
}

export function isNovember(me: Month): boolean {
  return me === MonthType.November
}

export function isDecember(me: Month): boolean {
  return me === MonthType.December
}

export function fromNumber(number: number): Option<Month> {
  if (Integer.isIntegerAndInRange(number, 
    MIN_VALUE, 
    MAX_VALUE,
  )) {
    return Some(Unique(number))
  } else {
    return None()
  }
}

export function asNumber(me: Month): number {
  return me
}

export function fromString(string: string): Tried<Month, CreateFromStringError.Error> {
  switch (string) {
    case JANUARY_AS_STRING: return Ok(January())
    case FEBRUARY_AS_STRING: return Ok(February())
    case MARCH_AS_STRING: return Ok(March())
    case APRIL_AS_STRING: return Ok(April())
    case MAY_AS_STRING: return Ok(May())
    case JUNE_AS_STRING: return Ok(June())
    case JULY_AS_STRING: return Ok(July())
    case AUGUST_AS_STRING: return Ok(August())
    case SEPTEMBER_AS_STRING: return Ok(September())
    case OCTOBER_AS_STRING: return Ok(October())
    case NOVEMBER_AS_STRING: return Ok(November())
    case DECEMBER_AS_STRING: return Ok(December())
    default: return Err(CreateFromStringError.InvalidString(string))
  }
}

export function asString(me: Month): string {
  switch (me as MonthType) {
    case MonthType.January: return JANUARY_AS_STRING
    case MonthType.February: return FEBRUARY_AS_STRING
    case MonthType.March: return MARCH_AS_STRING
    case MonthType.April: return APRIL_AS_STRING
    case MonthType.May: return MAY_AS_STRING
    case MonthType.June: return JUNE_AS_STRING
    case MonthType.July: return JULY_AS_STRING
    case MonthType.August: return AUGUST_AS_STRING
    case MonthType.September: return SEPTEMBER_AS_STRING
    case MonthType.October: return OCTOBER_AS_STRING
    case MonthType.November: return NOVEMBER_AS_STRING
    case MonthType.December: return DECEMBER_AS_STRING
  }
}

export function isEqualTo(lhs: Month, rhs: Month): boolean {
  return lhs === rhs
}

export function isLaterThan(lhs: Month, rhs: Month): boolean {
  return lhs > rhs
}

export function isLaterThanOrEqualTo(lhs: Month, rhs: Month): boolean {
  return lhs >= rhs
}

export function isEarilerThan(lhs: Month, rhs: Month): boolean {
  return lhs < rhs
}

export function isEarilerThanOrEqualTo(lhs: Month, rhs: Month): boolean {
  return lhs <= rhs
}

export const displayer = Displayer.implement<Month>(me =>
  Displayer.asWrappedString("Month", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Month>(me =>
  Displayer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Month>(context =>
  Tried.andThen(JsonDeserializer.asString(context), string => Tried.mapErr(
    fromString(string),
    error => JsonDeserializer.err(CreateFromStringError.displayer.display(error))
  ))
)