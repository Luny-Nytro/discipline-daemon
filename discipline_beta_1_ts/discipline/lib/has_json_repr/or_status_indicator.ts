import * as JSON  from "./mod.ts"
import { OrStatusIndicator, StatusIndicatorKind } from "Pkg"

export interface JsonRepr {
  readonly kind: StatusIndicatorKind.Or
  readonly items: JSON.StatusIndicator.JsonRepr[]
}

export function serialize(statusIndicator: OrStatusIndicator): JsonRepr {
  return {
    kind: statusIndicator.kind,
    items: statusIndicator.items.map(JSON.StatusIndicator.serialize)
  }
}

export function deserialize(jsonRepr: JsonRepr): OrStatusIndicator {
  return new OrStatusIndicator(jsonRepr.items.map(JSON.StatusIndicator.deserialize))
}