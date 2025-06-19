import { Hour, DateTime } from "Pkg"
import { StatusIndicatorKind } from "./common.ts"

export class HourRangeStatusIndicator {
  readonly kind = StatusIndicatorKind.HourRange
  constructor(
    readonly from: Hour,
    readonly till: Hour,
  ) {}

  isActive(now = DateTime.now()) {
    return now.hour().isWithin(this.from, this.till)
  }
}
