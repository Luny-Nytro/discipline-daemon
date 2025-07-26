import { Option, Time, Serde } from "../Prelude.ts";

export function intoIR(me: Time): Serde.IntermediateRepr {
  return Serde.Integer.intoIR(Time.asTimestamp(me))
}

export function fromIR(ir: Serde.IntermediateRepr): Option<Time> {
  return Option.map(Serde.Integer.fromIR(ir), Time.fromTimestamp)
}

export function serialize(me: Time) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}