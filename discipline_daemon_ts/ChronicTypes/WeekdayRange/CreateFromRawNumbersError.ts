import { Unique } from "../../ElementaryTypes/Unique.ts";
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as Duration from "../Duration.ts"
import * as Weekday from "../Weekday.ts";
import * as RangeType from "./RangeType.ts";
import { MAX_FROM_VALUE, MAX_TILL_VALUE, MIN_FROM_VALUE, MIN_TILL_VALUE } from "./WeekdayRange.ts";

const enum Type {
  InvalidFrom,
  InvalidTill,
  InvalidDuration,
  FromLaterThanTill
}

export type Error = Unique<"App.Chronic.WeekdayRange.CreateFromRawNumbersError", {
  readonly type: Type.InvalidFrom
  readonly from: number
} | {
  readonly type: Type.InvalidTill
  readonly till: number
} | {
  readonly type: Type.FromLaterThanTill
  readonly from: Weekday.Weekday
  readonly till: Weekday.Weekday
  readonly rangeType: RangeType.RangeType
} | {
  readonly type: Type.InvalidDuration
  readonly from: Weekday.Weekday
  readonly till: Weekday.Weekday
  readonly duration: Duration.Duration
  readonly rangeType: RangeType.RangeType
}>

export function InvalidFrom(from: number): Error {
  return Unique({
    type: Type.InvalidFrom,
    from,
  })
}

export function InvalidTill(till: number): Error {
  return Unique({
    type: Type.InvalidTill,
    till,
  })
}

export function FromLaterThanTill(
  from: Weekday.Weekday, 
  till: Weekday.Weekday,
  rangeType: RangeType.RangeType,
): Error {
  return Unique({
    type: Type.FromLaterThanTill,
    from,
    till,
    rangeType,
  })
}

export function InvalidDuration(
  from: Weekday.Weekday, 
  till: Weekday.Weekday,
  rangeType: RangeType.RangeType,
): Error {
  return Unique({
    type: Type.InvalidDuration,
    from,
    till,
    duration: Duration.fromMillisecondsOrThrow(till - from),
    rangeType,
  })
}

export interface Cases<A, B, C, D> {
  readonly InvalidFrom: (
    from: number,
  ) => A
  readonly InvalidTill: (
    till: number,
  ) => B
  readonly FromLaterThanTill: (
    from: Weekday.Weekday,
    till: Weekday.Weekday,
    rangeType: RangeType.RangeType,
  ) => C
  readonly InvalidDuration: (
    from: Weekday.Weekday, 
    till: Weekday.Weekday, 
    rangeType: RangeType.RangeType, 
    duration: Duration.Duration,
  ) => D
}

export function match<A, B, C, D>(
  me: Error,
  cases: Cases<A, B, C, D>
): A | B | C | D {
  switch (me.type) {
    case Type.InvalidFrom: {
      return cases.InvalidFrom(me.from)
    }
    case Type.InvalidTill: {
      return cases.InvalidTill(me.till)
    }
    case Type.InvalidDuration: {
      return cases.InvalidDuration(me.from, me.till, me.rangeType, me.duration)
    }
    case Type.FromLaterThanTill: {
      return cases.FromLaterThanTill(me.from, me.till, me.rangeType)
    }
  }
}

export const displayer = Displayer.implement<Error>(me => 
  match(me, {
    InvalidFrom: from => 
      Displayer.asEnumStringVariant("CreateFromRawNumbersError", "InvalidFrom", 
        `Expected "from" to be in this range ${MIN_FROM_VALUE} ..= ${MAX_FROM_VALUE}, but found ${from}.`
      ),

    InvalidTill: till => 
      Displayer.asEnumStringVariant("CreateFromRawNumbersError", "InvalidTill",
        `Expected "till" to be in this range ${MIN_TILL_VALUE} ..= ${MAX_TILL_VALUE}, but found ${till}.`
      ),

    FromLaterThanTill: (from, till, rangeType) =>
      Displayer.asEnumStringVariant("CreateFromRawNumbersError", "FromLaterThanTill",
        `Expected "from" to be earlier than "till". "from" is ${
          Weekday.asString(from)
        }. "till" is ${
          Weekday.asString(till)
        } ${
          RangeType.match(rangeType, {
            IntraWeek: () => `of the same week as "from"`,
            CrossWeek: () => `of the next week of "from"`,
          })
        }.`
      ),

    InvalidDuration: (from, till, rangeType, duration) =>
      Displayer.asEnumStringVariant("CreateFromRawNumbersError", "InvalidDuration", 
        `Expected duration between "from" and "till" to not be greater than a single day. "from" is ${
        Weekday.asString(from)
      }. "till" is ${
        Weekday.asString(till)
      } ${
        RangeType.match(rangeType, {
          IntraWeek: () => `of the same week as "from"`,
          CrossWeek: () => `of the next week of "from"`,
        })
      }. Duration is ${
        Duration.asString(duration)
      }.`
    ),

  })
)