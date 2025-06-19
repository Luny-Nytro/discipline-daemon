import * as JSON  from "./mod.ts"
import { AndStatusIndicator, StatusIndicatorKind } from "Pkg"

export interface JsonRepr {
  readonly kind: StatusIndicatorKind.And
  readonly items: JSON.StatusIndicator.JsonRepr[]
}

export function serialize(statusIndicator: AndStatusIndicator): JsonRepr {
  return {
    kind: statusIndicator.kind,
    items: statusIndicator.items.map(JSON.StatusIndicator.serialize)
  }
}

export function deserialize(jsonRepr: JsonRepr): AndStatusIndicator {
  return new AndStatusIndicator(jsonRepr.items.map(JSON.StatusIndicator.deserialize))
}