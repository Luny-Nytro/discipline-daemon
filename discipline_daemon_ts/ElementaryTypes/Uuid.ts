import * as Displayer from "./Display.ts";
import { Option, None, Some } from "./Option.ts";
import * as JsonSerializer from "./JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "./JsonSerde/JsonDeserializer.ts";
import { Tried } from "./Tried.ts";
import { Unique } from "./Unique.ts";

// Genetated with ChatGPT. 
// TODO: Verify this is correct.
const UUIDV4Regex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

export function isValidUUIDv4String(string: string): boolean {
  return UUIDV4Regex.test(string)
}

export type Uuid = Unique<"App.Elementary.Uuid", string>

export function generate(): Uuid {
  return Unique(crypto.randomUUID())
}

export function fromString(string: string): Option<Uuid> {
  if (isValidUUIDv4String(string)) {
    return Some(Unique(string))
  } else {
    return None()
  }
}

export function fromStringOrThrow(string: string): Uuid {
  if (isValidUUIDv4String(string)) {
    return Unique(string)
  } else {
    throw new Error(`UUID.FromStringOrThrow: Argument isn't valid uuid v4. Argument: ${string}`)
  }
}

export function asString(me: Uuid): string {
  return me
}

export function isEqualTo(me: Uuid, other: Uuid): boolean {
  return me === other
}

export const displayer = Displayer.implement<Uuid>(me => 
  Displayer.asWrappedString("Uuid", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Uuid>(me =>
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Uuid>(context =>
  Tried.andThen(JsonDeserializer.asString(context), string => Option.okOrElse(
    fromString(string),
    () => JsonDeserializer.err(`Uuid.CreateFromString: String is invalid UUID v4. String is ${string}`)
  ))
)

export const displayerOptional = Option.Displayer(displayer)
export const jsonSerializerOptional = Option.JsonSerializer(jsonSerializer)
export const jsonDeserializerOptional = Option.JsonDeserializer(jsonDeserializer);


export const Uuid = {
  isValidUUIDv4String,
  generate,
  fromString,
  fromStringOrThrow,
  asString,
  isEqualTo,
};