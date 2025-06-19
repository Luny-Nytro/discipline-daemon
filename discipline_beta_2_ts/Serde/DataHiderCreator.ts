import { Serde, DataHiderCreator, Option } from "../Prelude.ts";

export function intoIR(me: DataHiderCreator): Serde.IntermediateRepr {
  return [
    Serde.String.intoIR(me.name),
    Serde.String.intoIR(me.data),
    Serde.Duration.intoIR(me.timer),
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<DataHiderCreator> {
  return Serde.Tuble.fromIR3(
    ir,
    Serde.String.fromIR,
    Serde.String.fromIR,
    Serde.Duration.fromIR,
    DataHiderCreator.new,
  )
}

export function serialize(me: DataHiderCreator) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}