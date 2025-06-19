import { Serde, CountdownTimer, Option } from "../Prelude.ts";

export function intoIR(me: CountdownTimer): Serde.IntermediateRepr {
  return [
    Serde.Duration.intoIR(me.duration),
    Serde.Duration.intoIR(me.remainingDuration),
    Serde.DateTime.intoIR(me.previousSynchronizationTime),
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<CountdownTimer> {
  return Serde.Tuble.fromIR3(
    ir,
    Serde.Duration.fromIR,
    Serde.Duration.fromIR,
    Serde.DateTime.fromIR,
    CountdownTimer.constructor
  )
}

export function serialize(me: CountdownTimer) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}