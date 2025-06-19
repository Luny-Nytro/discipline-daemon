import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as CreateFromNumberError from "./CreateFromNumberError.ts"
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type Second = Unique<"Discipline.Chronic.Second", number>

export const MIN_VALUE = 0
export const MAX_VALUE = 59

export function fromNumber(number: number): Tried<Second, CreateFromNumberError.Error> {
  if (Integer.isIntegerAndInRange(number, 
    MIN_VALUE, 
    MAX_VALUE,
  )) {
    return Ok(Unique(number))
  } else {
    return Err(CreateFromNumberError.InvalidNumber(number))
  }
}

export function asNumber(me: Second): number {
  return me
}

export function isEqualTo(lhs: Second, rhs: Second): boolean {
  return lhs === rhs
}

export function isLaterThan(lhs: Second, rhs: Second): boolean {
  return lhs > rhs
}

export function isLaterThanOrEqualTo(lhs: Second, rhs: Second): boolean {
  return lhs >= rhs
}

export function isEarilerThan(lhs: Second, rhs: Second): boolean {
  return lhs < rhs
}

export function isEarilerThanOrEqualTo(lhs: Second, rhs: Second): boolean {
  return lhs <= rhs
}

export const displayer = Displayer.implement<Second>(value =>
  Displayer.asWrappedNumber("Second", asNumber(value))
)

export const jsonSerializer = JsonSerializer.implement<Second>(minute => 
  JsonSerializer.asInteger(asNumber(minute))
)

export const jsonDeserializer = JsonDeserializer.implement<Second>((context) =>
  Tried.andThen(JsonDeserializer.asInteger(context), integer => Tried.mapErr(
    fromNumber(integer),
    error => JsonDeserializer.err(CreateFromNumberError.displayer.display(error))
  ))
)