import { Unique } from "../../ElementaryTypes/Unique.ts";
import * as Displayer from "../../ElementaryTypes/Display.ts"
import { JANUARY_AS_STRING } from "./Month.ts";
import { FEBRUARY_AS_STRING } from "./Month.ts";
import { MARCH_AS_STRING } from "./Month.ts";
import { APRIL_AS_STRING } from "./Month.ts";
import { MAY_AS_STRING } from "./Month.ts";
import { JUNE_AS_STRING } from "./Month.ts";
import { JULY_AS_STRING } from "./Month.ts";
import { AUGUST_AS_STRING } from "./Month.ts";
import { SEPTEMBER_AS_STRING } from "./Month.ts";
import { OCTOBER_AS_STRING } from "./Month.ts";
import { NOVEMBER_AS_STRING } from "./Month.ts";
import { DECEMBER_AS_STRING } from "./Month.ts";

export type Error = Unique<"Discipline.Chronic.Month.CreateFromStringError", {
  readonly string: string
}>

export function InvalidString(string: string): Error {
  return Unique({
    string
  })
}

export interface Cases<A> {
  readonly InvalidString: (string: string) => A
}

export function match<A>(me: Error, cases: Cases<A>): A {
  return cases.InvalidString(me.string)
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("Month.CreateFromStringError", `Expected string to be "${
    JANUARY_AS_STRING
  }", "${
    FEBRUARY_AS_STRING
  }", "${
    MARCH_AS_STRING
  }", "${
    APRIL_AS_STRING
  }", "${
    MAY_AS_STRING
  }", "${
    JUNE_AS_STRING
  }", "${
    JULY_AS_STRING
  }", "${
    AUGUST_AS_STRING
  }", "${
    SEPTEMBER_AS_STRING
  }", "${
    OCTOBER_AS_STRING
  }", "${
    NOVEMBER_AS_STRING
  }", or "${
    DECEMBER_AS_STRING
  }", but found "${me.string}"`)
)