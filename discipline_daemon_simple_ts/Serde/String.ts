import { None, Option, Some, Serde } from "../Prelude.ts";

export function isString(value: unknown): value is string {
  return typeof value === "string"
}

export function intoIR(me: string): Serde.IntermediateRepr {
  return me
}

export function fromIR(ir: Serde.IntermediateRepr): Option<string> {
  return isString(ir) 
    ? Some(ir) 
    : None()
}


export function serialize(me: string) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}