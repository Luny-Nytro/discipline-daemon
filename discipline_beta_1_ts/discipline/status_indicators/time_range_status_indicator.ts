import { DateTime, Time } from "Pkg"
import { StatusIndicatorKind } from "./common.ts"

export class TimeRangeStatusIndicator {
  readonly kind = StatusIndicatorKind.TimeRange
  constructor(
    public from: Time,
    public till: Time,
  ) {}

  isActive(now = DateTime.now()) {
    return now.time().isWithin(this.from, this.till)
  }
}

