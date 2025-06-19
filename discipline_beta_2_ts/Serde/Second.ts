import { Option, Second, Serde } from "Prelude";

export function intoIR(me: Second): Serde.IntermediateRepr {
  return Serde.Integer.intoIR(Second.value(me))
}

export function fromIR(ir: Serde.IntermediateRepr): Option<Second> {
  return Option.map(Serde.Integer.fromIR(ir), Second.new)
}

export function serialize(me: Second) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}