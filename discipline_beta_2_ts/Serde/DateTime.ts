import { Serde, DateTime, Option } from "../Prelude.ts";

export function intoIR(me: DateTime): Serde.IntermediateRepr {
  return Serde.Integer.intoIR(DateTime.timestamp(me))
}

export function fromIR(ir: Serde.IntermediateRepr): Option<DateTime> {
  return Option.map(Serde.Integer.fromIR(ir), DateTime.fromTimestamp)
}

export function serialize(me: DateTime) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}