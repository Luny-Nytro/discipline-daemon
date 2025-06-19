import { Unique } from "@Pkg/Unique";
import { TimeRange } from "@Pkg/TimeRange";
import { WeekdayRange } from "@Pkg/WeekdayRange";
import { WeeklyTimeRange } from "@Pkg/WeeklyTimeRange";
import { DateTime } from "@Pkg/DateTime";

export const enum ActivatorType {
  InTimeRange,
  InWeekdayRange,
  InWeeklyTimeRange,
}

export type Activator = Unique<"App.UserAccess.Rule.Activator", {
  readonly type: ActivatorType.InTimeRange
  readonly range: TimeRange
} | {
  readonly type: ActivatorType.InWeekdayRange
  readonly range: WeekdayRange
} | {
  readonly type: ActivatorType.InWeeklyTimeRange
  readonly range: WeeklyTimeRange
}>

export type ActivatorCases<A, B, C> = {
  readonly InTimeRange: (range: TimeRange) => A
  readonly InWeekdayRange: (range: WeekdayRange) => B
  readonly InWeeklyTimeRange: (range: WeeklyTimeRange) => C
}

export const Activator = {
  InTimeRange(range: TimeRange): Activator {
    return Unique({
      type: ActivatorType.InTimeRange,
      range,
    })
  },

  InWeekdayRange(range: WeekdayRange): Activator {
    return Unique({
      type: ActivatorType.InWeekdayRange,
      range,
    })
  },
  
  InWeeklyRange(range: WeeklyTimeRange): Activator {
    return Unique({
      type: ActivatorType.InWeeklyTimeRange,
      range,
    })
  },

  match<A, B, C>(
    me: Activator,
    cases: ActivatorCases<A, B, C>
  ): 
    A | B | C 
  {
    switch (me.type) {
      case ActivatorType.InTimeRange: {
        return cases.InTimeRange(me.range)
      }
      case ActivatorType.InWeekdayRange: {
        return cases.InWeekdayRange(me.range)
      }
      case ActivatorType.InWeeklyTimeRange: {
        return cases.InWeeklyTimeRange(me.range)
      }
    }
  },

  isRuleActive(me: Activator, now: DateTime): boolean {
    return Activator.match(me, {
      InTimeRange: range => TimeRange.containsTime(range, DateTime.time(now)),
      InWeekdayRange: range => WeekdayRange.containsWeekday(range, DateTime.weekday(now)),
      InWeeklyTimeRange: range => WeeklyTimeRange.containWeekdayAndTime(range, DateTime.weekday(now), DateTime.time(now)),
    })
  },
}