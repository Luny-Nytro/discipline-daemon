import { AndStatusIndicator } from "./and_status_indicator.ts";
import { HourRangeStatusIndicator } from "./hour_range_status_indicator.ts";
import { MinuteRangeStatusIndicator } from "./minute_range_status_indicator.ts";
import { DurationStatusIndicator } from "./mod.ts";
import { OrStatusIndicator } from "./or_status_indicator.ts";
import { TimeRangeStatusIndicator } from "./time_range_status_indicator.ts";
import { WeekdayRangeStatusIndicator } from "./weekday_range_status_indicator.ts";

export type StatusIndicator = (
  | OrStatusIndicator
  | AndStatusIndicator
  | TimeRangeStatusIndicator
  | HourRangeStatusIndicator
  | MinuteRangeStatusIndicator
  | WeekdayRangeStatusIndicator
  | DurationStatusIndicator
)