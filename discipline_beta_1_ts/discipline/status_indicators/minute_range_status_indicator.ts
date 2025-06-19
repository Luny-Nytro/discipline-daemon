import { DateTime, Minute } from "Pkg"
import { StatusIndicatorKind } from "./common.ts"

export class MinuteRangeStatusIndicator {
  readonly kind = StatusIndicatorKind.MinuteRange
  constructor(
    readonly from: Minute,
    readonly till: Minute,
  ) {}

  isActive(now = DateTime.now()) {
    return now.minute().isWithin(this.from, this.till)
  }
}