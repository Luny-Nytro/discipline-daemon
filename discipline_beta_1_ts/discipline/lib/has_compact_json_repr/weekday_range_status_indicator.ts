import { Weekday, WeekdayRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export type JsonRepr = [
  kind: StatusIndicatorKind.WeekdayRange,
  from: number,
  till: number,
]

export function serialize(statusIndicator: WeekdayRangeStatusIndicator): JsonRepr {
  return [
    statusIndicator.kind,
    statusIndicator.from.value0(),
    statusIndicator.till.value0(),
  ]
}

export function deserialize(jsonRepr: JsonRepr): WeekdayRangeStatusIndicator {
  return new WeekdayRangeStatusIndicator(
    Weekday.fromNumber0(jsonRepr[1]).unwrap(),
    Weekday.fromNumber0(jsonRepr[2]).unwrap(),
  )
}