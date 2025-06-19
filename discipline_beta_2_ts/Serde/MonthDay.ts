import { MonthDay, Option, Serde } from "Prelude";

export function intoIR(me: MonthDay): Serde.IntermediateRepr {
  return Serde.Integer.intoIR(MonthDay.value(me))
}

export function fromIR(ir: Serde.IntermediateRepr): Option<MonthDay> {
  return Option.map(Serde.Integer.fromIR(ir), MonthDay.new)
}

export function serialize(me: MonthDay) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}