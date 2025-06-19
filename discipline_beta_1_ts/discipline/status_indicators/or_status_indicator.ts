import { DateTime } from "Pkg"
import { StatusIndicator } from "./any_status_indicator.ts"
import { StatusIndicatorKind } from "./common.ts"

export class OrStatusIndicator {
  readonly kind = StatusIndicatorKind.Or
  constructor(
    public items: StatusIndicator[],
  ) {}

  isActive(now = DateTime.now()): boolean {
    return this.items.length > 0 && this.items.some(item => item.isActive(now))
  }
}