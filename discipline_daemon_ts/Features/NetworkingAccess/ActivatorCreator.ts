import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as Duration from "../../ChronicTypes/Duration.ts"
import * as Hour from "../../ChronicTypes/Hour.ts"
import * as TimeRange from "../../ChronicTypes/TimeRange.ts"
import * as Weekday from "../../ChronicTypes/Weekday.ts"
import * as WeekdayRange from "../../ChronicTypes/WeekdayRange.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { ActivatorType } from "./Activator.ts"

export interface Cases<A, B, C, D, E, F, G, H, I> {
  readonly AtWeekday: (weekday: Weekday.Weekday) => A
  readonly NotAtWeekday: (weekday: Weekday.Weekday) => B
  readonly InTimeRange: (timeRange: TimeRange.TimeRange) => C
  readonly NotInTimeRange: (timeRange: TimeRange.TimeRange) => D
  readonly AtHour: (hour: Hour.Hour) => E
  readonly NotAtHour: (hour: Hour.Hour) => F
  readonly ForDuration: (duration: Duration.Duration) => G
  readonly InWeekdayRange: (weekdayRange: WeekdayRange.WeekdayRange) => H
  readonly NotInWeekdayRange: (weekdayRange: WeekdayRange.WeekdayRange) => I
}

export type ActivatorCreator = Unique<"App.NetworkingAccess.Activator.Creator", {
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
  readonly duration: Duration.Duration
} | {
  readonly type: ActivatorType.InWeekdayRange
  readonly weekdayRange: WeekdayRange.WeekdayRange
} | {
  readonly type: ActivatorType.NotInWeekdayRange
  readonly weekdayRange: WeekdayRange.WeekdayRange
}>

export function AtWeekday(weekday: Weekday.Weekday): ActivatorCreator {
  return Unique({
    type: ActivatorType.AtWeekday, 
    weekday,
  })
}
export function NotAtWeekday(weekday: Weekday.Weekday): ActivatorCreator {
  return Unique({
    type: ActivatorType.NotAtWeekday, 
    weekday,
  })
}
export function InTimeRange(timeRange: TimeRange.TimeRange): ActivatorCreator {
  return Unique({
    type: ActivatorType.InTimeRange, 
    timeRange,
  })
}
export function NotInTimeRange(timeRange: TimeRange.TimeRange): ActivatorCreator {
  return Unique({
    type: ActivatorType.NotInTimeRange, 
    timeRange,
  })
}
export function AtHour(hour: Hour.Hour): ActivatorCreator {
  return Unique({
    type: ActivatorType.AtHour, 
    hour,
  })
}
export function NotAtHour(hour: Hour.Hour): ActivatorCreator {
  return Unique({
    type: ActivatorType.NotAtHour, 
    hour,
  })
}
export function ForDuration(duration: Duration.Duration): ActivatorCreator {
  return Unique({
    type: ActivatorType.ForDuration, 
    duration,
  })
}
export function InWeekdayRange(weekdayRange: WeekdayRange.WeekdayRange): ActivatorCreator {
  return Unique({
    type: ActivatorType.InWeekdayRange, 
    weekdayRange,
  })
}
export function NotInWeekdayRange(weekdayRange: WeekdayRange.WeekdayRange): ActivatorCreator {
  return Unique({
    type: ActivatorType.NotInWeekdayRange, 
    weekdayRange,
  })
}
export function match<A, B, C, D, E, F, G, H, I>(
  me: ActivatorCreator,
  cases: Cases<A, B, C, D, E, F, G, H, I>
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
      return cases.ForDuration(me.duration)
    }
    case ActivatorType.InWeekdayRange: {
      return cases.InWeekdayRange(me.weekdayRange)
    }
    case ActivatorType.NotInWeekdayRange: {
      return cases.NotInWeekdayRange(me.weekdayRange)
    }
  }
}

export const displayer = Displayer.implement<ActivatorCreator>(me => 
  match(me, {
    AtHour: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "AtHour", Hour.displayer, value,
    ),
    AtWeekday: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "AtWeekday", Weekday.displayer, value,
    ),
    ForDuration: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "ForDuration", Duration.displayer, value,
    ),
    InTimeRange: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "InTimeRange", TimeRange.displayer, value,
    ),
    InWeekdayRange: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "InWeekdayRange", WeekdayRange.displayer, value,
    ),
    NotAtHour: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "NotAtHour", Hour.displayer, value,
    ),
    NotAtWeekday: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "NotAtWeekday", Weekday.displayer, value,
    ),
    NotInTimeRange: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "NotInTimeRange", TimeRange.displayer, value,
    ),
    NotInWeekdayRange: value => Displayer.asEnumDataVariantUsing(
      "ActivatorCreator", "NotInWeekdayRange", WeekdayRange.displayer, value,
    ),
  })
)

export const jsonSerializer = JsonSerializer.implement<ActivatorCreator>(me => 
  match(me, {
    AtHour: hour => JsonSerializer.asEnumDataVariant(
      "AtHour", Hour.jsonSerializer, hour,
    ),
    AtWeekday: weekday => JsonSerializer.asEnumDataVariant(
      "AtWeekday", Weekday.jsonSerializer, weekday,
    ),
    ForDuration: timer => JsonSerializer.asEnumDataVariant(
      "ForDuration", Duration.jsonSerializer, timer,
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

export const jsonDeserializer = JsonDeserializer.implement<ActivatorCreator>(context => 
  JsonDeserializer.asEnum(context, 
    JsonDeserializer.EnumDataVariant(
      "AtHour", Hour.jsonDeserializer, AtHour,
    ),
    JsonDeserializer.EnumDataVariant(
      "AtWeekday", Weekday.jsonDeserializer, AtWeekday,
    ),
    JsonDeserializer.EnumDataVariant(
      "ForDuration", Duration.jsonDeserializer, ForDuration,
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
