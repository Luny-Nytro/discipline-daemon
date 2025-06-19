import * as JSON  from "./mod.ts"
import { AndStatusIndicator, StatusIndicatorKind } from "Pkg"

export type JsonRepr = [
  kind: StatusIndicatorKind.And,
  items: JSON.StatusIndicator.JsonRepr[]
]

export function serialize(statusIndicator: AndStatusIndicator): JsonRepr {
  return [
    statusIndicator.kind,
    statusIndicator.items.map(JSON.StatusIndicator.serialize),
  ]
}

export function deserialize(jsonRepr: JsonRepr): AndStatusIndicator {
  return new AndStatusIndicator(jsonRepr[1].map(JSON.StatusIndicator.deserialize))
}