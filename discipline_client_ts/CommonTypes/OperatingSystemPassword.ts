import * as Displayer from "../ElementaryTypes/Display.ts"
import { None, Option, Some } from "../ElementaryTypes/Option.ts"
import * as JsonSerializer from "../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../ElementaryTypes/Tried.ts";
import { Unique } from "../ElementaryTypes/Unique.ts";

export const MIN_LENGTH = 1
export const MAX_LENGTH = 256

export type OperatingSystemPassword = Unique<"Discipline.Common.OperatingSystemPassword", string>

export function create(value: string): Option<OperatingSystemPassword> {
  if (value.length < MIN_LENGTH) {
    return None()
  } 
  
  if (value.length > MAX_LENGTH) {
    return None()
  }

  return Some(Unique(value))  
}

export function createOrThrow(value: string): OperatingSystemPassword {
  if (value.length < MIN_LENGTH) {
    throw new Error(`OperatingSystemPassword.NewOrThrow: Argument 'password' is shorter than ${MIN_LENGTH} characters. Argument: ${value}`)
  } 

  if (value.length > MAX_LENGTH) {
    throw new Error(`OperatingSystemPassword.NewOrThrow: Argument 'password' is longer than ${MAX_LENGTH} characters. Argument: ${value}`)
  }
  
  return Unique(value)
}

export function asString(me: OperatingSystemPassword): string {
  return me
}

export const displayer = Displayer.implement<OperatingSystemPassword>(me => 
  Displayer.asWrappedString("OperatingSystemPassword", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<OperatingSystemPassword>(me => 
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<OperatingSystemPassword>(context =>
  Tried.andThen(JsonDeserializer.asString(context), string => Option.okOrElse(
    create(string),
    () => JsonDeserializer.err(`OperatingSystemPassword: Expected pasword length to be in this range ${MIN_LENGTH} ..= ${MAX_LENGTH}. Found length: ${string.length}`)
  ))
)