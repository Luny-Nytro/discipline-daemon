import * as JSON from "./mod.ts"
import { DeviceAccessRegulator } from "Pkg"

export type JsonRepr = [
  blockIndicator: JSON.StatusIndicator.JsonRepr
]

export function serialize(regulator: DeviceAccessRegulator): JsonRepr {
  return [
    JSON.StatusIndicator.serialize(regulator.blockIndicator)
  ]
}

export function deserialize(jsonRepr: JsonRepr): DeviceAccessRegulator {
  return DeviceAccessRegulator.new(
    JSON.StatusIndicator.deserialize(jsonRepr[0])
  )
}