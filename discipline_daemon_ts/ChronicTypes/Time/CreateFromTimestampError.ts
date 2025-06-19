import { Unique } from "../../ElementaryTypes/Unique.ts";
import * as Displayer from "../../ElementaryTypes/Display.ts"
import { MAX_TIMESTAMP, MIN_TIMESTAMP } from "./Time.ts";

export type Error = Unique<"App.Chronic.Time.CreateFromTimeStampError", {
  readonly timestamp: number
}>

export function InvalidTimestamp(timestamp: number): Error {
  return Unique({
    timestamp
  })
}

export interface Cases<A> {
  readonly InvalidTimestamp: (timestamp: number) => A
}

export function match<A>(me: Error, cases: Cases<A>): A {
  return cases.InvalidTimestamp(me.timestamp)
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("Time.CreateFromTimestampError", `Expected timestamp to be in this range ${
    MIN_TIMESTAMP
  } ..= ${
    MAX_TIMESTAMP
  }, but found ${
    me.timestamp
  }`)
)