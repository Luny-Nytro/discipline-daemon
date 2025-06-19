import * as JSON from "./mod.ts"
import { UserAccessRegulator } from "Pkg"

export interface JsonRepr {
  readonly username: string
  readonly password: string
  readonly blockIndicator: JSON.StatusIndicator.JsonRepr
}

export function serialize(regulator: UserAccessRegulator): JsonRepr {
  return {
    username: regulator.username,
    password: regulator.password,
    blockIndicator: JSON.StatusIndicator.serialize(regulator.blockIndicator)
  }
}

export function deserialize(jsonRepr: JsonRepr): UserAccessRegulator {
  return UserAccessRegulator.new(
    jsonRepr.username,
    jsonRepr.password,
    JSON.StatusIndicator.deserialize(jsonRepr.blockIndicator),
  )
}