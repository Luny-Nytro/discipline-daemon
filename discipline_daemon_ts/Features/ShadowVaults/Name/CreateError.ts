import * as Displayer from "../../../ElementaryTypes/Display.ts"
import { Unique } from "../../../ElementaryTypes/Unique.ts";
import { MAX_LENGTH, MIN_LENGTH } from "./Name.ts";

const enum Type {
  InvalidLength,
}

export type Error = Unique<"App.ShadowVaults.Name.CreateError", {
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
        "message", Displayer.stringDisplayer, `Failed to create a shadow vault name from provided string: provided string is ${providedString.length} characters in length while valid shadow vault name length range is ${MIN_LENGTH} ..= ${MAX_LENGTH}.`,
        "providedString", Displayer.stringDisplayer, providedString,
      ),
  })
)