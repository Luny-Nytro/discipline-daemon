import * as JSON from "./mod.ts"
import { TimeRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export type JsonRepr = [
  kind: StatusIndicatorKind.TimeRange,
  from: JSON.Time.JsonRepr,
  till: JSON.Time.JsonRepr,
]

export function serialize(statusIndicator: TimeRangeStatusIndicator): JsonRepr {
  return [
    statusIndicator.kind,
    JSON.Time.serialize(statusIndicator.from),
    JSON.Time.serialize(statusIndicator.till),
  ]
}

export function deserialize(jsonRepr: JsonRepr): TimeRangeStatusIndicator {
  return new TimeRangeStatusIndicator(
    JSON.Time.deserialize(jsonRepr[1]),
    JSON.Time.deserialize(jsonRepr[2]),
  )
}