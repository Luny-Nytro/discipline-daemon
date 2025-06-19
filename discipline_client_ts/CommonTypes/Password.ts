import * as Displayer from "../ElementaryTypes/Display.ts"
import { None, Option, Some } from "../ElementaryTypes/Option.ts"
import * as JsonSerializer from "../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../ElementaryTypes/Tried.ts";
import { Unique } from "../ElementaryTypes/Unique.ts";

export const MIN_VALUE = 1
export const MAX_VALUE = 50

export type Password = Unique<"Discipline.Common.Password", string>

export function create(value: string): Option<Password> {
  if (value.length < MIN_VALUE) {
    return None()
  } 

  if (value.length > MAX_VALUE) {
    return None()
  }

  return Some(Unique(value))
}

export function createOrThrow(value: string): Password {
  if (value.length < MIN_VALUE) {
    throw new Error(`Password.newOrThrow: Argument must be at least ${MIN_VALUE} characters in length. Argument: ${value}`)
  } 

  if (value.length > MAX_VALUE) {
    throw new Error(`Password.newOrThrow: Argument must be at most ${MAX_VALUE} characters in length. Argument: ${value}`)
  }
  
  return Unique(value)
}

export function asString(me: Password): string {
  return me
}

export function isEqualTo(me: Password, other: Password): boolean {
  return me === other
}

export const displayer = Displayer.implement<Password>(me =>
  Displayer.asWrappedString("Password", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Password>(me => 
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Password>(context =>
  Tried.andThen(JsonDeserializer.asString(context), string => Option.okOrElse(
    create(string),
    () => JsonDeserializer.err("Invalid discipline password.")
  ))
)
