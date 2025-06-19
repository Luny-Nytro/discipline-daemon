import { DateTime, Duration, StatusIndicatorKind } from "Pkg"

export class DurationStatusIndicator {
  readonly kind = StatusIndicatorKind.Duration
  constructor(
    public duration: Duration,
    public previousSync: DateTime | null,
  ) {}

  static new(duration: Duration) {
    return new DurationStatusIndicator(duration, null)
  }

  isActive(now: DateTime = DateTime.now()) {
    const previousSync = this.previousSync ?? now
    this.duration = this.duration.minus(previousSync.until(now))
    this.previousSync = now
    return this.duration.milliseconds() > 0
  }
}
