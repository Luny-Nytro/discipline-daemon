import * as CreateFromNumberError from "./CreateFromNumberError.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export const MIN_VALUE = 1
export const MAX_VALUE = 31

export type MonthDay = Unique<"App.Chronic.MonthDay", number>

export function fromNumber(number: number): Tried<MonthDay, CreateFromNumberError.Error> {
  if (Integer.isIntegerAndInRange(number, 
    MIN_VALUE, 
    MAX_VALUE,
  )) {
    return Ok(Unique(number))
  } else {
    return Err(CreateFromNumberError.InvalidNumber(number))
  }
}

export function asNumber(me: MonthDay): number {
  return me
}

export function isEqualTo(lhs: MonthDay, rhs: MonthDay): boolean {
  return lhs === rhs
}

export function isLaterThan(lhs: MonthDay, rhs: MonthDay): boolean {
  return lhs > rhs
}

export function isLaterThanOrEqualTo(lhs: MonthDay, rhs: MonthDay): boolean {
  return lhs >= rhs
}

export function isEarilerThan(lhs: MonthDay, rhs: MonthDay): boolean {
  return lhs < rhs
}

export function isEarilerThanOrEqualTo(lhs: MonthDay, rhs: MonthDay): boolean {
  return lhs <= rhs
}

export const displayer = Displayer.implement<MonthDay>(me =>
  Displayer.asWrappedNumber("MonthDay", asNumber(me))
)

export const jsonSerializer = JsonSerializer.implement<MonthDay>(me =>
  JsonSerializer.asInteger(asNumber(me))
)

export const jsonDeserializer = JsonDeserializer.implement<MonthDay>((context) =>
  Tried.andThen(JsonDeserializer.asInteger(context), integer => Tried.mapErr(
    fromNumber(integer),
    error => JsonDeserializer.err(CreateFromNumberError.displayer.display(error))
  ))
)