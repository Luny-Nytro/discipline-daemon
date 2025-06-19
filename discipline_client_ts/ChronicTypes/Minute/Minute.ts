import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as CreateFromNumberError from "./CreateFromNumberError.ts"
import * as Integer from "../../ElementaryTypes/Integer/mod.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type Minute = Unique<"Discipline.Chronic.Minute", number>
export const MIN_VALUE = 0
export const MAX_VALUE = 59

export function fromNumber(number: number): Tried<Minute, CreateFromNumberError.Error> {
  if (Integer.isIntegerAndInRange(number, 
    MIN_VALUE, 
    MAX_VALUE,
  )) {
    return Ok(Unique(number))
  } else {
    return Err(CreateFromNumberError.InvalidNumber(number))
  }
}

export function asNumber(me: Minute): number {
  return me
}

export function isEqualTo(lhs: Minute, rhs: Minute): boolean {
  return lhs === rhs
}

export function isLaterThan(lhs: Minute, rhs: Minute): boolean {
  return lhs > rhs
}

export function isLaterThanOrEqualTo(lhs: Minute, rhs: Minute): boolean {
  return lhs >= rhs
}

export function isEarilerThan(lhs: Minute, rhs: Minute): boolean {
  return lhs < rhs
}

export function isEarilerThanOrEqualTo(lhs: Minute, rhs: Minute): boolean {
  return lhs <= rhs
}

export const displayer = Displayer.implement<Minute>(me =>
  Displayer.asWrappedNumber("Minute", asNumber(me))
)

export const jsonSerializer = JsonSerializer.implement<Minute>(me => 
  JsonSerializer.asInteger(asNumber(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Minute>(context =>
  Tried.andThen(JsonDeserializer.asInteger(context), integer => Tried.mapErr(
    fromNumber(integer),
    error => JsonDeserializer.err(CreateFromNumberError.displayer.display(error))
  ))
)