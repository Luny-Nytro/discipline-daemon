import { Serde, None, Option, Some } from "../Prelude.ts";
import { IntermediateRepr } from "./mod.ts";

export function isNumber(value: unknown): value is number {
  return typeof value === "number"
}

export function isInteger(value: unknown): value is number {
  return Number.isSafeInteger(value)
}
  
export function intoIR(me: number): IntermediateRepr {
  return me
}

export function fromIR(ir: IntermediateRepr): Option<number> {
  return isInteger(ir) 
    ? Some(ir) 
    : None()
}

export function serialize(me: number) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}