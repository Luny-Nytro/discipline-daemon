import { Serde, Minute, Option } from "Prelude";

export function intoIR(me: Minute): Serde.IntermediateRepr {
  return Serde.Integer.intoIR(Minute.value(me))
}

export function fromIR(ir: Serde.IntermediateRepr): Option<Minute> {
  return Option.map(Serde.Integer.fromIR(ir), Minute.new)
}

export function serialize(me: Minute) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}