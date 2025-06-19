import { Serde, DailyAllowance, Option } from "../Prelude.ts";

export function intoIR(me: DailyAllowance): Serde.IntermediateRepr {
  return [
    Serde.Duration.intoIR(me.allowance),
    Serde.Duration.intoIR(me.remainingAllowance),
    Serde.DateTime.intoIR(me.previousSynchronizationTime),
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<DailyAllowance> {
  return Serde.Tuble.fromIR3(
    ir,
    Serde.Duration.fromIR,
    Serde.Duration.fromIR,
    Serde.DateTime.fromIR,
    DailyAllowance.constructor
  )
}

export function serialize(me: DailyAllowance) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}