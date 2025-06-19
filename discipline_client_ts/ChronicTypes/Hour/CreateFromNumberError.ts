import * as Displayer from "../../ElementaryTypes/Display.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { MAX_VALUE, MIN_VALUE } from "./Hour.ts";

export type T = Unique<"Discipline.Chronic.Hour.CreateFromNumberError", {
  number: number
}>

export function InvalidNumber(number: number): T {
  return Unique({
    number
  })
}

export interface Cases<A> {
  readonly InvalidNumber: (number: number) => A
}

export function match<A>(me: T, cases: Cases<A>): A {
  return cases.InvalidNumber(me.number)
}

export const displayer = Displayer.implement<T>(me => 
  Displayer.asWrappedString("Hour.CreateFromNumberError", `Expected number to be in this range ${
    MIN_VALUE
  } ..= ${
    MAX_VALUE
  }, but found ${
    me.number
  }`)
)