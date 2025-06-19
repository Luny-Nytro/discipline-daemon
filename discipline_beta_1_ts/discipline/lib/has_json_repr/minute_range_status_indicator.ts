import { Minute, MinuteRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export interface JsonRepr {
  readonly kind: StatusIndicatorKind.MinuteRange
  readonly from: number
  readonly till: number
}

export function serialize(statusIndicator: MinuteRangeStatusIndicator): JsonRepr {
  return {
    kind: statusIndicator.kind,
    from: statusIndicator.from.value1(),
    till: statusIndicator.till.value1(),
  }
}

export function deserialize(jsonRepr: JsonRepr) {
  return new MinuteRangeStatusIndicator(
    Minute.new1Unchecked(jsonRepr.from),
    Minute.new1Unchecked(jsonRepr.till),
  )
}