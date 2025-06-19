import * as Displayer from "../ElementaryTypes/Display.ts"
import { None, Option, Some } from "../ElementaryTypes/Option.ts"
import * as JsonSerializer from "../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../ElementaryTypes/Tried.ts";
import { Unique } from "../ElementaryTypes/Unique.ts";

/**
 * Linux rules for a username are:
 * - It must be 1 to 32 characters long.
 * - It can only contain lowercase letters (a-z), digits (0-9), dashes (-), and underscores (_).
 * - It must start with a letter
 */
const LINUX_USERNAME_PATTERN = /^[a-z_][a-z0-9_-]{0,31}$/u

export function isValidUsername(username: string): boolean {
  return LINUX_USERNAME_PATTERN.test(username)
}

export type OperatingSystemUsername = Unique<"Discipline.Common.OperatingSystemUsername", string>

export function create(username: string): Option<OperatingSystemUsername> {
  if (isValidUsername(username)) {
    return Some(Unique(username))
  } else {
    return None()
  }
}

export function createOrThrow(username: string,): OperatingSystemUsername {
  if (isValidUsername(username)) {
    return Unique(username)
  } else {
    throw new Error(`OperatingSystemUsername.createOrThrow: Argument "username" is an invalid operating system username. Argument: ${username}`)
  }
}

export function asString(me: OperatingSystemUsername): string {
  return me
}

export function isEqualTo(me: OperatingSystemUsername, other: OperatingSystemUsername): boolean {
  return me === other
}

export const displayer = Displayer.implement<OperatingSystemUsername>(me =>
  Displayer.asWrappedString("OperatingSystemUsername", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<OperatingSystemUsername>(me => 
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<OperatingSystemUsername>(context =>
  Tried.andThen(JsonDeserializer.asString(context), string => Option.okOrElse(
    create(string),
    // TODO: Write a more descriptive error message.
    () => JsonDeserializer.err(`OperationSystemUsername: Invalid username`)
  ))
)
