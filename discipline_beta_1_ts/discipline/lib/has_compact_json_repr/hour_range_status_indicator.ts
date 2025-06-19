import { Hour, HourRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export type JsonRepr = [
  kind: StatusIndicatorKind.HourRange,
  from: number,
  till: number,
]

export function serialize(statusIndicator: HourRangeStatusIndicator): JsonRepr {
  return [
    statusIndicator.kind,
    statusIndicator.from.value1_24(),
    statusIndicator.till.value1_24(),
  ]
}

export function deserialize(jsonRepr: JsonRepr): HourRangeStatusIndicator {
  return new HourRangeStatusIndicator(
    Hour.new1_24Unchecked(jsonRepr[1]),
    Hour.new1_24Unchecked(jsonRepr[2]),
  )
}