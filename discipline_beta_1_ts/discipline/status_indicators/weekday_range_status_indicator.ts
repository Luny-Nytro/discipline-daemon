import { Weekday, DateTime } from "Pkg"
import { StatusIndicatorKind } from "./common.ts"

export class WeekdayRangeStatusIndicator {
  readonly kind = StatusIndicatorKind.WeekdayRange
  constructor(
    readonly from: Weekday,
    readonly till: Weekday,
  ) {}

  isActive(now = DateTime.now()) {
    return now.weekday().isWithin(this.from, this.till)
  }
}