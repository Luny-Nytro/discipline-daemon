import { DateTime } from "Pkg";
import { StatusIndicator } from "./any_status_indicator.ts";
import { StatusIndicatorKind } from "./common.ts"

export class AndStatusIndicator {
  readonly kind = StatusIndicatorKind.And
  constructor(
    public items: StatusIndicator[],
  ) {}

  isActive(now = DateTime.now()): boolean {
    return this.items.length > 0 && this.items.every(item => item.isActive(now))
  }
}