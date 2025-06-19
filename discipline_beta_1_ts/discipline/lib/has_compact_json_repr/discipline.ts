import * as JSON from "./mod.ts"
import { Discipline, TimeSyncer } from "Pkg"

export type JsonRepr = [
  privatePassword: string,
  userAccessRegulators: JSON.UserAccessRegulator.JsonRepr,
  deviceAccessRegulator: JSON.DeviceAccessRegulator.JsonRepr,
  networkAccessRegulator: JSON.NetworkAccessRegulator.JsonRepr,  
]

export function serialize(discipline: Discipline): JsonRepr {
  return [
    discipline.privatePassword,
    JSON.UserAccessRegulator.serialize(discipline.userAccessRegulator),
    JSON.DeviceAccessRegulator.serialize(discipline.deviceAccessRegulrator),
    JSON.NetworkAccessRegulator.serialize(discipline.networkAccessRegulrator),
  ]
}

export function deserialize(path: string, jsonRepr: JsonRepr): Discipline {
  return Discipline.new(
    path,
    TimeSyncer.new(),
    jsonRepr[0],
    JSON.UserAccessRegulator.deserialize(jsonRepr[1]),
    JSON.DeviceAccessRegulator.deserialize(jsonRepr[2]),
    JSON.NetworkAccessRegulator.deserialize(jsonRepr[3]),
  )
}