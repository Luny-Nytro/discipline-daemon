import { Serde, Duration, Option } from "../Prelude.ts";
import { Integer, IntermediateRepr } from "./mod.ts";

export function intoIR(me: Duration): IntermediateRepr {
  return Integer.intoIR(Duration.milliseconds(me))
}

export function fromIR(ir: IntermediateRepr): Option<Duration> {
  return Option.map(Integer.fromIR(ir), Duration.fromMilliseconds)
}

export function serialize(me: Duration) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}