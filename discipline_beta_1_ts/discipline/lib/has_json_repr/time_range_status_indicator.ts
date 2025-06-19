import * as JSON from "./mod.ts"
import { TimeRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export interface JsonRepr {
  readonly kind: StatusIndicatorKind.TimeRange
  readonly from: JSON.Time.JsonRepr
  readonly till: JSON.Time.JsonRepr
}

export function serialize(statusIndicator: TimeRangeStatusIndicator): JsonRepr {
  return {
    kind: statusIndicator.kind,
    from: JSON.Time.serialize(statusIndicator.from),
    till: JSON.Time.serialize(statusIndicator.till),
  }
}

export function deserialize(jsonRepr: JsonRepr): TimeRangeStatusIndicator {
  return new TimeRangeStatusIndicator(
    JSON.Time.deserialize(jsonRepr.from),
    JSON.Time.deserialize(jsonRepr.till),
  )
}