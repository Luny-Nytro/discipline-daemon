import * as JSON from "./mod.ts"
import { Discipline, TimeSyncer } from "Pkg"

export interface JsonRepr {
  readonly privatePassword: string
  readonly userAccessRegulator: JSON.UserAccessRegulator.JsonRepr
  readonly deviceAccessRegulator: JSON.DeviceAccessRegulator.JsonRepr
  readonly networkAccessRegulator: JSON.NetworkAccessRegulator.JsonRepr
}

export function serialize(discipline: Discipline): JsonRepr {
  return {
    privatePassword: discipline.privatePassword,
    userAccessRegulator: JSON.UserAccessRegulator.serialize(discipline.userAccessRegulator),
    deviceAccessRegulator: JSON.DeviceAccessRegulator.serialize(discipline.deviceAccessRegulrator),
    networkAccessRegulator: JSON.NetworkAccessRegulator.serialize(discipline.networkAccessRegulrator),
  }
}

export function deserialize(path: string, jsonRepr: JsonRepr): Discipline {
  return Discipline.new(
    path,
    TimeSyncer.new(),
    jsonRepr.privatePassword,
    JSON.UserAccessRegulator.deserialize(jsonRepr.userAccessRegulator),
    JSON.DeviceAccessRegulator.deserialize(jsonRepr.deviceAccessRegulator),
    JSON.NetworkAccessRegulator.deserialize(jsonRepr.networkAccessRegulator),
  )
}