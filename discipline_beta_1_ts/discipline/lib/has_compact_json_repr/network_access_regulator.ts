import { NetworkAccessRegulator } from "Pkg"
import * as JSON  from "./mod.ts"

export type JsonRepr = [
  blockIndicator: JSON.StatusIndicator.JsonRepr
]

export function serialize(regulator: NetworkAccessRegulator): JsonRepr {
  return [
    JSON.StatusIndicator.serialize(regulator.blockIndicator)
  ]
}

export function deserialize(jsonRepr: JsonRepr): NetworkAccessRegulator {
  return NetworkAccessRegulator.new(
    JSON.StatusIndicator.deserialize(jsonRepr[0])
  )
}