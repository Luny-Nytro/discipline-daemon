import { Serde, DataHider, Option } from "../Prelude.ts";

export function intoIR(me: DataHider): Serde.IntermediateRepr {
  return [
    Serde.String.intoIR(me.name),
    Serde.String.intoIR(me.data),
    Serde.CountdownTimer.intoIR(me.countdownTimer)
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<DataHider> {
  return Serde.Tuble.fromIR3(
    ir,
    Serde.String.fromIR,
    Serde.String.fromIR,
    Serde.CountdownTimer.fromIR,
    DataHider.constructor
  )
}

export function serialize(me: DataHider) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}