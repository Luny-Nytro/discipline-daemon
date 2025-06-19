import * as Displayer from "../../ElementaryTypes/Display.ts"
import { MAX_VALUE, MIN_VALUE } from "./Minute.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type Error = Unique<"Discipline.Chronic.Minute.CreateFromNumberError", {
  number: number
}>

export function InvalidNumber(number: number): Error {
  return Unique({
    number,
  })
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("Minute.CreateFromNumberError", `Expected a number in this range ${MIN_VALUE} ..= ${MAX_VALUE}, but found ${me.number}`)
)

export interface Cases<A> {
  readonly InvalidNumber: (number: number) => A
}

export function match<A>(me: Error, cases: Cases<A>): A {
  return cases.InvalidNumber(me.number)
}