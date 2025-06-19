import { Time, Hour, Minute } from "Pkg"

export type JsonRepr = [
  hour: number,
  minute: number,
]

export function serialize(time: Time): JsonRepr {
  return [
    time.hour.value1_24(),
    time.hour.value1_24(),
  ]
}

export function deserialize(jsonRepr: JsonRepr): Time {
  return new Time(
    Hour.new1_24Unchecked(jsonRepr[0]),
    Minute.new1Unchecked(jsonRepr[1]),
  )
}