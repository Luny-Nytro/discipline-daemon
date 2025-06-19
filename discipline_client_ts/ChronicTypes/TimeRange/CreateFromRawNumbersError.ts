import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as Time from "../Time.ts"
import * as RangeType from "./TimeRangeType.ts"
import * as Duration from "../Duration.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { MAX_FROM_VALUE, MAX_TILL_VALUE, MIN_FROM_VALUE, MIN_TILL_VALUE } from "./TimeRange.ts";

const enum Type {
  InvalidFrom,
  InvalidTill,
  InvalidDuration,
  FromLaterThanTill
}

export type Error = Unique<"Discipline.Chronic.TimeRange.CreateFromRawNumbersError", {
  readonly type: Type.InvalidFrom
  readonly from: number
} | {
  readonly type: Type.InvalidTill
  readonly till: number
} | {
  readonly type: Type.FromLaterThanTill
  readonly from: Time.Time
  readonly till: Time.Time
  readonly rangeType: RangeType.RangeType
} | {
  readonly type: Type.InvalidDuration
  readonly from: Time.Time
  readonly till: Time.Time
  readonly rangeType: RangeType.RangeType
  readonly duration: Duration.Duration
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
  from: Time.Time, 
  till: Time.Time,
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
  from: Time.Time, 
  till: Time.Time,
  rangeType: RangeType.RangeType,
): Error {
  return Unique({
    type: Type.InvalidDuration,
    from,
    till,
    rangeType,
    duration: RangeType.isIntraday(rangeType)
      ? Duration.fromMillisecondsOrThrow(Time.asTimestamp(till) - Time.asTimestamp(from))
      : Duration.fromMillisecondsOrThrow(Time.asTimestamp(till) + Time.MAX_TIMESTAMP - Time.asTimestamp(from)),
  })
}

export interface Cases<A, B, C, D> {
  readonly InvalidFrom: (from: number) => A
  readonly InvalidTill: (till: number) => B
  readonly FromLaterThanTill: (from: Time.Time, till: Time.Time, rangeTime: RangeType.RangeType) => C
  readonly InvalidDuration: (from: Time.Time, till: Time.Time, rangeType: RangeType.RangeType, duration: Duration.Duration) => D
}

export function match<A, B, C, D>(me: Error, cases: Cases<A, B, C, D>): A | B | C | D {
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
        `Expected "from" to be an integer in this range ${MIN_FROM_VALUE} ..= ${MAX_FROM_VALUE}, but found ${from}. "from" is a millisecond-based timestamp since midnight of a day till the end of that day.`
      ),

    InvalidTill: till => 
      Displayer.asEnumStringVariant("CreateFromRawNumbersError", "InvalidTill",
        `Expected "till" to be an integer in this range ${MIN_TILL_VALUE} ..= ${MAX_TILL_VALUE}, but found ${till}. "till" is a millisecond-based timestamp since midnight of a day till the end of the next day.`
      ),

    FromLaterThanTill: (from, till, rangeType) => 
      Displayer.asEnumStringVariant("CreateFromRawNumbersError", "FromLaterThanTill", 
        `Expected "from" to be earlier than "till". "from" is ${
          Time.asString(from, { hour12: true })
        }. "till" is ${
          Time.asString(till, { hour12: true })
        } ${
          RangeType.match(rangeType, {
            Intraday: () => `of the same day as "from"`,
            Crossday: () => `of the next day of "from"`
          })
        }`),

    InvalidDuration: (from, till, rangeType, duration) => 
      Displayer.asEnumStringVariant("CreateFromRawNumbersError", "InvalidDuration", 
        `Expected duration between "from" and "till" to not be greater than a single day. "from" is ${
        Time.asString(from, { hour12: true })
      }. "till" is ${
        Time.asString(till, { hour12: true })
      } ${
        RangeType.match(rangeType, {
          Intraday: () => `of the same day as "from"`,
          Crossday: () => `of the next day of "from"`
        })
      }. Duration is ${
        Duration.asString(duration)
      }`),
  })
)