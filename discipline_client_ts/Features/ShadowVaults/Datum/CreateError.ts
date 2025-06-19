import * as Displayer from "../../../ElementaryTypes/Display.ts"
import { Unique } from "../../../ElementaryTypes/Unique.ts";
import { MIN_LENGTH, MAX_LENGTH } from "./Datum.ts"

const enum Type {
  InvalidLength
}

export type Error = Unique<"Discipline.ShadowVaults.Datum.CreateError", {
  readonly type: Type.InvalidLength
  readonly providedString: string
}>

export function InvalidLength(providedString: string): Error {
  return Unique({
    type: Type.InvalidLength,
    providedString,
  })
}

export interface Cases<A> {
  InvalidLength: (providedString: string) => A
}

export function match<A>(
  me: Error,
  cases: Cases<A>
):
  A 
{
  switch (me.type) {
    case Type.InvalidLength: {
      return cases.InvalidLength(me.providedString)
    }
  } 
}

export const displayer = Displayer.implement<Error>(me => 
  match(me, {
    InvalidLength: providedString => 
      Displayer.asNamedObject("CreateShadowVaultNameError", 
        "message", Displayer.stringDisplayer, `Failed to create a shadow vault datum from provided string: provided string is ${providedString.length} characters in length while valid shadow vault datum length range is ${MIN_LENGTH} ..= ${MAX_LENGTH}.`,
        "providedString", Displayer.stringDisplayer, providedString,
      ),
  })
)