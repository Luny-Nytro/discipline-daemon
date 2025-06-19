import { Unique } from "@Pkg/Unique";
import { TimeRange } from "@Pkg/TimeRange";
import { WeekdayRange } from "@Pkg/WeekdayRange";
import { Weekday } from "@Pkg/Weekday";
import { Time } from "@Pkg/Time";

export type WeeklyTimeRange = Unique<"App.Chronic.WeeklyTimeRange", {
  readonly timeRange: TimeRange
  readonly weekdayRange: WeekdayRange
}>

export const WeeklyTimeRange = {
  containWeekdayAndTime(me: WeeklyTimeRange, weekday: Weekday, time: Time): boolean {
    return WeekdayRange.containsWeekday(me.weekdayRange, weekday)
      && TimeRange.containsTime(me.timeRange, time)
  }
}