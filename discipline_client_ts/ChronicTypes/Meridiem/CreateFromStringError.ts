import { AM_AS_STRING, PM_AS_STRING } from "./Meridiem.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type T = Unique<"Discipline.Chronic.Meridiem.CreateFromStringError", {
  string: string
}>

export function InvalidString(string: string): T {
  return Unique({
    string,
  })
}

export interface Cases<A> {
  readonly InvalidString: (string: string) => A
}

export function match<A>(me: T, cases: Cases<A>): A {
  return cases.InvalidString(me.string)
}

export const displayer = Displayer.implement<T>(me => 
  Displayer.asWrappedString("Meridiem.CreateFromStringError", `Invalid String. Expected "${
    AM_AS_STRING
  }" or "${
    PM_AS_STRING
  }", but found "${
    me.string
  }"`)
)