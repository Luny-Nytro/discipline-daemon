import { Option, Weekday, Serde } from "Prelude";

export function intoIR(me: Weekday): Serde.IntermediateRepr {
  return Serde.Integer.intoIR(Weekday.value0To6(me))
}

export function fromIR(ir: Serde.IntermediateRepr): Option<Weekday> {
  return Option.map(Serde.Integer.fromIR(ir), Weekday.from0To6)
}

export function serialize(me: Weekday) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}