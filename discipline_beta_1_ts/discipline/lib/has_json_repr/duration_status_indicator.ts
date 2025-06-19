import { StatusIndicatorKind, DurationStatusIndicator, Duration, DateTime } from "Pkg"

export interface JsonRepr {
  readonly kind: StatusIndicatorKind.Duration
  readonly duration: number
  readonly previousSync: number | null
}

export function serialize(statusIndicator: DurationStatusIndicator): JsonRepr {
  return {
    kind: statusIndicator.kind,
    duration: statusIndicator.duration.milliseconds(),
    previousSync: statusIndicator.previousSync === null
      ? null
      : statusIndicator.previousSync.epochMilliseconds()
  }
}

export function deserialize(jsonRepr: JsonRepr): DurationStatusIndicator {
  return new DurationStatusIndicator(
    Duration.fromMilliseconds(jsonRepr.duration),
    jsonRepr.previousSync === null
      ? null
      : DateTime.fromEpochMilliseconds(jsonRepr.previousSync)
  )
}