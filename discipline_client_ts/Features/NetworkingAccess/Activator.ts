import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as CountdownTimer from "../../ChronicTypes/CountdownTimer.ts"
import * as DateTime from "../../ChronicTypes/DateTime.ts"
import * as Hour from "../../ChronicTypes/Hour.ts"
import * as TimeRange from "../../ChronicTypes/TimeRange.ts"
import * as Weekday from "../../ChronicTypes/Weekday.ts"
import * as WeekdayRange from "../../ChronicTypes/WeekdayRange.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

// export const TypeAtWeekdayAsString = "AtWeekday"
// export const TypeNotAtWeekdayAsString = "NotAtWeekday"
// export const TypeInTimeRangeAsString = "InTimeRange"
// export const TypeNotInTimeRangeAsString = "NotInTimeRange"
// export const TypeAtHourAsString = "AtHour"
// export const TypeNotAtHourAsString = "NotAtHour"
// export const TypeForDurationAsString = "CountdownTimer"
// export const TypeInWeekdayRangeAsString = "InWeekdayRange"
// export const TypeNotInWeekdayRangeAsString = "NotInWeekdayRange"

export const enum ActivatorType {
  AtWeekday,
  NotAtWeekday,
  InTimeRange,
  NotInTimeRange,
  AtHour,
  NotAtHour,
  ForDuration,
  InWeekdayRange,
  NotInWeekdayRange,  
}

export interface ActivatorCases<A, B, C, D, E, F, G, H, I> {
  readonly AtWeekday: (weekday: Weekday.Weekday) => A
  readonly NotAtWeekday: (weekday: Weekday.Weekday) => B
  readonly InTimeRange: (timeRange: TimeRange.TimeRange) => C
  readonly NotInTimeRange: (timeRange: TimeRange.TimeRange) => D
  readonly AtHour: (hour: Hour.Hour) => E
  readonly NotAtHour: (hour: Hour.Hour) => F
  readonly ForDuration: (timer: CountdownTimer.CountdownTimer) => G
  readonly InWeekdayRange: (weekdayRange: WeekdayRange.WeekdayRange) => H
  readonly NotInWeekdayRange: (weekdayRange: WeekdayRange.WeekdayRange) => I
}

export type Activator = Unique<"Discipline.NetworkingAccess.Activator", {
  readonly type: ActivatorType.AtWeekday
  readonly weekday: Weekday.Weekday
} | {
  readonly type: ActivatorType.NotAtWeekday
  readonly weekday: Weekday.Weekday
} | {
  readonly type: ActivatorType.InTimeRange
  readonly timeRange: TimeRange.TimeRange
} | {
  readonly type: ActivatorType.NotInTimeRange
  readonly timeRange: TimeRange.TimeRange
} | {
  readonly type: ActivatorType.AtHour
  readonly hour: Hour.Hour
} | {
  readonly type: ActivatorType.NotAtHour
  readonly hour: Hour.Hour
} | {
  readonly type: ActivatorType.ForDuration
  readonly timer: CountdownTimer.CountdownTimer
} | {
  readonly type: ActivatorType.InWeekdayRange
  readonly weekdayRange: WeekdayRange.WeekdayRange
} | {
  readonly type: ActivatorType.NotInWeekdayRange
  readonly weekdayRange: WeekdayRange.WeekdayRange
}>

