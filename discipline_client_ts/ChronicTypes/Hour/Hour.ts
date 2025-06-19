import * as CreateFromNumberError from "./CreateFromNumberError.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as Meridiem from "../Meridiem.ts"
import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import { None, Some, Option } from "../../ElementaryTypes/Option.ts";
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type Hour = Unique<"Discipline.Chronic.Hour", number>

export const MIN_VALUE = 0
export const MAX_VALUE = 23

export const MIN_12_BASED_VALUE = 0
export const MAX_12_BASED_VALUE = 11

export function fromNumber(number: number): Tried<Hour, CreateFromNumberError.T> {
  if (Integer.isIntegerAndInRange(number, MIN_VALUE, MAX_VALUE)) {
    return Ok(Unique(number))
  } else {
    return Err(CreateFromNumberError.InvalidNumber(number))
  }
}

export function fromNumberAM(number: number): Option<Hour>  {
  if (Integer.isIntegerAndInRange(number, 
    MIN_12_BASED_VALUE, 
    MAX_12_BASED_VALUE,
  )) {
    return Some(Unique(number))
  } else {
    return None()
  }
}

export function fromNumberPM(number: number): Option<Hour> {
  if (Integer.isIntegerAndInRange(number, 
    MIN_12_BASED_VALUE, 
    MAX_12_BASED_VALUE,
  )) {
    return Some(Unique(number + 12))
  } else {
    return None()
  }
}

/**
 * 
 * @returns An integer in this range 0 ..= 23
 */
export function asNumber(me: Hour): number {
  return me
}

/**
 * 
 * @returns An integer in this range 0 ..= 11
 */
export function asNumber12(me: Hour): number {
  if (me < 12) {
    return me
  } else {
    return me - 12
  }
}

export function meridiem(me: Hour): Meridiem.T {
  if (me < 12) {
    return Meridiem.AM()
  } else {
    return Meridiem.PM()
  }
}

export function toNumberAndMeridiem(me: Hour): [ number, Meridiem.T ] {
  if (me < 12) {
    return [ me, Meridiem.AM() ]
  } else {
    return [ me, Meridiem.PM() ]
  }
}

export function toNumber12AndMeridiem(me: Hour): [ number, Meridiem.T ] {
  if (me < 12) {
    return [ me, Meridiem.AM() ]
  } else {
    return [ me - 12, Meridiem.PM() ]
  }
}

export function asNumber12AndMeridiemString(me: Hour): string {
  if (me < 12) {
    return `${me} ${Meridiem.asString(Meridiem.AM())}`
  } else {
    return `${me - 12} ${Meridiem.asString(Meridiem.PM())}`
  }
}

export function isEqualTo(lhs: Hour, rhs: Hour): boolean {
  return lhs === rhs
}

export function isLaterThan(lhs: Hour, rhs: Hour): boolean {
  return lhs > rhs
}

export function isLaterThanOrEqualTo(lhs: Hour, rhs: Hour): boolean {
  return lhs >= rhs
}

export function isEalierThan(lhs: Hour, rhs: Hour): boolean {
  return lhs < rhs
}

export function isEarlierThanOrEqualTo(lhs: Hour, rhs: Hour): boolean {
  return lhs <= rhs
}

export const displayer = Displayer.implement<Hour>(me => {
  const [ hour12, meridiem ] = toNumber12AndMeridiem(me)
  return Displayer.asWrappedString("Hour", `${
    hour12.toString()
  } ${
    Meridiem.asString(meridiem)
  }`)
})

export const jsonSerializer = JsonSerializer.implement<Hour>(me =>
  JsonSerializer.asInteger(asNumber(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Hour>(context =>
  Tried.andThen(JsonDeserializer.asInteger(context), integer => Tried.mapErr(
    fromNumber(integer),
    error => JsonDeserializer.err(CreateFromNumberError.displayer.display(error))
  ))
)
