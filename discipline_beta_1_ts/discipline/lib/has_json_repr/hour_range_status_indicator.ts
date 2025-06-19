import { Hour, HourRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export interface JsonRepr {
  readonly kind: StatusIndicatorKind.HourRange
  readonly from: number
  readonly till: number
}

export function serialize(statusIndicator: HourRangeStatusIndicator): JsonRepr {
  return {
    kind: statusIndicator.kind,
    from: statusIndicator.from.value1_24(),
    till: statusIndicator.till.value1_24(),
  }
}

export function deserialize(jsonRepr: JsonRepr): HourRangeStatusIndicator {
  return new HourRangeStatusIndicator(
    Hour.new1_24Unchecked(jsonRepr.from),
    Hour.new1_24Unchecked(jsonRepr.till),
  )
}