export function AtWeekday(weekday: Weekday.Weekday): Activator {
  return Unique({
    type: ActivatorType.AtWeekday, 
    weekday,
  })
}
export function NotAtWeekday(weekday: Weekday.Weekday): Activator {
  return Unique({
    type: ActivatorType.NotAtWeekday, 
    weekday,
  })
}
export function InTimeRange(timeRange: TimeRange.TimeRange): Activator {
  return Unique({
    type: ActivatorType.InTimeRange, 
    timeRange,
  })
}
export function NotInTimeRange(timeRange: TimeRange.TimeRange): Activator {
  return Unique({
    type: ActivatorType.NotInTimeRange, 
    timeRange,
  })
}
export function AtHour(hour: Hour.Hour): Activator {
  return Unique({
    type: ActivatorType.AtHour, 
    hour,
  })
}
export function NotAtHour(hour: Hour.Hour): Activator {
  return Unique({
    type: ActivatorType.NotAtHour, 
    hour,
  })
}
export function ForDuration(timer: CountdownTimer.CountdownTimer): Activator {
  return Unique({
    type: ActivatorType.ForDuration, 
    timer,
  })
}
export function InWeekdayRange(weekdayRange: WeekdayRange.WeekdayRange): Activator {
  return Unique({
    type: ActivatorType.InWeekdayRange, 
    weekdayRange,
  })
}
export function NotInWeekdayRange(weekdayRange: WeekdayRange.WeekdayRange): Activator {
  return Unique({
    type: ActivatorType.NotInWeekdayRange, 
    weekdayRange,
  })
}
export function match<A, B, C, D, E, F, G, H, I>(
  me: Activator,
  cases: ActivatorCases<A, B, C, D, E, F, G, H, I>
):
  A | B | C | D | E | F | G | H | I 
{
  switch (me.type) {
    case ActivatorType.AtWeekday: {
      return cases.AtWeekday(me.weekday)
    }
    case ActivatorType.NotAtWeekday: {
      return cases.NotAtWeekday(me.weekday)
    }
    case ActivatorType.InTimeRange: {
      return cases.InTimeRange(me.timeRange)
    }
    case ActivatorType.NotInTimeRange: {
      return cases.NotInTimeRange(me.timeRange)
    }
    case ActivatorType.AtHour: {
      return cases.AtHour(me.hour)
    }
    case ActivatorType.NotAtHour: {
      return cases.NotAtHour(me.hour)
    }
    case ActivatorType.ForDuration: {
      return cases.ForDuration(me.timer)
    }
    case ActivatorType.InWeekdayRange: {
      return cases.InWeekdayRange(me.weekdayRange)
    }
    case ActivatorType.NotInWeekdayRange: {
      return cases.NotInWeekdayRange(me.weekdayRange)
    }
  }
}
export function isEffective(me: Activator, now: DateTime.DateTime): boolean {
  return match(me, {
    AtHour: hour => 
      Hour.isEqualTo(hour, DateTime.hour(now)),

    AtWeekday: weekday => 
      Weekday.isEqualTo(weekday, DateTime.weekday(now)),

    ForDuration: timer => 
      CountdownTimer.isRunningUpdated(timer, now),
    
    InTimeRange: range => 
      TimeRange.containsTime(range, DateTime.time(now)),

    InWeekdayRange: range => 
      WeekdayRange.containsWeekday(range, DateTime.weekday(now)),

    NotAtHour: hour => 
      !Hour.isEqualTo(hour, DateTime.hour(now)),

    NotAtWeekday: weekday => 
      !Weekday.isEqualTo(weekday, DateTime.weekday(now)),

    NotInTimeRange: range => 
      !TimeRange.containsTime(range, DateTime.time(now)),

    NotInWeekdayRange: range => 
      !WeekdayRange.containsWeekday(range, DateTime.weekday(now)),
  })
}

