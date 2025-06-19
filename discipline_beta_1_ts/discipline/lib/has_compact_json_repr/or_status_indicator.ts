import * as JSON  from "./mod.ts"
import { OrStatusIndicator, StatusIndicatorKind } from "Pkg"

export type JsonRepr = [
  kind: StatusIndicatorKind.Or,
  items: JSON.StatusIndicator.JsonRepr[],
]

export function serialize(statusIndicator: OrStatusIndicator): JsonRepr {
  return [
    statusIndicator.kind,
    statusIndicator.items.map(JSON.StatusIndicator.serialize)
  ]
}

export function deserialize(jsonRepr: JsonRepr): OrStatusIndicator {
  return new OrStatusIndicator(jsonRepr[1].map(JSON.StatusIndicator.deserialize))
}