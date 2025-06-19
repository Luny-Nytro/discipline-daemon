import { Time, Hour, Minute } from "Pkg"

export interface JsonRepr {
  readonly hour: number
  readonly minute: number
}

export function serialize(time: Time): JsonRepr {
  return {
    hour: time.hour.value1_24(),
    minute: time.hour.value1_24(),
  }
}

export function deserialize(jsonRepr: JsonRepr): Time {
  return new Time(
    Hour.new1_24Unchecked(jsonRepr.hour),
    Minute.new1Unchecked(jsonRepr.minute),
  )
}