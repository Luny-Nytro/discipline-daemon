import { Unique } from "../../ElementaryTypes/Unique.ts";

const enum Value {
  Intraday,
  Crossday,
}

export type RangeType = Unique<"Discipline.Chronic.TimeRange.TimeRangeType", Value>

export function Intraday(): RangeType {
  return Unique(Value.Intraday)
}

export function Crossday(): RangeType {
  return Unique(Value.Crossday)
}

export function isIntraday(me: RangeType): boolean {
  return me === Value.Intraday
}

export function isCrossday(me: RangeType): boolean {
  return me === Value.Crossday
}

export interface Cases<A, B> {
  readonly Intraday: () => A
  readonly Crossday: () => B
}

export function match<A, B>(me: RangeType, cases: Cases<A, B>): A | B {
  switch (me as Value) {
    case Value.Intraday: {
      return cases.Intraday()
    }
    case Value.Crossday: {
      return cases.Crossday()
    }
  }
}