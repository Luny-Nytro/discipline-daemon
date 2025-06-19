import { Unique } from "../../ElementaryTypes/Unique.ts";

const enum Value {
  IntraWeek,
  CrossWeek,
}

export type RangeType = Unique<"Discipline.Chronic.WeekdayRange.TimeRangeType", Value>

export function IntraWeek(): RangeType {
  return Unique(Value.IntraWeek)
}

export function CrossWeek(): RangeType {
  return Unique(Value.CrossWeek)
}

export function isIntrWeek(me: RangeType): boolean {
  return me === Value.IntraWeek
}

export function isCrossWeek(me: RangeType): boolean {
  return me === Value.CrossWeek
}

export interface Cases<A, B> {
  readonly IntraWeek: () => A
  readonly CrossWeek: () => B
}

export function match<A, B>(me: RangeType, cases: Cases<A, B>): A | B {
  switch (me as Value) {
    case Value.IntraWeek: {
      return cases.IntraWeek()
    }
    case Value.CrossWeek: {
      return cases.CrossWeek()
    }
  }
}