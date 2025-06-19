import { Serde, Hour, Option } from "Prelude";

export function intoIR(me: Hour): Serde.IntermediateRepr {
  return Serde.Integer.intoIR(Hour.value(me))
}

export function fromIR(ir: Serde.IntermediateRepr): Option<Hour> {
  return Option.map(Serde.Integer.fromIR(ir), Hour.new)
}

export function serialize(me: Hour) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}