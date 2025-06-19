import * as JSON from "./mod.ts"
import { UserAccessRegulator } from "Pkg"

export type JsonRepr = [
  username: string,
  password: string,
  blockIndicator: JSON.StatusIndicator.JsonRepr,
]

export function serialize(regulator: UserAccessRegulator): JsonRepr {
  return [
    regulator.username,
    regulator.password,
    JSON.StatusIndicator.serialize(regulator.blockIndicator)
  ]
}

export function deserialize(jsonRepr: JsonRepr): UserAccessRegulator {
  return UserAccessRegulator.new(
    jsonRepr[0],
    jsonRepr[1],
    JSON.StatusIndicator.deserialize(jsonRepr[2]),
  )
}