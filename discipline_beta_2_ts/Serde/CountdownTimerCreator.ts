import { Serde, CountdownTimerCreator, Option } from "../Prelude.ts";

export function intoIR(me: CountdownTimerCreator): Serde.IntermediateRepr {
  return [
    Serde.Duration.intoIR(me.duration)
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<CountdownTimerCreator> {
  return Serde.Tuble.fromIR1(
    ir, 
    Serde.Duration.fromIR, 
    CountdownTimerCreator.new,
  )
}

export function serialize(me: CountdownTimerCreator) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}