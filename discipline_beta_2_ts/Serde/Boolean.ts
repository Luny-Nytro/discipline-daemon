import { Serde, Some, None, Option } from "../Prelude.ts";

export function isBoolean(ir: unknown): ir is boolean {
  return typeof ir === "boolean"
}

export function fromIR(ir: Serde.IntermediateRepr): Option<boolean> {
  return isBoolean(ir)
    ? Some(ir)
    : None()
}

export function intoIR(me: boolean): Serde.IntermediateRepr {
  return me
}

export function serialize(me: boolean) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}