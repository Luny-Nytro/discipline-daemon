import { StatusIndicatorKind, DurationStatusIndicator, Duration, DateTime } from "Pkg"

export type JsonRepr = [
  kind: StatusIndicatorKind.Duration,
  duration: number,
  previousSync: number | null,
]

export function serialize(statusIndicator: DurationStatusIndicator): JsonRepr {
  return [
    statusIndicator.kind,
    statusIndicator.duration.milliseconds(),
    statusIndicator.previousSync === null
      ? null
      : statusIndicator.previousSync.epochMilliseconds()
  ]
}

export function deserialize(jsonRepr: JsonRepr): DurationStatusIndicator {
  return new DurationStatusIndicator(
    Duration.fromMilliseconds(jsonRepr[1]),
    jsonRepr[2] !== null
      ? DateTime.fromEpochMilliseconds(jsonRepr[2])
      : null
  )
}