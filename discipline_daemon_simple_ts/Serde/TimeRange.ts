import { Option, TimeRange, Serde } from "Prelude";

export function intoIR(me: TimeRange): Serde.IntermediateRepr {
  return [
    Serde.Time.intoIR(me.min), 
    Serde.Time.intoIR(me.max),
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<TimeRange> {
  return Serde.Tuble.fromIR2(
    ir,
    Serde.Time.fromIR,
    Serde.Time.fromIR,
    TimeRange.new
  )
}

export function serialize(me: TimeRange) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}