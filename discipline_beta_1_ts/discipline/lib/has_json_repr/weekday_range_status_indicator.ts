import { Weekday, WeekdayRangeStatusIndicator, StatusIndicatorKind } from "Pkg"

export interface JsonRepr {
  readonly kind: StatusIndicatorKind.WeekdayRange
  readonly from: number
  readonly till: number
}

export function serialize(statusIndicator: WeekdayRangeStatusIndicator): JsonRepr {
  return {
    kind: statusIndicator.kind,
    from: statusIndicator.from.value0(),
    till: statusIndicator.till.value0(),
  }
}

export function deserialize(jsonRepr: JsonRepr): WeekdayRangeStatusIndicator {
  return new WeekdayRangeStatusIndicator(
    Weekday.fromNumber0(jsonRepr.from).unwrap(),
    Weekday.fromNumber0(jsonRepr.till).unwrap(),
  )
}