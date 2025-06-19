import * as CreateError from "./CreateError.ts"
import * as JsonSerializer from "../../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as Displayer from "../../../ElementaryTypes/Display.ts"
import { Option } from "../../../ElementaryTypes/Option.ts"
import { Err, Ok, Tried } from "../../../ElementaryTypes/Tried.ts"
import { Unique } from "../../../ElementaryTypes/Unique.ts";

export const MIN_LENGTH = 1
export const MAX_LENGTH = 40

export type Datum = Unique<"App.ShadowVault.Datum", string>

export function isValidDatum(string: string): boolean {
  return string.length >= MIN_LENGTH 
      && string.length <= MAX_LENGTH
}

export function create(string: string): Tried<Datum, CreateError.Error> {
  if (isValidDatum(string)) {
    return Err(CreateError.InvalidLength(string))
  } else {
    return Ok(Unique(string))
  }
}

export function createOrThrow(string: string): Datum {
  if (isValidDatum(string)) {
    return Unique(string)
  } else{
    throw new Error(CreateError.displayer.display(CreateError.InvalidLength(string)))
  }
}

export function asString(me: Datum): string {
  return me
}

export function length(me: Datum): number {
  return me.length
}

export function isEqualTo(lhs: Datum, rhs: Datum): boolean {
  return lhs === rhs
}

export const displayer = Displayer.implement<Datum>(me => 
  Displayer.asWrappedString("Datum", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Datum>(me => 
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Datum>(context => 
  Tried.andThen(JsonDeserializer.asString(context), string => Tried.mapErr(
    create(string),
    error => JsonDeserializer.err(CreateError.displayer.display(error))
  ))
)

export const displayerOptional = Option.Displayer(displayer)
export const jsonSerializerOptional = Option.JsonSerializer(jsonSerializer)
export const jsonDeserializerOptional = Option.JsonDeserializer(jsonDeserializer)