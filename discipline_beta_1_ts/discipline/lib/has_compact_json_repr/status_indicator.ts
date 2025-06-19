import { StatusIndicator, StatusIndicatorKind } from "Pkg"
import * as JSON from "./mod.ts"

export type JsonRepr = (
  | JSON.OrStatusIndicator.JsonRepr
  | JSON.AndStatusIndicator.JsonRepr
  | JSON.DurationStatusIndicator.JsonRepr
  | JSON.TimeRangeStatusIndicator.JsonRepr
  | JSON.HourRangeStatusIndicator.JsonRepr
  | JSON.MinuteRangeStatusIndicator.JsonRepr
  | JSON.WeekdayRangeStatusIndicator.JsonRepr
)

export function serialize(statusIndicator: StatusIndicator): JsonRepr {
  switch (statusIndicator.kind) {
    case StatusIndicatorKind.Or: return JSON.OrStatusIndicator.serialize(statusIndicator)
    case StatusIndicatorKind.And: return JSON.AndStatusIndicator.serialize(statusIndicator)
    case StatusIndicatorKind.Duration: return JSON.DurationStatusIndicator.serialize(statusIndicator)
    case StatusIndicatorKind.TimeRange: return JSON.TimeRangeStatusIndicator.serialize(statusIndicator)
    case StatusIndicatorKind.HourRange: return JSON.HourRangeStatusIndicator.serialize(statusIndicator)
    case StatusIndicatorKind.MinuteRange: return JSON.MinuteRangeStatusIndicator.serialize(statusIndicator)
    case StatusIndicatorKind.WeekdayRange: return JSON.WeekdayRangeStatusIndicator.serialize(statusIndicator)
  }
}

export function deserialize(jsonRepr: JsonRepr): StatusIndicator {
  switch (jsonRepr[0]) {
    case StatusIndicatorKind.Or: return JSON.OrStatusIndicator.deserialize(jsonRepr)
    case StatusIndicatorKind.And: return JSON.AndStatusIndicator.deserialize(jsonRepr)
    case StatusIndicatorKind.Duration: return JSON.DurationStatusIndicator.deserialize(jsonRepr)
    case StatusIndicatorKind.TimeRange: return JSON.TimeRangeStatusIndicator.deserialize(jsonRepr)
    case StatusIndicatorKind.HourRange: return JSON.HourRangeStatusIndicator.deserialize(jsonRepr)
    case StatusIndicatorKind.MinuteRange: return JSON.MinuteRangeStatusIndicator.deserialize(jsonRepr)
    case StatusIndicatorKind.WeekdayRange: return JSON.WeekdayRangeStatusIndicator.deserialize(jsonRepr)
  }
}