export const displayer = Displayer.implement<Activator>(me => 
  match(me, {
    AtHour: value => Displayer.asEnumDataVariantUsing(
      "Activator", "AtHour", Hour.displayer, value,
    ),
    AtWeekday: value => Displayer.asEnumDataVariantUsing(
      "Activator", "AtWeekday", Weekday.displayer, value,
    ),
    ForDuration: value => Displayer.asEnumDataVariantUsing(
      "Activator", "ForDuration", CountdownTimer.displayer, value,
    ),
    InTimeRange: value => Displayer.asEnumDataVariantUsing(
      "Activator", "InTimeRange", TimeRange.displayer, value,
    ),
    InWeekdayRange: value => Displayer.asEnumDataVariantUsing(
      "Activator", "InWeekdayRange", WeekdayRange.displayer, value,
    ),
    NotAtHour: value => Displayer.asEnumDataVariantUsing(
      "Activator", "NotAtHour", Hour.displayer, value,
    ),
    NotAtWeekday: value => Displayer.asEnumDataVariantUsing(
      "Activator", "NotAtWeekday", Weekday.displayer, value,
    ),
    NotInTimeRange: value => Displayer.asEnumDataVariantUsing(
      "Activator", "NotInTimeRange", TimeRange.displayer, value,
    ),
    NotInWeekdayRange: value => Displayer.asEnumDataVariantUsing(
      "Activator", "NotInWeekdayRange", WeekdayRange.displayer, value,
    ),    
  })
)

export const jsonSerializer = JsonSerializer.implement<Activator>(me => 
  match(me, {
    AtHour: hour => JsonSerializer.asEnumDataVariant(
      "AtHour", Hour.jsonSerializer, hour,
    ),
    AtWeekday: weekday => JsonSerializer.asEnumDataVariant(
      "AtWeekday", Weekday.jsonSerializer, weekday,
    ),
    ForDuration: timer => JsonSerializer.asEnumDataVariant(
      "ForDuration", CountdownTimer.jsonSerializer, timer,
    ),
    InTimeRange: timeRange => JsonSerializer.asEnumDataVariant(
      "InTimeRange", TimeRange.jsonSerializer, timeRange,
    ),
    InWeekdayRange: weekdayRange => JsonSerializer.asEnumDataVariant(
      "InWeekdayRange", WeekdayRange.jsonSerializer, weekdayRange,
    ),
    NotAtHour: hour => JsonSerializer.asEnumDataVariant(
      "NotAtHour", Hour.jsonSerializer, hour,
    ),
    NotAtWeekday: weekday => JsonSerializer.asEnumDataVariant(
      "NotAtWeekday", Weekday.jsonSerializer, weekday,
    ),
    NotInTimeRange: timeRange => JsonSerializer.asEnumDataVariant(
      "NotInTimeRange", TimeRange.jsonSerializer, timeRange,
    ),
    NotInWeekdayRange: weekdayRange => JsonSerializer.asEnumDataVariant(
      "NotInWeekdayRange", WeekdayRange.jsonSerializer, weekdayRange,
    ),
  })
)

export const jsonDeserializer = JsonDeserializer.implement<Activator>(context => 
  JsonDeserializer.asEnum(context,
    JsonDeserializer.EnumDataVariant(
      "AtHour", Hour.jsonDeserializer, AtHour,
    ),
    JsonDeserializer.EnumDataVariant(
      "AtWeekday", Weekday.jsonDeserializer, AtWeekday,
    ),
    JsonDeserializer.EnumDataVariant(
      "ForDuration", CountdownTimer.jsonDeserializer, ForDuration,
    ),
    JsonDeserializer.EnumDataVariant(
      "InTimeRange", TimeRange.jsonDeserializer, InTimeRange,
    ),
    JsonDeserializer.EnumDataVariant(
      "InWeekdayRange", WeekdayRange.jsonDeserializer, InWeekdayRange,
    ),
    JsonDeserializer.EnumDataVariant(
      "NotAtHour", Hour.jsonDeserializer, NotAtHour,
    ),
    JsonDeserializer.EnumDataVariant(
      "NotAtWeekday", Weekday.jsonDeserializer, NotAtWeekday,
    ),
    JsonDeserializer.EnumDataVariant(
      "NotInTimeRange", TimeRange.jsonDeserializer, NotInTimeRange,
    ),
    JsonDeserializer.EnumDataVariant(
      "NotInWeekdayRange", WeekdayRange.jsonDeserializer, NotInWeekdayRange,
    ),
  )
)