import { Minute, MinuteRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export type JsonRepr = [
  kind: StatusIndicatorKind.MinuteRange,
  from: number,
  till: number,
]

export function serialize(statusIndicator: MinuteRangeStatusIndicator): JsonRepr {
  return [
    statusIndicator.kind,
    statusIndicator.from.value1(),
    statusIndicator.till.value1(),
  ]
}

export function deserialize(jsonRepr: JsonRepr) {
  return new MinuteRangeStatusIndicator(
    Minute.new1Unchecked(jsonRepr[1]),
    Minute.new1Unchecked(jsonRepr[2]),
  )
}