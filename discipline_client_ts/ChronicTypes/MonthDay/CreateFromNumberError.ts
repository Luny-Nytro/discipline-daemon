import * as Displayer from "../../ElementaryTypes/Display.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { MAX_VALUE, MIN_VALUE } from "./MonthDay.ts";

export type Error = Unique<"Discipline.Chronic.MonthDay.CreateFromNumberError", {
  readonly number: number
}>

export function InvalidNumber(number: number): Error {
  return Unique({
    number
  })
}

export interface Cases<A> {
  readonly InvalidNumber: (number: number) => A
}

export function match<A>(me: Error, cases: Cases<A>): A {
  return cases.InvalidNumber(me.number)
}

export const displayer = Displayer.implement<Error>(me =>
  Displayer.asWrappedString("MonthDay.CreateFromStringError", `Expected number to be in this range ${
    MIN_VALUE
  } ..= ${
    MAX_VALUE
  }, but found ${
    me.number
  }`)
)