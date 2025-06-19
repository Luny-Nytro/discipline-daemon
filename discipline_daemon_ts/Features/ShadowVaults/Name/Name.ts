import * as CreateError from "./CreateError.ts"
import * as Displayer from "../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Err, Ok, Tried } from "../../../ElementaryTypes/Tried.ts"
import { Unique } from "../../../ElementaryTypes/Unique.ts";

export const MIN_LENGTH = 1
export const MAX_LENGTH = 20

export type Name = Unique<"App.ShadowVault.Name", string>

export function isValidName(string: string): boolean {
  return string.length >= MIN_LENGTH 
      && string.length <= MAX_LENGTH
}

export function create(name: string): Tried<Name, CreateError.Error> {
  return isValidName(name)
    ? Ok(Unique(name))
    : Err(CreateError.InvalidLength(name))
}

export function createOrThrow(name: string): Name {
  if (isValidName(name)) {
    return Unique(name)
  } else {
    throw new Error(CreateError.displayer.display(CreateError.InvalidLength(name))) 
  }
}

export function asString(me: Name): string {
  return me
}

export function isEqualTo(lhs: Name, rhs: Name): boolean {
  return lhs === rhs
}

export const displayer = Displayer.implement<Name>(me => 
  Displayer.asWrappedString("ShadowVaultName", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Name>(me => 
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Name>(context => 
  Tried.andThen(JsonDeserializer.asString(context), string => Tried.mapErr(
    create(string),
    error => JsonDeserializer.err(CreateError.displayer.display(error)),
  ))
)
